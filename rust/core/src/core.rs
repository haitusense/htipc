pub mod namedpipe;
pub mod memorymappedfile;
use std::time::Instant;

use anyhow::Context as _;

use colored::Colorize;

// debug print

#[macro_export]
macro_rules! cprintln {
  ($a:expr, $b:expr) => { println!("{:>12} {:?}", $a.bold(), $b) };
  (red : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.red().bold(), $b) };
  (green : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.green().bold(), $b) };
  (blue : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.blue().bold(), $b) };
  (yellow : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.yellow().bold(), $b) };
  (magenta : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.magenta().bold(), $b) };
  (purple : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.purple().bold(), $b) };
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
  #[command(about = "Hello World :-)")]
  Hello {
    #[arg(short, long)]
    opt: String,
  },
  #[command(about = "Send Message using NamedPipe")]
  PIPE(PipeArgs),
  
  #[command(about = "Search NamedPipe by wildcard pattern")]
  Search{
    #[arg(num_args(1), help = "wildcard pattern")] 
    pipename: String,
  },
}

// commandを使用するため、clap::Args -> clap::Parser
#[derive(argsproc::PyRffi ,clap::Parser, serde::Serialize, serde::Deserialize, Debug)]
pub struct PipeArgs {
  #[arg(num_args(1))] 
  pub addr: String,

  #[arg(num_args(0..))] 
  pub commands: Option<Vec<String>>,

  #[arg(short, long, help = "default : inf.[ms]")] 
  pub connect_timeout: Option<u32>,

  #[arg(short, long, help = "default : inf.[ms]")] 
  pub read_timeout: Option<u32>,

  #[arg(short, long, help = "default : inf.[ms]")] 
  pub write_timeout: Option<u32>,

  #[arg(short, long, help = "use serialize json")] 
  pub json: Option<bool>, // Optionにして、オプション指定時true, falseの入力を強制（py側で破綻させないため）

  #[arg(short, long, help = "use serialize json")] 
  pub action: Option<bool>,
}

impl PipeArgs {

  pub fn to_value(&self) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::to_value(&self).context("err")?)
  }
  pub fn to_json(&self) -> anyhow::Result<String> { Ok(serde_json::to_string(self).context("err")?) }

  pub fn get_command_string(&self) -> String { 
    match (self.commands.clone(), self.json, self.action) {
      (Some(n), _, Some(true)) => {
        serde_json::json!({ "type": n[0], "payload": n[1..]}).to_string()
      },
      (Some(n), None, _) => format!("{}", n.join(" ")),
      (Some(n), Some(false), _) => format!("{}", n.join(" ")),
      (Some(n), Some(true), _) => serde_json::to_string(&n).unwrap(),
      (None, Some(true), _) => serde_json::json!({ }).to_string(),
      _ => "".to_string()
    }
  }
  pub fn get_addr_string(&self) -> String { format!(r##"\\.\pipe\{}"##, self.addr) }
}

pub fn namedpipe(args: PipeArgs) -> &'static str {
  cprintln!(blue: "connecting", args.addr);
  cprintln!(blue: "sending",  args.get_command_string());
  match namedpipe::send(args) {
    Ok(src) => {
      cprintln!(green: "received", src);
      "OK"
    },
    Err(e) => {
      cprintln!(red: "ERROR", e);
      "ERR"
    }
  }
}

pub fn header(path: &str) -> anyhow::Result<memorymappedfile::Header> {
  crate::memorymappedfile::header(path)
}

pub fn get_i32pixel(path: &str, index: usize) -> anyhow::Result<i32> {
  crate::memorymappedfile::get_pixel(path, index)
}

pub fn set_i32pixel(path: &str, index: usize, val: i32) -> anyhow::Result<()> {
  crate::memorymappedfile::set_pixel(path, index, val)
}

pub fn get_i32pixels(path: &str, index: usize, src: &mut Vec<i32>) -> anyhow::Result<()> {
  crate::memorymappedfile::get_pixels(path, index, src)
}

