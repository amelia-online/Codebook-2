use std::io;
use std::io::stdout;
use std::io::Write;

pub mod interpreter;
use interpreter::session::SessionInfo;
use interpreter::interpet;

fn main() {
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
