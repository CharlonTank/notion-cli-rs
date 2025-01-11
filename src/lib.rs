pub mod config;
pub mod notion;

pub use config::Config;
pub use notion::{NotionClient, Task, TaskStatus, TaskPriority};

pub type Result<T> = anyhow::Result<T>; 