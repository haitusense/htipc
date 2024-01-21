#![allow(non_snake_case)]

use pyo3::prelude::*;
use pyo3::{Python, types::PyDict};

#[pyfunction]
#[pyo3(signature = (*args, **kwargs))]
fn namedpipe<'a>(args: &PyAny, kwargs: Option<&PyDict>) -> anyhow::Result<&'a str> {
  let args = htipc::core::PipeArgs::from_pydict(args, kwargs)?;
  let dst = htipc::core::namedpipe(args);
  Ok(dst)
}

/// A Python module implemented in Rust.
#[allow(non_snake_case)]
#[pymodule]
fn htipcPyo(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(namedpipe, m)?)?;
  Ok(())
}
