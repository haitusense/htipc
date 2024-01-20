mod core;
use core::*;
use clap::Parser;

fn main() {
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

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() -> anyhow::Result<()> {
    super::core::namedpipe::search("Init*");

    Ok(())
  }
  
 
}