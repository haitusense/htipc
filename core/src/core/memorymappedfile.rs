use anyhow::Result;
use anyhow::Context as _;

use std::fs::File;
use std::io::{Read, Write, BufWriter, BufReader};

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::um::memoryapi::{CreateFileMappingW, MapViewOfFile, FILE_MAP_ALL_ACCESS};
use winapi::um::winnt::{HANDLE, PAGE_READWRITE};
use winapi::um::errhandlingapi::GetLastError;

// use std::time::Duration;
// use tokio::time;
// use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;


/*

struct MemoryMappedFile {
  handle : *mut std::ffi::c_void,
  view : *mut std::ffi::c_void,
}

impl MemoryMappedFile {

  fn open(path:&str) -> anyhow::Result<Self> {
    let wide_name: Vec<u16> = std::ffi::OsStr::new(path)
      .encode_wide()
      .chain(std::iter::once(0))
      .collect();
    let handle = unsafe { kernel32::OpenFileMappingW(
      winapi::um::winnt::SECTION_MAP_WRITE,
      winapi::shared::minwindef::FALSE,
      wide_name.as_ptr()
    ) };
  
    if handle.is_null() { anyhow::bail!("open mmf err") }

    let view = unsafe { kernel32::MapViewOfFile(
      handle,
      winapi::um::winnt::SECTION_MAP_WRITE,
      0, 0, 0,
    ) };

    if view.is_null() {
      unsafe { kernel32::CloseHandle(handle) };
      anyhow::bail!("open mmf err");
    }
    
    Ok(Self { handle, view })
  }

  fn close(&self) {
    unsafe {
      kernel32::UnmapViewOfFile(self.view);
      kernel32::CloseHandle(self.handle);
    };
  }

  fn to_vec(&self) -> Vec<i32> {
    let memory_size = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, 1)[0] as usize
    };
    println!("memorysize : {memory_size}");

    let memory_slice = unsafe {
      &std::slice::from_raw_parts_mut(self.view as *mut i32, memory_size + 1)[1..=memory_size]
    };
    memory_slice.to_vec()
  }

  fn to_vector<T>(&self) -> Vec<T> where T: Clone {
    let memory_size = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, 1)[0] as usize
    };
    println!("memorysize : {memory_size}");

    let memory_slice = unsafe {
      std::slice::from_raw_parts_mut(self.view.offset(4) as *mut T, memory_size)
    };
    memory_slice.to_vec()
  }

  fn from_vec(&self, src: Vec<i32>) {
    let memory_slice = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, src.len() + 1)
    };
    memory_slice[0] = src.len() as i32;
    for i in 0..src.len() {
      memory_slice[i+1] = src[i];
    }
  }

  fn from_vector<T>(&self, src: Vec<T>) where T: Clone + Copy {
    let memory_size = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, 1)
    };
    memory_size[0] = src.len() as i32;
    let memory_slice = unsafe {
      std::slice::from_raw_parts_mut(self.view.offset(4) as *mut T, src.len())
    };

    for i in 0..src.len() {
      memory_slice[i] = src[i];
    }
  }

}

#[allow(non_snake_case)]
#[extendr]
fn readMemoryMappedFile(path:&str) -> Vec<i32> {
  let mmf = MemoryMappedFile::open(path).unwrap();
  let dst = mmf.to_vec();
  mmf.close();
  dst
}

#[allow(non_snake_case)]
#[extendr]
fn readMemoryMappedFileFloat(path:&str) -> Vec<f64> {
  let mmf = MemoryMappedFile::open(path).unwrap();
  let dst = mmf.to_vector::<f64>();
  mmf.close();
  dst
}

#[allow(non_snake_case)]
#[extendr]
fn writeMemoryMappedFile(path:&str, src:Vec<i32>) {
  let mmf = MemoryMappedFile::open(path).unwrap();
  mmf.from_vec(src);
  mmf.close();
}

#[allow(non_snake_case)]
#[extendr]
fn writeMemoryMappedFileFloat(path:&str, src:Vec<f64>) {
  let mmf = MemoryMappedFile::open(path).unwrap();
  mmf.from_vector::<f64>(src);
  mmf.close();
}

#[allow(dead_code)]
fn memory_mapped_file(path:&str) {
  let wide_name: Vec<u16> = std::ffi::OsStr::new(path)
    .encode_wide()
    .chain(std::iter::once(0))
    .collect();
  let handle = unsafe { kernel32::OpenFileMappingW(
    winapi::um::winnt::SECTION_MAP_WRITE,
    winapi::shared::minwindef::FALSE,
    wide_name.as_ptr()
  ) };
  if handle.is_null() { panic!("panic"); }

  let view = unsafe { kernel32::MapViewOfFile(
    handle,
    winapi::um::winnt::SECTION_MAP_WRITE,
    0, 0, 0,
  ) };

  if view.is_null() {
    unsafe { kernel32::CloseHandle(handle) };
    // anyhow::bail!("mmf err");
    panic!("panic");
  }

  let memory_size = unsafe {
    std::slice::from_raw_parts_mut(view as *mut i32, 1)[0] as usize
  };
  println!("memorysize : {memory_size}");

  let memory_slice = unsafe {
    &std::slice::from_raw_parts_mut(view as *mut i32, memory_size + 1)[1..memory_size]
  };
  for i in 0..memory_slice.len() {
    let val = memory_slice[i];
    println!("index {i} : {val}");
  }

  unsafe {
    kernel32::UnmapViewOfFile(view);
    kernel32::CloseHandle(handle);
  };
}

*/


#[cfg(test)]
mod tests {
  use std::path::PathBuf;
  use super::*;

  use std::ptr;

  #[test]
  fn it_works() -> Result<()> {

    let name = OsStr::new("shared_memory").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
    let handle = unsafe { CreateFileMappingW(null_mut(), null_mut(), PAGE_READWRITE, 0, 1024, name.as_ptr()) };
    
    if handle.is_null() {
      println!("Failed to create file mapping: {}", unsafe { GetLastError() });
    }
    
    let view = unsafe { MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, 1024) };
    
    if view.is_null() {
      println!("Failed to map view of file: {}", unsafe { GetLastError() });  
    }

    unsafe { 
      let u8_slice = std::slice::from_raw_parts_mut(view as *mut u8, 1024);
      // let first_element = u8_slice[2] as u8;
      println!("{:?}", u8_slice);
    }


    
    Ok(())
  }
  
  #[test]
  fn it_works2() -> Result<()> {

    // https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html

    let name = OsStr::new("shared_memory").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
    let handle = unsafe { CreateFileMappingW(null_mut(), null_mut(), PAGE_READWRITE, 0, 1024, name.as_ptr()) };
    
    if handle.is_null() {
      println!("Failed to create file mapping: {}", unsafe { GetLastError() });
    }
    
    let view = unsafe { MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, 1024) };
    
    if view.is_null() {
      println!("Failed to map view of file: {}", unsafe { GetLastError() });  
    }

    unsafe {

      let ptr = std::slice::from_raw_parts_mut(view as *mut u8, 1024).as_ptr();
      let slice  = std::ptr::slice_from_raw_parts(view as *const u8, 1024);
      // let first_element = u8_slice[2] as u8;
      println!("{:?}", *ptr.add(0) as u8);
      println!("{:?}", *ptr.add(1) as u8);

      // println!("{} {:?}", slice.len(), *slice[0]);

    }
    
    Ok(())
  }


}