pub mod core;
use clap::Parser;
use core::*;

/******** for clu *********/

pub fn _main() { app(Cli::parse()); }

pub fn app(args :Cli) {
  match args.command {
    Commands::Hello { opt } => {
      // https://rust-cli.github.io/book/in-depth/signals.html
      // https://qiita.com/qnighy/items/b3b728adf5e4a3f1a841

      /* Atomic変数（排他制御メモリ）の宣言 */
      let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
      let r = running.clone();

      /* 別スレッドでctrl+cをキャッチ Atomic変数のフラグを立てる */
      ctrlc::set_handler(move ||{
        /* ここでstdoutすると表示被る */
        println!("handl");
        match r.load(std::sync::atomic::Ordering::SeqCst) {
          true => {
            println!("reset");
            r.store(false, std::sync::atomic::Ordering::SeqCst);
          },
          false => {
            println!("reset2");
            std::process::exit(1);
          }
        }
      }).expect("Error setting Ctrl-C handler");

      let mut a = crate::core::WaitCounter::new();
      while !a.check_time() & running.load(std::sync::atomic::Ordering::SeqCst) {
        /* Heavy Process */
      }
      a.stop();
      match running.load(std::sync::atomic::Ordering::SeqCst) {
        true => { println!("successed {}", opt); },
        false => { println!("failed {}", opt); },
      }
      println!("finished");
      // cargo runの場合は、cargoの方がctrl-cを別でキャッチしてkillしてしまう
      std::thread::sleep(std::time::Duration::from_secs(3));
      println!("finished2");
    }
    Commands::PIPE( args) => {
      let _ = core::namedpipe(args);
    }
    Commands::Search{ pipename: wm } => {
      core::namedpipe::search(&wm);
    }
  }
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


mod tests {

  #[test]
  fn it_works_cli() -> anyhow::Result<()> {
    use clap::{CommandFactory, FromArgMatches};
    let am = crate::core::Cli::command()
      .try_get_matches_from(vec!["cli","hello","--opt", "a"]).unwrap();
    let args = crate::core::Cli::from_arg_matches(&am).unwrap();

    crate::app(args);

    Ok(())
  }

  #[test]
  fn it_works_console_() -> anyhow::Result<()> {
    use crossterm::event::{read, Event, KeyEvent, KeyCode, EnableMouseCapture, DisableMouseCapture};
    use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    loop {
      // `read()` blocks until an `Event` is available
      match read()? {
          Event::Mouse(event) => println!("{:?}", event),
          Event::Key(event) => match event {
              KeyEvent {
                  code: KeyCode::Esc, ..
              } => break,
              _ => println!("{:?}", event),
          },
          Event::Resize(width, height) => println!("New size {}x{}", width, height),
          _ => {}
      }
  }
  execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    
    Ok(())
  }


  #[test]
  fn it_works_search() -> anyhow::Result<()> {

    crate::core::namedpipe::search("*Simple*");

    use clap::{CommandFactory, FromArgMatches};
    let am = crate::core::PipeArgs::command()
      .try_get_matches_from(vec!["cli","SimpleGui","draw", "--action", "true"]).unwrap();
    let args = crate::core::PipeArgs::from_arg_matches(&am).unwrap();

    crate::core::namedpipe(args);

    Ok(())
  }


}




