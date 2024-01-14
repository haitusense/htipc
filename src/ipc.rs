use anyhow::{Result, bail};
// use anyhow::Context as _;
use colored::Colorize;

use tokio::net::windows::named_pipe::ClientOptions;
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

pub fn ipc_write(pipename:&str, command:&str) -> Result<String> {
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
        Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => bail!(e.to_string()),
        Err(e) => bail!(e.to_string())
      };
      println!("{:>12} {:?}", "received".blue().bold(), dst);
      Ok(dst.lines().collect::<String>())
    });
    result
}
