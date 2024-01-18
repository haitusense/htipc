#![allow(non_snake_case)]

use colored::Colorize;
use serde::{Serialize, Deserialize};
use pyo3::prelude::*;
use pyo3::{Python, types::PyDict};
use serde_pyobject::{to_pyobject, from_pyobject, pydict};

#[pyfunction]
fn namedpipe() {
  println!("namedpipe");
}

/// A Python module implemented in Rust.
#[allow(non_snake_case)]
#[pymodule]
fn htipcPyo(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(hello_world, m)?)?;
  Ok(())
}
