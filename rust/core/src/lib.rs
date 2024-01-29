pub mod core;
use clap::Parser;
use core::*;

/******** for clu *********/

pub fn _main() {
  let cli = Cli::parse();
  match cli.command {
    Commands::Hello { opt } => {
      let a = timeout::<u32>(10000u64, |now| {
        std::thread::sleep(std::time::Duration::from_micros(100u64));
        println!("{:?}", now.elapsed());
        None
      });
      println!("{:?}", a);
      println!("hello {} !!", opt);

    }
    Commands::PIPE( args) => {
      let _ = core::namedpipe(args);
    }
    Commands::Search{ pipename: wm } => {
      core::namedpipe::search(&wm);
    }
  }

}

fn timeout<T>(ms:u64, fp: fn(std::time::Instant) -> Option<anyhow::Result<T>> ) -> anyhow::Result<T> {
  let now = std::time::Instant::now();
  while std::time::Duration::from_micros(ms) > now.elapsed() {
    if let Some(n) = fp(now) { return n; } 
  }
  anyhow::bail!("err");
}

/******** for py *********/

pub fn env() -> anyhow::Result<std::collections::HashMap<String, String>> {
  println!("{} {} by {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
  let mut dst = std::collections::HashMap::<String, String>::new();
  dst.insert("NAME".to_string(), env!("CARGO_PKG_NAME").to_string());
  dst.insert("VERSION".to_string(), env!("CARGO_PKG_VERSION").to_string());
  dst.insert("AUTHORS".to_string(), env!("CARGO_PKG_AUTHORS").to_string());
  Ok(dst)
}

#[argsproc::show_streams]
pub fn env2() {
  println!("{} {} by {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
}





