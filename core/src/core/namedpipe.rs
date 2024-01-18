use anyhow::{Result, bail};
use anyhow::Context as _;
use thiserror::Error; // 式の評価が起きないので、contextよりwith_contextの方が速い
use colored::Colorize;

use tokio::net::windows::named_pipe::ClientOptions;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use named_pipe::PipeClient;
use std::io::{Write, Read};
// use std::os::windows::ffi::OsStrExt;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum NamedPipeError {
  #[error("NoConnection '{0}'")]
	NoConnection(String),
  
  #[error("NoConnectionErr")]
	CommunicationError ,

  #[error("Timeout")]
  Timeout,
}

pub fn send(pipename:&str, command:&str) -> Result<String> {
  let pipeaddr = format!(r##"\\.\pipe\{pipename}"##);
  let command = format!("{command}\r\n");
  let mut buf = String::new();

  let mut pipe = PipeClient::connect(pipeaddr.to_owned())
    .with_context(|| NamedPipeError::NoConnection(pipeaddr))?;
  pipe.write_all(command.as_str().as_bytes())
    .context(NamedPipeError::CommunicationError)?;
  pipe.flush()
    .context(NamedPipeError::CommunicationError)?;
  pipe.read_to_string(&mut buf).context(NamedPipeError::CommunicationError)?;

  Ok(buf)
}

#[allow(dead_code)]
pub fn send2(pipename:&str, command:&str) -> Result<String> {
  let pipeaddr = format!(r"\\.\pipe\{}", pipename);
  let command = format!("{}\r\n", command);

  let result = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      let dst = match ClientOptions::new().open(pipeaddr.to_owned()) {
        Ok(mut client) => {
          println!("{:>12} {:?}", "connected".blue().bold(), pipeaddr);
          client.write_all(command.as_bytes()).await.unwrap();
          client.flush().await.unwrap();
          println!("{:>12} {:?}", "sent".blue().bold(), command);

          let mut buf = vec![0u8; 128];
          let mut string = String::new();
          loop {
            let n = client.read(&mut buf).await.unwrap();
            string.push_str(&String::from_utf8_lossy(&buf[..n]));
            if string.contains('\n') { break; }
          }
          string
        },
        // Err(e) if e.raw_os_error() == Some(windows_sys::Win32::Foundation::ERROR_PIPE_BUSY as i32) => bail!(e.to_string()),
        Err(e) => bail!(e.to_string())
      };
      println!("{:>12} {:?}", "received".blue().bold(), dst);
      Ok(dst.lines().collect::<String>())
    });
    result
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() -> Result<()> {
    send("namedpipe", "A B C").unwrap();
    Ok(())
  }
  
}