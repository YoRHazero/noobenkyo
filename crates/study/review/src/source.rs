/// Where a quiz's sense comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Pool,
    Queue,
}
