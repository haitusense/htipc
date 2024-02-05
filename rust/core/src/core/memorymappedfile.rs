/*
  // https://github.com/wez/wezterm/blob/3ec1cfba730b20e0426b6de106201bd7f32d4125/wezterm-client/src/discovery.rs#L24

  rust 1.75 ポインタバイトオフセットAPI
*/


/******** ptr ********/

pub fn open_mmf<T, F: FnMut(*mut u8) -> anyhow::Result<T>>(path:&str, mut f: F) -> anyhow::Result<T> {
  use std::os::windows::ffi::OsStrExt;
  let path = std::ffi::OsStr::new(path).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();

  /* create */
  // let handle = unsafe { CreateFileMappingW(INVALID_HANDLE_VALUE, null_mut(), PAGE_READWRITE, 0, 32*240+32, path.as_ptr()) };

  /* open */
  let handle = unsafe { winapi::um::memoryapi::OpenFileMappingW(winapi::um::memoryapi::FILE_MAP_ALL_ACCESS, 0, path.as_ptr()) };
  if handle.is_null() { anyhow::bail!("cannt open mmf"); }
  let buf = unsafe { winapi::um::memoryapi::MapViewOfFile(handle, winapi::um::memoryapi::FILE_MAP_ALL_ACCESS, 0, 0, 0) };
  if buf.is_null() { anyhow::bail!("cannt open mmf"); }

  let dst = f(buf as *mut u8);

  unsafe { winapi::um::memoryapi::UnmapViewOfFile(buf); }
  unsafe { winapi::um::handleapi::CloseHandle(handle); }
  dst
}

// addも内部的にはoffsetよんでるだけみたい
// repr(C) メモリレイアウト
// byte_addが要素の最大値に限定に限定されるので、
// アライメントされてない読み込みには repr(packed) か read_unaligned

fn ptr_read<T: Copy>(ptr: *const u8, pos: usize) -> T {
  let dst_ptr = unsafe { (ptr as *const T).byte_add(pos) };
  let dst = unsafe { dst_ptr.read_unaligned() };
  dst
}

fn ptr_read_array<T: Copy>(ptr: *const u8, pos: usize, dst: &mut Vec<T>) {
  let ptr = unsafe { (ptr as *const T).byte_add(pos) };
  // for i in 0..dst.len() {
  //   dst[i] = unsafe { ptr.add(i).read_unaligned() };
  // }
  unsafe { std::ptr::copy(ptr, dst.as_mut_ptr(), dst.len()); }
}

fn ptr_write<T: Copy>(ptr: *const u8, pos: usize, val: T) {
  let ptr = unsafe { (ptr as *mut T).byte_add(pos) };
  unsafe { std::ptr::write_unaligned(ptr, val) };
}

fn ptr_write_array<T: Copy>(ptr: *const u8, pos: usize, src: &Vec<T>) {
  let ptr = unsafe { (ptr as *mut T).byte_add(pos) };
  unsafe { std::ptr::copy(src.as_ptr(), ptr, src.len()); }
}

pub fn get_state(src: *const u8) -> (i32,i32,i32) {
  let ptr = src as *const i32;
  let size = unsafe{ *ptr.add(0) };
  let width = unsafe{ *ptr.add(1) };
  let height = unsafe{ *ptr.add(2) };
  (size,width,height)
}

/******** raw ********/

pub fn read<T: Copy>(path: &str, pos: usize) -> anyhow::Result<T> {
  open_mmf(path, |ptr| { 
    let dst_ptr = unsafe { (ptr as *const T).byte_add(pos) };
    let dst = unsafe { dst_ptr.read_unaligned() };
    Ok(dst)
  })
}

pub fn read_array<T: Copy>(path: &str, pos: usize, dst: &mut Vec<T>) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {
    ptr_read_array(ptr, pos, dst);
    Ok(())
  })
}

pub fn write<T: Copy>(path: &str, pos: usize, val: T) -> anyhow::Result<()> {
  open_mmf(path, |ptr| { 
    ptr_write(ptr, pos, val);
    Ok(())
  })
}

pub fn write_array<T: Copy>(path: &str, pos: usize, src: &Vec<T>) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {
    ptr_write_array(ptr, pos, src);
    Ok(())
  })
}

