pub mod session {

    use std::collections::HashMap;
    use super::namespaces::{Namespace};
    pub struct SessionInfo {
        pub functions: HashMap<String, Vec<String>>,
        pub variables: HashMap<String, f32>,
        pub constants: HashMap<String, f32>,
        pub idents: Vec<String>,
        pub stack: Vec<f32>,
        pub script: Vec<String>,
        pub bool_val: Option<f32>,
        pub last: f32,
        pub namespaces: Vec<Namespace>,
        pub branches: HashMap<usize, usize>,
    }
    
    impl SessionInfo {
        pub fn new() -> Self {
            SessionInfo {
                functions: HashMap::new(),
                variables: HashMap::new(),
                constants: HashMap::new(),
                idents: Vec::new(),
                stack: Vec::new(),
                script: Vec::new(),
                bool_val: None,
                last: 0.0,
                namespaces: Vec::new(),
                branches: HashMap::new(),
            }
        }
    
        pub fn reset(&mut self) {
            self.functions.clear();
            self.variables.clear();
        }

        pub fn contains_function(&self, name: String) -> bool {
            if self.functions.contains_key(&name) {
                true
            } else {

            for namespace in &self.namespaces {
                if namespace.functions.contains_key(&name) {
                    return true;
                }
            }

                false
            }
        }

        pub fn get_function(&self, name: String) -> Vec<String> {
            if self.functions.contains_key(&name) {
                self.functions.get(&name).unwrap().to_vec()
            } else {
                for namespace in &self.namespaces {
                    if namespace.functions.contains_key(&name) {
                        return namespace.functions.get(&name).unwrap().to_vec();
                    }
                }
                let empty = Vec::new();
                empty
            }
        }
    }
    
}

pub mod interpet {
use super::namespaces::Namespace;
use super::session::SessionInfo;
use std::fs::File;
use std::io;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::process::exit;


pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[96m";
pub const BOLD: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const NO_UNDERLINE: &str = "\x1b[24m";
pub const DEFAULT: &str = "\x1b[0m";

pub fn set_color(colorcode: &str) {
    print!("{}", colorcode);
    stdout().flush().expect("Could not change color.");
}

pub fn read_line() -> String {
    set_color(BLUE);
    print!("[In] << ");
    stdout().flush().expect("Could not flush console.");

    let mut input: String = String::from("");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input
}

pub fn out(msg: &str) {
    set_color(GREEN);
    println!("[Out] >> {}", msg);
}

pub fn get_string(line: &String, start: usize) -> (String, usize) {
    let mut index = start;
    let chars: Vec<char> = line.chars().collect();
    let mut result = String::new();
    while index < line.len() {

        let ch = chars[index];

        match ch {

            '\\' => {
                index += 1;
                match chars[index] {

                    'n' => result.push('\n'),

                    't' => result.push('\t'),

                    '\\' => result.push('\\'),

                    '\"' => result.push('\"'),

                    _ => {
                        error("Unknown escape sequence encountered.");
                        exit(0);
                    }
                }


            }

            '\"' => {
                return (result, index);
            }


            _ => result.push(ch),
        }
        index += 1;
    }   
    (result, index)    
}

pub fn tokenize(line: &String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut index = 0;
    let line_chars: Vec<char> = line.chars().collect();
    let mut current_token = String::new();
    while index < line.len() {
        let ch: char = line_chars[index];

        match ch {

            ' ' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.to_owned());
                    current_token.clear();
                }
            }

            '\n' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.to_owned());
                    current_token.clear();
                }
            }

            '\"' => {
                let (str, new_index) = get_string(&line, index+1);
                index = new_index+1;
                tokens.push(str);
            }

            _ => current_token.push(ch),

        }
        index += 1;
    }

    if !current_token.is_empty() {
        tokens.push(current_token.to_owned());
    }

    tokens
}

pub fn error(msg: &str) {
    set_color(RED);
    println!("[Error]: {}", msg);
}

