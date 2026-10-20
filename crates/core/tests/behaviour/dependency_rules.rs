//! Checks the dependency rules between our crates, as declared in their manifests:
//! groups never depend on each other, sub-crates never depend on their group's
//! service crate, and core depends only on service crates.

use std::collections::HashMap;
use std::path::Path;

use cargo_metadata::{Metadata, MetadataCommand, Package};

/// Where a workspace crate sits: its group (first directory under crates/) and whether
/// it is the group's service crate.
struct Place {
    group: String,
    is_service: bool,
}

fn metadata() -> Metadata {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");
    MetadataCommand::new()
        .manifest_path(root)
        .no_deps()
        .exec()
        .unwrap()
}

fn places(metadata: &Metadata) -> HashMap<String, Place> {
    let crates_dir = metadata.workspace_root.join("crates");
    metadata
        .workspace_packages()
        .into_iter()
        .map(|package| {
            let dir = package.manifest_path.parent().unwrap();
            let parts: Vec<&str> = dir.strip_prefix(&crates_dir).unwrap().iter().collect();
            let place = Place {
                group: parts[0].to_owned(),
                is_service: parts.get(1) == Some(&"service"),
            };
            (package.name.to_string(), place)
        })
        .collect()
}

/// Names of the workspace crates a package depends on (any kind of dependency).
fn internal_dependencies<'a>(
    package: &'a Package,
    places: &HashMap<String, Place>,
) -> Vec<&'a str> {
    package
        .dependencies
        .iter()
        .map(|dependency| dependency.name.as_str())
        .filter(|name| places.contains_key(*name))
        .collect()
}

#[test]
fn groups_do_not_depend_on_each_other() {
    let metadata = metadata();
    let places = places(&metadata);
    for package in metadata.workspace_packages() {
        let own = &places[package.name.as_str()];
        if own.group == "core" {
            continue;
        }
        for dependency in internal_dependencies(package, &places) {
            let other = &places[dependency];
            assert_eq!(
                other.group, own.group,
                "{} (group {}) depends on {} (group {})",
                package.name, own.group, dependency, other.group
            );
            assert!(
                !other.is_service,
                "{} depends on its group's service crate {}",
                package.name, dependency
            );
        }
    }
}

#[test]
fn core_depends_only_on_service_crates() {
    let metadata = metadata();
    let places = places(&metadata);
    let core = metadata
        .workspace_packages()
        .into_iter()
        .find(|package| places[package.name.as_str()].group == "core")
        .unwrap();
    let dependencies = internal_dependencies(core, &places);
    assert!(!dependencies.is_empty());
    for dependency in dependencies {
        assert!(
            places[dependency].is_service,
            "core depends on {dependency}, which is not a service crate"
        );
    }
}

#[test]
fn every_group_has_one_service_crate() {
    let metadata = metadata();
    let places = places(&metadata);
    let mut services: HashMap<&str, usize> = HashMap::new();
    for place in places.values().filter(|place| place.group != "core") {
        *services.entry(place.group.as_str()).or_default() += usize::from(place.is_service);
    }
    for (group, count) in services {
        assert_eq!(count, 1, "group {group} has {count} service crates");
    }
}