/******** for pixel ********/

// #[repr(C)]
// #[pyo3::prelude::pyclass]
// #[derive(Copy, Clone, Debug, Default)]
// pub struct Header {
//   #[pyo3(get)]
//   size : i32,
// }


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Header {
  pub size : i32,
  pub typecode :i32,
  pub width: i32,
  pub height: i32,
  pub depth: i32,

  dummy1: i32,
  dummy2: i32,
  dummy3: i32,
}


pub fn header(path: &str) -> anyhow::Result<Header> {
  open_mmf(path, |ptr| {
    let head : Header = ptr_read(ptr, 0);
    Ok(head)
  })
}

pub fn get_pixel<T: Copy>(path: &str, index: usize) -> anyhow::Result<T> {
  open_mmf(path, |ptr| {
    let head : Header = ptr_read(ptr, 0);
    let offset = std::mem::size_of::<Header>() + index * head.depth as usize;
    let dst : T = ptr_read(ptr, offset);
    Ok(dst)
  })
}

pub fn get_pixels<T: Copy>(path: &str, index: usize, dst: &mut Vec<T>) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {
    let head : Header = ptr_read(ptr, 0);
    let offset = std::mem::size_of::<Header>() + index * head.depth as usize;
    ptr_read_array(ptr, offset, dst);
    Ok(())
  })
}


pub fn set_pixel<T: Copy>(path: &str, index: usize, val: T) -> anyhow::Result<()> {
  open_mmf(path, |ptr| { 
    let head : Header = ptr_read(ptr, 0);
    let offset = std::mem::size_of::<Header>() + index * head.depth as usize;
    let dst_ptr = unsafe { (ptr as *mut T).byte_add(offset) };
    unsafe { std::ptr::write_unaligned(dst_ptr, val) };
    Ok(())
  })
}

pub fn set_pixels<T: Copy>(path: &str, index: usize, dst: &Vec<T>) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {
    let head : Header = ptr_read(ptr, 0);
    let offset = std::mem::size_of::<Header>() + index * head.depth as usize;
    ptr_write_array(ptr, offset, dst);
    Ok(())
  })
}




#[cfg(test)] 
mod tests {
  use super::*;

  #[test]
  fn it_works_mmf_read() -> anyhow::Result<()> {
    let dst = read::<Header>("SimpleGuiMmf", 0)?;
    println!("offset 0 {:?}", dst);
    let dst = read::<Header>("SimpleGuiMmf", 2)?;
    println!("offset 2 {:?}", dst);
    let dst = read::<Header>("SimpleGuiMmf", 4)?;
    println!("offset 4 {:?}", dst);

    let mut buf = vec![0i32; 20];
    read_array::<i32>("SimpleGuiMmf", 0, &mut buf)?;
    println!("before {:?}", buf);
    let mut hoge = vec![0i32; 320 * 240];
    for y in 0..240 {
      for x in 0..320 {
        hoge[x + y * 320] = x as i32;
      }
    }
    write_array::<i32>("SimpleGuiMmf", 32, &mut hoge)?;
    write::<i32>("SimpleGuiMmf", 32, 100)?;
    read_array::<i32>("SimpleGuiMmf", 0, &mut buf)?;
    println!("offset 0 {:?}", buf);
    read_array::<i32>("SimpleGuiMmf", 32, &mut buf)?;
    println!("offset 32 {:?}", buf);

    for i in 0..10  {
      let dst = get_pixel::<i32>("SimpleGuiMmf", i)?;
      println!("{} : {:?}", i, dst);
    }

    // write::<i32>("SimpleGuiMmf", 32 + 4 * 3, 10)?;

    // let mut buf = vec![0i32; 20];
    // read_array::<i32>("SimpleGuiMmf", 4, &mut buf)?;
    // println!("{:?}", buf);
    // read_array::<i32>("SimpleGuiMmf", 32, &mut buf)?;
    // println!("{:?}", buf);

    // let mut hoge = vec![3i32; 20];
    // write_array::<i32>("SimpleGuiMmf", 32, &mut hoge)?;
    // println!("{:?}", buf);

    // read_array::<i32>("SimpleGuiMmf", 32, &mut buf)?;
    // println!("{:?}", buf);
    Ok(())
  }

  
}