pub fn warn(w: &str) {
    set_color(YELLOW);
    println!("[Warn]: {}", w);
}

pub fn interpret_vec(vector: &Vec<String>, session: &mut SessionInfo) {
    let line = vec_to_line(vector);
    interpret_line(line, session);
}

pub fn vec_to_line(vector: &Vec<String>) -> String {
    let mut res = String::from("");
    let mut i = 0;
    for item in vector {
        if i < vector.len() && i > 0 {
            res.push_str(" ");
        }

        res.push_str(item);
        i += 1;
    }

    res
}

pub fn copy_vec(vector: &Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for item in vector.into_iter() {
        res.push(item.to_string());
    }

    res
}

pub fn load_file(filepath: &str, session: &mut SessionInfo) -> Result<(), ()> {
    let mut line = String::from("");
    if let Ok(mut file) = File::open(filepath) {
        file.read_to_string(&mut line)
            .expect("Could not read file.");
        interpret_line(line, session);
        Ok(())
    } else {
        error(&format!("Could not read file '{}'.", filepath));
        Err(())
    }
}

pub fn tab(num_tabs: u16) {
    for _ in 0..num_tabs {
        print!("\t");
    }
    stdout().flush().expect("failed to flush tabs.");
}

pub fn print_help_item(keyword: &str, desc: &str) {
    tab(2);
    println!("{}{}{}{}{}{} => {}",
                YELLOW, BOLD, UNDERLINE, keyword,
                DEFAULT, GREEN, desc);
}

