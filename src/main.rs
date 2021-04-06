use rustyline::error::ReadlineError;
use rustyline::Editor;

struct Config<'a> {
    name: &'a str,
    version: &'a str,
}

fn main() {
    let config = Config {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    };
    print_banner(config);

    repl();
}

fn print_banner(config: Config) {
    println!("{} version {}", config.name, config.version);
    println!("Use Ctrl-C, or Ctrl-D to exit prompt");
    println!();
}

fn repl() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("bhoot.txt").is_err() {
        println!("प्रयोग ईतिहास भेटिएन ।");
    }
    loop {
        let readline = rl.readline("अ > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("bhoot.txt").unwrap();
}
