pub mod source;
pub mod single_threaded;
mod error;

pub trait Environment: Send + Sync {
    fn name(&self) -> &'static str;
}