pub fn print_help() {
    set_color(GREEN);
    out("\n");
    tab(1);
    println!("{}OPERATORS:{}{}", BOLD, DEFAULT, GREEN);
    print_help_item("+", "(Addition) - adds the last two items on the stack.");
    print_help_item("-", "(Subtraction) - subtracts the last two items on the stack.");
    print_help_item("*", "(Multiplication) - multiplies the last two items on the stack.");
    print_help_item("/", "(Division) - divides the last two items on the stack.");
    print_help_item("**", "(Pow) - performs a to the power of b\n\t\twhere a and b are the last two items on the stack.");
    print_help_item("%", "(Modulus) - Performs division and returns the remainder.");
    print_help_item("++", "(Increment) - Increments the last item on the stack by 1.");
    print_help_item("--", "(Decrement) - Decrements the last item on the stack by 1.");
    print_help_item("n!", "(Factorial) - Returns the factorial of the last item on the stack.");
    print_help_item("==", "(Equality) - Determines if the last two items on the stack are equal.");
    print_help_item("!=", "(Not Equality) - Determines if the last two items on the stack are not equal.");
    print_help_item(">", "(Greater Than) - Determines if the last item is less than the second to last.");
    print_help_item("<", "(Less Than) - Determines if the last item is greater than the second to last");
    print_help_item("<=", "(Less Than Equal To) - Determines if the last item is greater than or equal to the second to last.");
    print_help_item(">=", "(Greater Than Equal To) - Determines if the last item is less than or equal to the second to last.");
    print_help_item("<<", "(Shift Left) - Shifts the bits of a number to the left a specified amount of times.");
    print_help_item(">>", "(Shift Right) - Shifts the bits of a number to the right a specified amount of times.");
    print_help_item("&", "(AND) - Performs the logical AND operation on the last two items on the stack.");
    print_help_item("|", "(OR) - Performs the logical OR operation on the last two items on the stack.");
    print_help_item("^", "(XOR) - Performs the logical XOR operation on the last two items on the stack.");
    println!("\t{}FUNCTIONS:{}{}", BOLD, DEFAULT, GREEN);
    print_help_item("drop", "Deletes the last item on the stack.");
    print_help_item("dup", "Duplicates the last item on the stack.");
    print_help_item("STACK_SIZE", "Pushes the size of the stack onto the stack.");
    println!("\t{}UTILITIES:{}{}", BOLD, DEFAULT, GREEN);
    print_help_item("functions", "Prints a list of all user-defined functions.");
    print_help_item("variables", "Prints a list of all user-defined variables.");
    print_help_item("constants", "Prints a list of all user-defined constants.");
    print_help_item("identifiers", "Prints a list of all user-defined identifiers.");
    print_help_item("script", "Prints a list of tokens in {code}.");
    print_help_item("pop_ident", "Pops the last identifier off the identifier stack.");
    print_help_item("push_ident", "Pushes the last number onto the identifier stack.");
    print_help_item("out", "Prints a desired message from [Out].");
    print_help_item("oout", "Prints a suppressable desired message from [Out].");
    print_help_item("warn", "Prints a desired message from [Warn].");
    print_help_item("owarn", "Prints a suppressable desired message from [Warn].");
    print_help_item("err", "Prints a desired message from [Error].");
    print_help_item("oerr", "Prints a suppressable desired message from [Error].");
    print_help_item("flush", "Prints a desired message w/o any source.");
    print_help_item("def", "Defines a variable using the last identifier and number.");
    print_help_item("pub fn", "Defines a function using the last {code} and identifier.");
    print_help_item("{", "Begins adding tokens to code.");
    print_help_item("}", "Ends adding tokens to innermost scope.");
    print_help_item("load_file", "Loads a file into the calculator.");
    print_help_item("reset", "If arguments are provided in {code}, it resets those specific variables/functions,
    \t\totherwise, it resets all functions and variables.");
    print_help_item("concatf", "Concatenates the top identifier to {code}.");
    print_help_item("clear", "Clears the screen.");
    print_help_item("ansi", "Executes ANSI commands within {code}.");
    print_help_item("help", "Prints this page.");
    print_help_item("credits", "Prints credits.");
    print_help_item("about", "Prints more about this project.");
    print_help_item("clear_code", "Clears {code}, this is automatically done after execution ends.");
    print_help_item("quit", "Prompts the user to quit.");
    println!("\t{}PROGRAMMING:{}{}", BOLD, DEFAULT, GREEN);
    print_help_item("if", "If statement; executes {code} if last item on the stack is 1.");
    print_help_item("elif", "Else if statement; executes {code} if last item on the stack is 1 and if statement is false.");
    print_help_item("else", "Else statement; executes {code} if last if/elif is false.");
    print_help_item("times", "Executes {code} specified amount of times.");
    print_help_item("break", "Breaks execution.");
    println!("\t\t{}MORE TO COME!{}{}", BOLD, DEFAULT, GREEN);
    println!("\t{}FLAGS:{}{}", BOLD, DEFAULT, GREEN);
    print_help_item("-s", "(Suppress output) - Suppresses any optional function output.");
    print_help_item("-sall", "(Suppress all outputs) - Suppresses all optional function outputs.");    
}

pub fn interpret_line(line: String, info: &mut SessionInfo) {
    let tokens = tokenize(&line);    

    let mut recording = false;
    let mut suppress_out = false;
    let mut supress_all = false;
    let mut open_curly = 0;
    for t in tokens {
        let token = t.as_str();
        if info.variables.contains_key(token) && !recording {
            info.stack.push(*info.variables.get(token).unwrap());
        } else if info.constants.contains_key(token) && !recording { 
            info.stack.push(*info.constants.get(token).unwrap());
        } else if token == "{" {
            if open_curly > 0 {
                open_curly += 1;
                info.script.push(token.to_owned());
                continue;
            }

            recording = true;
            open_curly += 1;
            continue;
        } else if token == "}" {
            if open_curly > 1 {
                open_curly -= 1;
                info.script.push(token.to_owned());
                continue;
            } else if open_curly == 0 {
                error("Unexpected '}' encountered!");
                break;
            }

            open_curly -= 1;
            recording = false;
        }

        if recording {
            info.script.push(token.to_owned());
            continue;
        }

        match token {

            "else" => {
                if let Some(res) = info.bool_val {
                    if res == 0.0 {
                        let exec = copy_vec(&info.script);
                        info.script.clear();
                        interpret_vec(&exec, info);
                    } else {
                        info.bool_val = None;
                    }
                } else {
                    info.script.clear();
                    continue;
                }
                info.script.clear();
            }

            "elif" => {
                if info.stack.len() == 0 {
                    error("Insufficient arguments!");
                    break;
                }

                let cond = info.stack.pop().unwrap();
                if let Some(res) = info.bool_val {
                    if res == 0.0 {

                        if cond == 1.0 {
                            let exec = copy_vec(&info.script);
                            info.script.clear();
                            info.bool_val = Some(1.0);
                            interpret_vec(&exec, info);
                        } else {
                            info.bool_val = Some(0.0);
                        }
                        
                    } else {
                        info.bool_val = None;
                    }
                } else {
                    error("elif block missing if/elif block!");
                    info.script.clear();
                    break;
                }
                info.script.clear();
            }

            "script" => {
                if info.script.is_empty() {
                    out("None");
                    break;
                } 

                out("\n");
                for i in 0..info.script.len() {
                    println!("\t{}{}{}{}: {}", BOLD, i, DEFAULT, GREEN, info.script[i]);
                }
            }

            "clear_code" => {
                info.script.clear();
            }

            "concatf" => {
                if info.idents.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let ident = info.idents.pop().unwrap();
                info.script.push(ident);
            }


            "push_ident" => {
                if info.stack.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let ident = info.stack.pop().unwrap();
                info.idents.push(ident.to_string());

            }

            "last" => {
                info.stack.push(info.last);
            }

            "owarn" => {
                if info.script.is_empty() {
                    info.script.push(" ".to_string());
                }

                if !suppress_out && !supress_all {
                    let msg = vec_to_line(&info.script);
                    warn(&msg);
                }
                suppress_out = false;
                info.script.clear();
            },

            "oerr" => {
                if info.script.is_empty() {
                    info.script.push(" ".to_string());
                }

                if !suppress_out && !supress_all {
                    let msg = vec_to_line(&info.script);
                    error(&msg);
                }
                suppress_out = false;
                info.script.clear();
            },

            "oout" => {
                if info.script.is_empty() {
                    info.script.push(" ".to_string());
                }

                if !suppress_out && !supress_all {
                    let msg = vec_to_line(&info.script);
                    out(&msg);
                }
                suppress_out = false;
                info.script.clear();
            }

            "pop_ident" => {
                if info.idents.is_empty() {
                    error("Nothing to pop.");
                    break;
                }
                info.idents.pop();
            }

            "identifiers" => {
                if info.idents.is_empty() {
                    out("None");
                    break;
                }

                out("\n");
                for i in 0..info.idents.len() {
                    println!("\t{}{}{}",
                    UNDERLINE, info.idents[i], NO_UNDERLINE);
                }
                println!("");
            }


            "+" => {
                let rhs = info.stack.pop();
                let lhs = info.stack.pop();

                if rhs.is_none() || lhs.is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let right = rhs.unwrap();
                let left = lhs.unwrap();

                info.stack.push(left + right);
            }

            "-" => {
                let rhs = info.stack.pop();
                let lhs = info.stack.pop();

                if rhs.is_none() || lhs.is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let right = rhs.unwrap();
                let left = lhs.unwrap();

                info.stack.push(left - right);
            }

            "*" => {
                let rhs = info.stack.pop();
                let lhs = info.stack.pop();

                if rhs.is_none() || lhs.is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let right = rhs.unwrap();
                let left = lhs.unwrap();

                info.stack.push(left * right);
            }

            "/" => {
                let rhs = info.stack.pop();
                let lhs = info.stack.pop();

                if rhs.is_none() || lhs.is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let right = rhs.unwrap();
                let left = lhs.unwrap();

                info.stack.push(left / right);
            }

            "%" => {
                let rhs = info.stack.pop();
                let lhs = info.stack.pop();

                if rhs.is_none() || lhs.is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let right = rhs.unwrap();
                let left = lhs.unwrap();

                info.stack.push(left % right);
            }

            "**" => {
                let rhs = info.stack.pop();
                let lhs = info.stack.pop();

                if rhs.is_none() || lhs.is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let right = rhs.unwrap();
                let left = lhs.unwrap();

                info.stack.push(left.powf(right));
            }

            "++" => {
                if info.stack.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let num = info.stack.pop().unwrap();
                info.stack.push(num + 1.0);
            }

            "--" => {
                if info.stack.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let num = info.stack.pop().unwrap();
                info.stack.push(num - 1.0);
            }

            ">>" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap() as i32;
                let lhs = info.stack.pop().unwrap() as i32;

                info.stack.push((lhs >> rhs) as f32);
            }

            "<" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap();
                let lhs = info.stack.pop().unwrap();

                if lhs < rhs {
                    info.stack.push(1.0);
                } else {
                    info.stack.push(0.0);
                }
            }

            ">" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap();
                let lhs = info.stack.pop().unwrap();

                if lhs > rhs {
                    info.stack.push(1.0);
                } else {
                    info.stack.push(0.0);
                }
            }

            "<=" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap();
                let lhs = info.stack.pop().unwrap();

                if lhs <= rhs {
                    info.stack.push(1.0);
                } else {
                    info.stack.push(0.0);
                }
            }

            ">=" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap();
                let lhs = info.stack.pop().unwrap();

                if lhs >= rhs {
                    info.stack.push(1.0);
                } else {
                    info.stack.push(0.0);
                }
            }

            "<<" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap() as i32;
                let lhs = info.stack.pop().unwrap() as i32;

                info.stack.push((lhs << rhs) as f32);
            }

            "|" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap() as i32;
                let lhs = info.stack.pop().unwrap() as i32;

                info.stack.push((lhs | rhs) as f32);
            }

            "&" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap() as i32;
                let lhs = info.stack.pop().unwrap() as i32;

                info.stack.push((lhs & rhs) as f32);
            }

            "^" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap() as i32;
                let lhs = info.stack.pop().unwrap() as i32;

                info.stack.push((lhs ^ rhs) as f32);
            }

            "break" => {
                break;
            }

            "err" => {
                if info.script.is_empty() {
                    error("");
                    continue;
                }

                let msg = vec_to_line(&info.script);
                error(&msg);
                info.script.clear();
            }

            "constants" => {
                if info.constants.len() == 0 {
                    out("None");
                    continue;
                }

                out("\n");
                for variable in info.constants.keys() {
                    println!("\t{}{}{}{} = {}", 
                    BOLD,
                    variable, 
                    DEFAULT,
                    GREEN,
                    info.constants.get(variable).unwrap());
                }
            }

            "warn" => {
                if info.script.is_empty() {
                    error("");
                    continue;
                }

                let msg = vec_to_line(&info.script);
                warn(&msg);
                info.script.clear();
            }

            "times" => {
                if info.stack.is_empty() && info.script.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }
                let times = info.stack.pop().unwrap();
                let copy = copy_vec(&info.script);
                info.script.clear();
                for _ in 0..times as i32 {
                    interpret_vec(&copy, info);
                }
            }

            "puts" => {
                if info.idents.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let item = info.idents.pop().unwrap();
                print!("{item}");
                std::io::stdout().flush().expect("");
            }

            "STACK_SIZE" => {
                let size = info.stack.len();
                info.stack.push(size as f32);
            }

            "load_file" => {
                if info.idents.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let filepath = info.idents.pop().unwrap();
                let res = load_file(filepath.as_str(), info).is_ok();

                if !suppress_out && !supress_all && res {
                    out(&format!("Successfully loaded file '{}'.", filepath));
                }
            }

            "if" => {
                if info.stack.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let res = info.stack.pop().unwrap();
                if res == 1.0 {
                    let copy = copy_vec(&info.script);
                    info.script.clear();
                    interpret_vec(&copy, info);
                    info.bool_val = Some(1.0);
                } else {
                    info.bool_val = Some(0.0);
                }
                info.script.clear();
            }

            "n!" => {
                if info.stack.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let num = info.stack.pop().unwrap();
                let mut res = 1;

                for i in 2..num as i32 + 1 {
                    res *= i;
                }
                info.stack.push(res as f32);
            }

            "==" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap();
                let lhs = info.stack.pop().unwrap();
                if rhs == lhs {
                    info.stack.push(1.0);
                } else {
                    info.stack.push(0.0);
                }
            },

            "!=" => {
                if info.stack.len() < 2 {
                    error("Insufficient arguments!");
                    break;
                }

                let rhs = info.stack.pop().unwrap();
                let lhs = info.stack.pop().unwrap();
                if rhs != lhs {
                    info.stack.push(1.0);
                } else {
                    info.stack.push(0.0);
                }
            },


            "credits" => out("Codebook created by Amelia Johnson."),

            "about" => {
                out("Not yet implemented.");
            },

            "def" => {
                if info.idents.last().is_none() || info.stack.last().is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let key = info.idents.pop().unwrap();
                let value = info.stack.pop().unwrap();

                if !suppress_out && !supress_all {
                    out(&format!("{} = {}", key, value));
                }

                suppress_out = false;
                info.variables.insert(key, value);
            }

            "const" => {
                if info.idents.last().is_none() || info.stack.last().is_none() {
                    error("Insufficient arguments!");
                    break;
                }

                let key = info.idents.pop().unwrap();
                let value = info.stack.pop().unwrap();

                if info.constants.contains_key(&key) {
                    error("You cannot define a constant.");
                    break;
                }

                if !suppress_out && !supress_all {
                    out(&format!("{} = {}", key, value));
                }

                suppress_out = false;

 
                info.constants.insert(key, value);
            }

            "help" => {
                print_help();
            }

            "reset" => {
                if !info.script.is_empty() {

                    for ident in info.script.iter() {
                        if info.variables.contains_key(ident) {
                            info.variables.remove(ident);
                            if !suppress_out && !supress_all {
                                out(&format!("Variable '{}' removed.", ident));
                            }
                        } else if info.functions.contains_key(ident) {
                            info.functions.remove(ident);
                            if !suppress_out && !supress_all {
                                out(&format!("Function '{}' removed.", ident));
                            }
                        } else {
                            error(&format!("Could not find identifier '{}'.", ident));
                        }
                    }
                    suppress_out = false;
                    info.script.clear();
                    continue;
                }

                info.reset();
                if !suppress_out {
                    out("Variables and functions reset.");
                }

                suppress_out = false;
            }

            "-s" => {
                suppress_out = true;
            }

            "-sall" => {
                supress_all = true;
            }

            "dup" => {
                if info.stack.is_empty() {
                    error("Nothing to duplicate.");
                    break;
                }

                info.stack.push(*info.stack.last().unwrap());
            }

            "drop" => {
                if info.stack.is_empty() {
                    error("Nothing to drop.");
                    break;
                }

                info.stack.pop();
            }

            "fn" => {
                if info.idents.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let copy = copy_vec(&info.script);
                let ident = info.idents.pop().unwrap();

                if !suppress_out && !supress_all {
                    out(&format!("{} = function {:?}", ident, copy));
                }

                suppress_out = false;
                info.functions.insert(ident, copy);
                info.script.clear();
            }

            "clear" => {
                print!("\x1b[2J");
                stdout().flush().expect("Could not clear screen.");
            }

            "variables" => {
                if info.variables.len() == 0 {
                    out("None");
                    continue;
                }

                out("\n");
                for variable in info.variables.keys() {
                    println!("\t{}{}{}{} = {}", 
                    BOLD,
                    variable, 
                    DEFAULT,
                    GREEN,
                    info.variables.get(variable).unwrap());
                }
            },

            "ansi" => {

                if info.script.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                let cmd = vec_to_line(&info.script);
                print!("\x1b{}", cmd);
                stdout().flush().expect("Could not execute ANSI command.");
                info.script.clear();
            },

            "namespaces" => {
                if info.namespaces.is_empty() {
                    out("None");
                    continue;
                }

                out("\n");

                for namespace in &info.namespaces {
                    println!("\t{BOLD}{}{DEFAULT}{GREEN}", namespace.title);
                }
            }

            "functions" => {
                if info.functions.len() == 0 && info.namespaces.is_empty(){
                    out("None");
                    continue;
                }

                out("\n");

                for namespace in &info.namespaces {
                    println!("\tNAMESPACE: {UNDERLINE}{BOLD}{}{DEFAULT}{GREEN}", namespace.title);
                    for func in namespace.functions.keys() {
                        println!(
                            "\t{}{}{}{} = {:?}",
                            BOLD,
                            func,
                            DEFAULT,
                            GREEN,
                            namespace.functions.get(func).unwrap()
                        );
                    }
                }

                println!("\t{UNDERLINE}{BOLD}N/A{DEFAULT}{GREEN}");
                for function in info.functions.keys() {
                    println!(
                        "\t{}{}{}{} = {:?}",
                        BOLD,
                        function,
                        DEFAULT,
                        GREEN,
                        info.functions.get(function).unwrap()
                    );
                }
            }

            "out" => {
                if info.script.is_empty() {
                    out("");
                    continue;
                }

                let res = vec_to_line(&info.script);
                out(&res);
                info.script.clear();
            }

            "flush" => {
                if info.script.is_empty() {
                    println!("");
                    continue;
                }

                let res = vec_to_line(&info.script);
                println!("{}", res);
                info.script.clear();
            }

            "namespace" => {
                if info.idents.is_empty() {
                    error("Namespace requires a name.");
                    break;
                }

                let title = info.idents.pop().unwrap();
                let mut namespace = Namespace::new(title.to_owned());
                let mut namespace_session = SessionInfo::new();
                interpret_line(vec_to_line(&info.script), &mut namespace_session);
                for func in namespace_session.functions {
                    let name = func.0;
                    let contents = func.1;
                    namespace.functions.insert(name, contents);
                }
                info.namespaces.push(namespace);
                info.script.clear();
                
            },

            "pushch" => {
                if info.stack.is_empty() {
                    error("Insufficient arguments!");
                    break;
                }

                if info.script.is_empty() {
                    let item = String::from(info.stack.pop().unwrap().to_string());
                    info.script.push(item);
                } else {
                    let item = String::from(info.stack.pop().unwrap().to_string());
                    let mut last = info.script.pop().unwrap();
                    last.push_str(item.as_str());
                    info.script.push(last);
                }
            },

            _ => {
                if let Ok(num) = token.parse::<f32>() {
                    info.stack.push(num);
                } else if info.contains_function(token.to_string()) {
                    let func = info.get_function(token.to_string());
                    //println!("{func:?}");
                    interpret_vec(&func, info);
                } else {
                    if token.trim() == "{" || token.trim() == "}" {
                        continue;
                    }
                    info.idents.push(token.to_owned());
                }
            }
        }
    }
}

}

pub mod namespaces {
    use std::collections::HashMap;

    pub struct Namespace {
        pub title: String,
        pub functions: HashMap<String, Vec<String>>
    }

    impl Namespace {
        pub fn new(title: String) -> Self {
            Self {
                title: title,
                functions: HashMap::new(),
            }
        }
    }

}