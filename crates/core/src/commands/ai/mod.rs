//! The app's own use of models. Speaking an episode's lines runs as a task:
//! it starts at once, reports through the ai-task events, and can be
//! cancelled. Designing a voice takes seconds and is simply awaited; its WAV
//! goes to the frontend to be listened to before anything is stored. The
//! model page reads and replaces the settings and probes the machines.

pub mod probe;
pub mod settings;
pub mod speak_lines;
pub mod tasks;
pub mod voice;
