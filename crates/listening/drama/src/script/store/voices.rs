use crate::Role;
use crate::script::rules::voice_mapping::Voices;
use crate::world::store::cast::read::Member;

/// The voices a complete cast offers, by character name.
pub fn voices_of(members: &[Member]) -> Voices<'_> {
    let named = |wanted: fn(&Role) -> bool| -> Vec<&str> {
        members
            .iter()
            .filter(|member| wanted(&member.role))
            .map(|member| member.character.name.as_str())
            .collect()
    };
    Voices {
        narrator: named(|role| *role == Role::Narrator)
            .first()
            .copied()
            .unwrap_or_default(),
        mains: named(|role| matches!(role, Role::Main { .. })),
        minors: named(|role| *role == Role::Minor),
    }
}
