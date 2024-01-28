#![allow(non_snake_case)]

use pyo3::prelude::*;
// use pyo3::types::PyTuple;
use pyo3::Python;//, types::PyDict

#[pyfunction]
fn env() -> anyhow::Result<std::collections::HashMap<String, String>> { htipc::env() } 

#[pyfunction]
#[pyo3(signature = (*args, **kwargs))]
fn namedpipe(args: &pyo3::types::PyTuple, kwargs: Option<&pyo3::types::PyDict>) -> anyhow::Result<String> {
  let pa = htipc::core::PipeArgs::from_clap_py(args, kwargs)?;
  let dst = htipc::core::namedpipe(pa).to_string();
  Ok(dst)
}

#[pyfunction]
fn header<'p>(py: Python<'p>, path: &str) -> anyhow::Result<&'p PyAny> {
  let src = htipc::core::header(path)?;
  let obj: &'p PyAny = serde_pyobject::to_pyobject(py, &src).unwrap();
  Ok(obj)
}

#[pyfunction]
fn get_pixel(path: &str, index: usize) -> anyhow::Result<i32> {
  let dst = htipc::core::get_i32pixel(path, index)?;
  Ok(dst)
}

#[pyfunction]
fn set_pixel(path: &str, index: usize, val: i32) -> anyhow::Result<()> {
  htipc::core::set_i32pixel(path, index, val)
}

#[pyfunction]
fn get_pixels(path: &str, index: usize, size: usize) -> anyhow::Result<Vec<i32>> {
  let dst = &mut vec![0i32; size];
  htipc::core::get_i32pixels(path, index, dst)?;
  Ok(dst.clone())
}

#[pyfunction]
fn set_pixels(path: &str, index: usize, mut src: Vec<i32>) -> anyhow::Result<()> {
  htipc::core::set_i32pixels(path, index, &mut src)
}

// #[pyfunction]
// fn get_full_pixels(path: &str) -> anyhow::Result<Vec<i32>> {
//   let head = htipc::core::header(path)?;
//   let dst = &mut vec![0i32; head.size as usize];
//   htipc::core::get_i32pixels(path, 0, dst)?;
//   Ok(dst.clone())
// }

use numpy::{ToPyArray, PyArray};
use numpy::ndarray::prelude::*;

#[pyfunction]
fn get_pixels_fullarray<'p>(py: Python<'p>, path: &str) -> anyhow::Result<&'p PyArray<i32, Dim<[usize; 2]>>> {
  let head = htipc::core::header(path)?;
  let dst = &mut vec![0i32; head.size as usize];  
  htipc::core::get_i32pixels(path, 0, dst)?;

  let arr = PyArray::from_vec(py, dst.clone())
    .reshape([head.width as usize, head.height as usize])?;

  // let np = py.import("numpy")?
  //   .getattr("array")?
  //   .call1(dst)?
  //   .getattr("reshape")?
  //   .call1((head.width, head.height))?;
  Ok(arr)
}


/// A Python module implemented in Rust.
#[allow(non_snake_case)]
#[pymodule]
fn htipcPyo(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(env, m)?)?;
  m.add_function(wrap_pyfunction!(namedpipe, m)?)?;
  m.add_function(wrap_pyfunction!(header, m)?)?;
  m.add_function(wrap_pyfunction!(get_pixel, m)?)?;
  m.add_function(wrap_pyfunction!(set_pixel, m)?)?;
  m.add_function(wrap_pyfunction!(get_pixels, m)?)?;
  m.add_function(wrap_pyfunction!(set_pixels, m)?)?;
  Ok(())
}



#[cfg(test)]
mod tests {

  #[test]
  fn it_works_header() -> pyo3::PyResult<()> {
    pyo3::Python::with_gil(|py| {
      let code = indoc::formatdoc!{r#"
        def func():
          import htipcPyo
          import numpy as np

          print(htipcPyo.header("SimpleGuiMmf"))
          print(htipcPyo.header("SimpleGuiMmf")["size"])
          print(htipcPyo.get_pixel("SimpleGuiMmf", 0))
          print(htipcPyo.get_pixel("SimpleGuiMmf", 1))
          print(htipcPyo.get_pixel("SimpleGuiMmf", 2))
          
          htipcPyo.set_pixel("SimpleGuiMmf", 0, 20)
          htipcPyo.set_pixel("SimpleGuiMmf", 1, 19)
          htipcPyo.set_pixel("SimpleGuiMmf", 2, 18)

          print(htipcPyo.get_pixel("SimpleGuiMmf", 0))
          print(htipcPyo.get_pixel("SimpleGuiMmf", 1))
          print(htipcPyo.get_pixel("SimpleGuiMmf", 2))

          list = [30, 35]
          htipcPyo.set_pixels("SimpleGuiMmf", 0, list);
          
          print(htipcPyo.get_pixel("SimpleGuiMmf", 0))
          print(htipcPyo.get_pixel("SimpleGuiMmf", 1))
          print(htipcPyo.get_pixel("SimpleGuiMmf", 2))

          dst = htipcPyo.get_pixels("SimpleGuiMmf", 0, 10);
          print(dst)

          a = np.array(dst)
          print(a)

      "#};
      let func = pyo3::types::PyModule::from_code(py, &code, "", "")?
        .getattr("func")?;
      let _ = func.call0()?;
      Ok(())
    })
  }

  #[test]
  fn it_works_headers() -> pyo3::PyResult<()> {
    pyo3::Python::with_gil(|py| {
      let np = py.import("numpy")?;
      let dst = np
        .getattr("array")?
        .call1((vec![1, 2, 3, 4, 5, 6],))?
        .getattr("reshape")?
        .call1((3,2))?;

      println!("{:?}", dst);
      Ok(())
    })
  }


  #[test]
  fn it_works() -> pyo3::PyResult<()> {
    pyo3::Python::with_gil(|py| {
      let sys = py.import("sys")?;
      let version: String = sys.getattr("version")?.extract()?;
      let paths: &pyo3::PyAny = sys.getattr("path")?;
  
      println!("-------- version --------");
      println!("{version}");
      println!("-------- paths --------");
      println!("{paths:?}");
  
      let code = indoc::formatdoc!{r#"
        def func():
          import htipcPyo
          print(htipcPyo.env()["PKG_NAME"])
      "#};
      let func = pyo3::types::PyModule::from_code(py, &code, "", "")?
        .getattr("func")?;
      let _ = func.call0()?;
      Ok(())
    })
  }

}