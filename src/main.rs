use std::io;
use std::io::Read;
use std::io::stdout;
use std::io::Write;

pub mod interpreter;
use interpreter::interpet::interpret_line;
use interpreter::session::SessionInfo;
use interpreter::interpet;

fn arguments() {
    let args: Vec<String> = std::env::args().collect();
    let mut index = 1;
    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {

            "f" => {
                index += 1;
                if let Ok(mut file) = std::fs::File::open(&args[index]) {
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer).expect("");
                    let mut session = SessionInfo::new();
                    interpret_line(buffer, &mut session);
                    std::process::exit(0);
                } else {
                    panic!("File not found!");
                }
            }


            _ => panic!("Unrecognized argument '{arg}'"),
        }
    }
}

fn main() {
    arguments();
    use interpet::*;
    println!();
    println!("Welcome to Codebook 2.0! The programmable CLI calculator.");
    println!("Type '{}{}help{}' for help, and '{}{}quit{}' to terminate.", 
                GREEN, BOLD, DEFAULT, RED, BOLD, DEFAULT);
    let mut session = SessionInfo::new();
    let _ = load_file("./src/config.cb", &mut session);
    loop {
        let line = read_line();

        if line.trim() == "quit" || line.is_empty() {
            let mut confirmation: String = String::new();
                set_color(YELLOW);
                print!("Are you sure you'd like to quit? [Y/n]: ");
                set_color(GREEN);
                stdout().flush().expect("Couldn't flush console.");
                io::stdin()
                    .read_line(&mut confirmation)
                    .expect("Could not read input.");

                

                if confirmation.trim() == "Y" {
                    set_color(DEFAULT);
                    break;
                }
        }

        interpret_line(line, &mut session);
        if session.stack.last().is_some() {
            let res = session.stack.pop().unwrap();
            out(&format!("{}", res));
            session.last = res;
        }

        session.stack.clear();
        session.idents.clear();
        println!();
    }
}
