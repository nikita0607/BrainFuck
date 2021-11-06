extern crate rustyline;

use std::collections::HashMap;
use std::env;
use std::char;
use std::io::Read;
use std::io::stdin;
use rustyline::Editor;

use std::fs::File;


#[derive(Debug)]
#[derive(Clone)]
enum Error {
    UnclosedBracket(usize, usize),
    UnopenedBracket(usize, usize)
}


enum Task {
    Function(usize, usize),
    Main(usize)
}


struct Interp{
    _syms: [char; 12]
}

impl Interp {

    fn new() -> Interp {
        Interp {_syms: ['+', '-', '(', ')', '[', ']', 
            '{', '}', '<', '>', ',', '.']}
    }

    fn check_code(&self, code: &String) -> Vec<Error> {
        let mut errors: Vec<Error> = vec![];

        for error in self._check_brackets(&code).iter() {
            errors.push(error.clone());
        };

        errors
    }

    fn _check_brackets(&self, code: &String) -> Vec<Error> {
        let mut _brackets: Vec<(u8, usize)> = vec![];
        let mut errors: Vec<Error> = vec![];
        let mut string_id: usize = 0;

        let mut in_comment = false;

        for (id, ch) in code.chars().enumerate() {
            if ch == '\n' || ch == ' ' {
                string_id += 1;
                continue;
            }

            if in_comment {
                in_comment = false;
                continue;
            }

            if ch == '`' {
                in_comment = true;
                continue;
            }

            if ch == '(' {
                _brackets.push((0, id));
                continue;
            }
            else if ch == '{' {
                _brackets.push((1, id));
                continue;
            }
            else if ch == '[' {
                _brackets.push((2, id));
                continue;
            }

            if ch != ')' && ch != '}' && ch != ']' {
                continue;
            }

            let mut _len = _brackets.len();
            if _len == 0 {
                errors.push(Error::UnopenedBracket(string_id, id));
                continue;
            }
            
            _len -= 1;
            let _sym = _brackets.get(_len).unwrap().0;

            if ch == ')' {
                if _sym != 0 {
                    errors.push(Error::UnopenedBracket(string_id, id));
                }
                else {
                    _brackets.remove(_len);
                }
            }
            else if ch == '}' {
                if _sym != 1 {
                    errors.push(Error::UnopenedBracket(string_id, id));
                }
                else {
                    _brackets.remove(_len);
                }

            }
            else if ch == ']' {
                if _sym != 2 {
                    errors.push(Error::UnopenedBracket(string_id, id));
                }
                else {
                    _brackets.remove(_len);
                }

            }
            
        };

        for (.., id) in _brackets.iter() {
            errors.push(Error::UnclosedBracket(string_id, *id))
        };

        errors
    }

    fn interpret_main(&self, code: String) {
        self.interpret(code, &mut [0;100], 0, Task::Main(0),
        &mut vec![], &mut HashMap::new(), &mut [0;100]);
    }

