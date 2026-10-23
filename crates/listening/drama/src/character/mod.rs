//! Characters: a name, its reading, gender, age, an optional voice prompt,
//! and reference recordings to clone the voice from.

pub mod rules;
pub mod store;

mod values;

pub use values::{
    CharacterDetail, CharacterSummary, Gender, NewCharacter, NewReference, Reference, ReferenceText,
};
