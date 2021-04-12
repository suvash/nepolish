use rustyline::error::ReadlineError;
use rustyline::Editor;

use nepolish::parser;

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
    let history_filename = "np_history.txt";
    if rl.load_history(&history_filename).is_err() {
        println!("प्रयोग ईतिहास भेटिएन ।");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                read_eval_print(&line)
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
    rl.save_history(&history_filename).unwrap();
}

fn read_eval_print(line: &str) {
    match parser::parse(line) {
        Ok(parsed) => match parser::eval(parsed) {
            Ok(value) => println!("{}", value),
            Err(e) => eprintln!("Error : {:?}", e),
        },
        Err(e) => eprintln!("Error : {}", e),
    }
}