    fn interpret(&self, code: String, regs: &mut [u32;100], mut id: usize, 
                 task: Task, loops: &mut Vec<usize>, 
                 func: &mut HashMap<u32, usize>, main_regs: &mut [u32;100]) {

        let mut current_reg = 0;
        let mut input = String::new();
        // println!("RUN CODE: {}, {}", code, id);

        let mut in_comment = false;

        while id < code.len() {
            if in_comment {
                in_comment = false;
                id += 1;
                continue;
            }

            let sym = code.get(id..id+1).unwrap();
            // println!("Current sym: {}", sym);

            match sym {
                "`" => in_comment = true,
                "+" => {regs[current_reg] += 1},
                "-" => {if regs[current_reg] > 0 {regs[current_reg] -= 1} else {regs[current_reg] = 255} },
                ">" => {if current_reg < 99 {current_reg += 1;} else {current_reg=0;}},
                "<" => {if current_reg > 0 {current_reg -= 1;} else {current_reg=99}},
                "." => {print!("{}", char::from_u32(regs[current_reg]).unwrap())},
                "," => {
                    if input.len() < 1 {
                        stdin().read_line(&mut input).unwrap();
                        input = input.chars()
                            .rev()
                            .collect::<String>()
                            .replace("\n", "");
                    }
                    regs[current_reg] = input.pop().unwrap() as u32;
                }
                "[" => {loops.push(id)},
                "]" => {
                    if regs[current_reg] > 0 {
                        id = loops[loops.len()-1].try_into().unwrap();
                    }       
                    else 
                        {loops.remove(loops.len()-1);}
                },
                "(" => {
                    if !func.contains_key(&regs[current_reg]) {
                        func.insert(regs[current_reg], id);
                        
                        let mut _close = 1;
                        id += 1;
                        loop { 
                            let _sym = code.get(id..id+1).unwrap();
                            if _sym == "(" {_close += 1}
                            else if _sym == ")" {_close -= 1}
                            
                            if _close == 0 {break;};

                            id += 1;
                        };
                    }

                    else {
                        let mut _id = id+1;
                        let _task = Task::Function(_id.clone(), *func.get(&regs[current_reg]).clone().unwrap());
                        
                        let mut _close = 1;
                        id += 1;
                        loop { 
                            let _sym = code.get(id..id+1).unwrap();
                            if _sym == "(" {_close += 1}
                            else if _sym == ")" {_close -= 1}
                            
                            if _close == 0 {break;};

                            id += 1;
                        };
                    
                        self.interpret(code.clone(), &mut [0;100], _id, _task, &mut vec![], &mut HashMap::new(), main_regs);
                    }
                },
                ")" => {
                    match task {
                        Task::Function(first_id, second_id) => {
                            if id >= first_id {
                                id = second_id;
                            }

                            else {return}
                        }
                        _ => println!("Error?")
                    }
                }

                "{" => {
                    let _id = id+1;
                    
                    let mut _close = 1;
                    id += 1;
                    loop {
                        let _sym = code.get(id..id+1).unwrap();

                        if _sym == "{" {_close += 1}
                        else if _sym == "}" {_close -= 1}

                        if _close == 0 {break;};
                        
                        id += 1;
                    }

                    self.interpret(code.clone(), main_regs, _id, Task::Main(_id), &mut vec![], &mut HashMap::new(), regs);


                },
                "}" => return,
                _ => {}
            }

            id += 1;
        };
    }
}


fn print_error(s: &String, error: Error) {
    let mut string_id: usize = 0;
    let mut sym_id: usize = 0;
    let mut error_string = String::new();

    match error {
        Error::UnopenedBracket(_string_id, _sym_id) => {
            error_string = String::from("Bracket was newer opened");
            sym_id = _sym_id;
            string_id = _string_id;
        },
        Error::UnclosedBracket(_string_id, _sym_id) => {
            error_string = String::from("Bracket was newer closed");
            sym_id = _sym_id;
            string_id = _string_id;
        },
    };

    let shift = " ".repeat(format!("{}:   ", string_id).len());
    println!("{}:   {}", string_id, s);
    println!("{}{}", shift, " ".repeat(sym_id)+"^");
    
    for i in 0..3 {
        println!("{}{}", shift, " ".repeat(sym_id)+"|");
    }

    println!("{}", "-".repeat(sym_id+1+shift.len()));
    println!("{}", error_string);
    println!();
}


fn one_line_mode() -> rustyline::Result<()>{
    println!("Run one-line mode!");
    let mut editor = Editor::<()>::new();
    let mut last_lines: Vec<String> = vec![];

    let mut interp = Interp::new();

    loop {   
        let code: String = editor.readline(">>> ")?.replace("\u{1b}[", " ");
        
        let errors = interp.check_code(&code);

        if errors.len() > 0 {
            for error in errors.iter() {
                print_error(&code, error.clone())   
            }
            println!("_________________________________");
            continue;
        }

        if code.trim() == "exit" { break; }

        interp.interpret_main(code);

        println!("\n>End of programm<");
    };

    Ok(())
}


fn run_code(code: String) {
    let interp = Interp::new();

    let errors = interp.check_code(&code);

    if errors.len() > 0 {
        for error in errors.iter() {
            print_error(&code, error.clone())   
        }
        return;
    };
    
    interp.interpret_main(code);
    println!();
}


fn main() -> rustyline::Result<()> {
    let help_string = "-f FILE_NAME: Interpretate file with FILE_NAME\n
        -h: See this message";

    let args: Vec<String> = env::args().collect();
    
    let mut file_name: Option<String> = None;

    for (id, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-h" => {
                println!("{}", help_string);
                return Ok(());
            },
            "-f" => {
                if args.len() > id+1 {
                    file_name = Some((*args.get(id+1).expect("Enter file name!")).clone());
                } 
            },
            _ => {}
        }
    }
    
    match file_name {
        Some(_file_name) => {
            let mut file = File::open(_file_name.as_str())
                .expect("File not found!");
            let mut code = String::new();
            file.read_to_string(&mut code).expect("Can't read from file!");
            run_code(code);
        }
        None => one_line_mode()?
    }
    
    Ok(())
}
