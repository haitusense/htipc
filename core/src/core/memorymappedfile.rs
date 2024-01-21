// use anyhow::Result;
// use anyhow::Context as _;

/*
rust 1.75 ポインタバイトオフセットAPI
*/
pub fn fill(path:&str, val:u8) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {

    let state = get_state(ptr);
    println!("{:?}", state);
    let body = unsafe { ptr.add(32) as *mut u8 };

    for i in 0..state.0 as usize {
      unsafe { *body.add(i) = val; } 
    }
  })
}

pub fn lattice(path:&str) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {
    let state = get_state(ptr);
    let body = unsafe { ptr.add(32) as *mut u8 };
    println!("{:?}", state);
    for y in 0..state.2 as usize {
      for x in 0..state.1 as usize {
        if (x / 10) % 2 == (y / 10) % 2 {
          unsafe { *body.add(x + y * state.1 as usize) = 215; } 
        } else {
          unsafe { *body.add(x + y * state.1 as usize) = 40; } 
        }
      }
    }

  })
}

pub fn mmf_copy_to(path:&str, val:&[u8]) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {

    let state = get_state(ptr);
    println!("{:?}", state);
    let body = unsafe { ptr.add(32) as *mut u8 };

    for i in 0..state.0 as usize {
      unsafe { *body.add(i) = val[i]; } 
    }
  })
}


pub fn set_pixel(path:&str, index:usize, val:u8) -> anyhow::Result<()> {
  open_mmf(path, |ptr| {
    unsafe { *ptr.add(32 + index) = val; } 
  });
  Ok(())
}

pub fn get_pixel(path:&str, index:usize) -> anyhow::Result<u8> {
  let mut dst :u8 = 0;
  open_mmf(path, |ptr| {
     unsafe{ dst = *ptr.add(index); }
  });
  Ok(dst)
}

pub fn get_state(src: *const u8) -> (i32,i32,i32) {
  let ptr = src as *const i32;
  let size = unsafe{ *ptr.add(0) };
  let width = unsafe{ *ptr.add(1) };
  let height = unsafe{ *ptr.add(2) };
  (size,width,height)
}

fn open_mmf<F: FnMut(*mut u8)>(path:&str, mut f: F) -> anyhow::Result<()> {
  use std::os::windows::ffi::OsStrExt;
  let path = std::ffi::OsStr::new(path).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();

  /* create */
  // let handle = unsafe { CreateFileMappingW(INVALID_HANDLE_VALUE, null_mut(), PAGE_READWRITE, 0, 32*240+32, path.as_ptr()) };
  /* open */
  let handle = unsafe { winapi::um::memoryapi::OpenFileMappingW(winapi::um::memoryapi::FILE_MAP_ALL_ACCESS, 0, path.as_ptr()) };
  if handle.is_null() { anyhow::bail!("cannt open mmf"); }
  let buf = unsafe { winapi::um::memoryapi::MapViewOfFile(handle, winapi::um::memoryapi::FILE_MAP_ALL_ACCESS, 0, 0, 0) };
  if buf.is_null() { anyhow::bail!("cannt open mmf"); }

  f(buf as *mut u8);

  unsafe { winapi::um::memoryapi::UnmapViewOfFile(buf); }
  unsafe { winapi::um::handleapi::CloseHandle(handle); }
  Ok(())
}


#[cfg(test)] 
mod tests {
  use super::*;

  #[test]
  fn it_works_mmf() -> anyhow::Result<()> {

    // mmf_fill("SimpleGuiMmf",255).unwrap();
    // mmf_lattice("SimpleGuiMmf").unwrap();

    for i in 0..(320*240) {
      set_pixel("SimpleGuiMmf",i, 200u8).unwrap();
    }

    // let src = 40u8;
    // open_mmf(path, |ptr| {

    //   let ptr32 = ptr as *mut i32;
    //   let ptr32_wo_header = unsafe{ (ptr as *mut u8).add(4) as *mut i32 };
    //   let ptr8_mut = ptr as *mut u8;
    //   let dst :&[u8] = unsafe { std::slice::from_raw_parts(ptr as *mut u8, 32) };

    //   println!("{:?} {:?}", unsafe{ &*ptr32.add(1) }, unsafe{ &*ptr32.add(2) });
    //   println!("{:?} {:?}", unsafe{ &*ptr32_wo_header.add(0) }, unsafe{ &*ptr32_wo_header.add(1) });
  
    //   println!("{:?} {:?}", unsafe { &*ptr8_mut.add(33) }, unsafe{ &*ptr8_mut.add(34) });
    //   for i in 0..(320*240) {
    //     unsafe { *ptr8_mut.add(33 + i) = src; } 
    //   }
    //   println!("{:?} {:?}", unsafe { &*ptr8_mut.add(33) }, unsafe{ &*ptr8_mut.add(34) });


    Ok(())
  }


  // https://github.com/wez/wezterm/blob/3ec1cfba730b20e0426b6de106201bd7f32d4125/wezterm-client/src/discovery.rs#L24

  
}