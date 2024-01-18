pub mod core;
use anyhow::Result;

#[allow(dead_code)]
pub fn namedpipe(path: &str, value: &str) -> Result<String> {
  core::namedpipe::send(path, value)
}