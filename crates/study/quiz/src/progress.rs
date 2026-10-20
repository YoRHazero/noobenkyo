/// Progress of a sense, as the dictionary keeps it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Progress {
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
}