pub fn set_i32pixels(path: &str, index: usize, src: &mut Vec<i32>) -> anyhow::Result<()> {
  crate::memorymappedfile::set_pixels(path, index, src)
}


// let a = core::timeout::<u32>(10000u64, |now| {
//   std::thread::sleep(std::time::Duration::from_micros(100u64));
//   println!("{:?}", now.elapsed());
//   None
// });
pub fn timeout<T>(ms:u64, fp: fn(std::time::Instant) -> Option<anyhow::Result<T>> ) -> anyhow::Result<T> {
  let now = std::time::Instant::now();
  while std::time::Duration::from_micros(ms) > now.elapsed() {
    if let Some(n) = fp(now) { return n; } 
  }
  anyhow::bail!("err");
}

pub struct WaitCounter {
  ms : u64,
  time : Instant,
  cnt : u8,
  timeout : u64
}

use crossterm::{queue, cursor};
use std::io::stdout;

impl WaitCounter {
  pub fn new() -> Self { 
    Self { ms : 240, time : std::time::Instant::now(), cnt : 0, timeout : 10000 } 
  }
  pub fn init(&mut self) { self.time = std::time::Instant::now(); }
  pub fn check(&mut self) {
    if std::time::Duration::from_micros(self.ms) < self.time.elapsed() {
      self.init();
      self.cnt = (self.cnt + 1) & 0b11;
      let n = match self.cnt {
        0 => r"|",
        1 => r"/",
        2 => r"-",
        3 => r"\",
        _ => r"."
    };
      print!("{}", n);
      queue!(stdout(), cursor::MoveLeft(1)).unwrap();
    }
  }

  pub fn check_time(&mut self) -> bool {
    let prog = self.time.elapsed();
    if std::time::Duration::from_millis(self.timeout) > prog {
      let n = match (prog.as_millis() / self.ms as u128) % 4 {
        0 => r"|",
        1 => r"/",
        2 => r"-",
        3 => r"\",
        _ => r"."
      };
      let val = format!("{} {:.2} sec", n, prog.as_secs_f64());
      queue!(stdout(), crossterm::cursor::Hide).unwrap();
      print!("{val:<32}");
      queue!(stdout(), cursor::MoveLeft(32)).unwrap();  
      false
    } else {
      queue!(stdout(), crossterm::cursor::Show).unwrap();
      true
    }
  }

  pub fn stop(&mut self) {
    queue!(stdout(), crossterm::cursor::Show).unwrap();
    println!("");
  }

}


mod tests {

  #[test]
  fn it_works_mmf() -> anyhow::Result<()> {

    let mut hoge = vec![0i32; 320 * 240];
    for y in 0..240 {
      for x in 0..320 {
        hoge[x + y * 320] = (x % 10) as i32;
      }
    }
    super::memorymappedfile::write_array::<i32>("SimpleGuiMmf", 32, &mut hoge)?;

    let args = crate::PipeArgs::from_clap_vec(vec!["PipeArgs", "SimpleGui", "draw", "--json", "true"])?;
    let _ = super::namedpipe(args);
    
    Ok(())
  }


  #[test]
  fn it_works_ser() -> anyhow::Result<()> {
    #[derive(Clone, Debug, Default, serde::Serialize)]
    pub struct AA {
      pub size : i32,
      pub typecode :String,
      pub c :bool,
    }
    let a = AA{ size:1, typecode:"2".to_string(), c:false };
    // let b = serde_json::to_value(a).unwrap().as_object().unwrap().iter().collect::<Vec<_>>();

    let b = serde_json::to_value(a).unwrap();
    let c = b.as_object().unwrap().iter().filter_map(|n|{
      if let Some(m) = n.1.as_i64() { return Some((n.0, m.to_string())); }
      if let Some(m) = n.1.as_f64() { return Some((n.0, m.to_string())); }
      if let Some(m) = n.1.as_str() { return Some((n.0, m.to_string())); }
      if let Some(m) = n.1.as_bool() { return Some((n.0, m.to_string())); }
      None
    }).collect::<Vec<_>>();
  
    println!("{:?}", c);
    
    Ok(())
  }


}


