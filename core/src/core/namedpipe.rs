use anyhow::{Result, bail};
use anyhow::Context as _;
use thiserror::Error; 
use super::*;
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

pub fn send(args: super::PipeArgs) -> Result<String> {
  let addr = args.get_addr_string();
  let com = args.get_command_string();

  let mut pipe = (|| -> anyhow::Result<PipeClient> {
    let now = std::time::Instant::now();
    match args.connect_timeout {
      Some(n) => while std::time::Duration::from_millis(n as u64) > now.elapsed() {
        if let Ok(n) = PipeClient::connect(addr.to_owned()) { return Ok(n); } 
      },
      None => loop {
        if let Ok(n) = PipeClient::connect(addr.to_owned()) { return Ok(n); } 
      }  
    };
    anyhow::bail!(NamedPipeError::Timeout);
  })()?;

  if let Some(n) = args.read_timeout { pipe.set_read_timeout(Some(std::time::Duration::from_millis(n as u64))); }
  if let Some(n) = args.write_timeout { pipe.set_write_timeout( Some(std::time::Duration::from_millis(n as u64))); }

  pipe.write_all(com.as_str().as_bytes())
    .context(NamedPipeError::CommunicationError)?;
  pipe.flush()
    .context(NamedPipeError::CommunicationError)?;
  
  let mut buf = String::new();
  pipe.read_to_string(&mut buf).context(NamedPipeError::CommunicationError)?;

  Ok(buf)
}

#[allow(dead_code)]
fn send2(pipename:&str, command:&str) -> Result<String> {
  use tokio::net::windows::named_pipe::ClientOptions;
  // use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;
  use tokio::io::{AsyncWriteExt, AsyncReadExt};
  use colored::Colorize;

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
        // Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => bail!(e.to_string()),
        Err(e) => bail!(e.to_string())
      };
      println!("{:>12} {:?}", "received".blue().bold(), dst);
      Ok(dst.lines().collect::<String>())
    });
    result
}
