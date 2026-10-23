//! bindings.ts is generated from the command list.

use noobenkyo_core::bindings::{builder, export};

#[test]
fn exports_bindings() {
    export(&builder()).unwrap();
}
