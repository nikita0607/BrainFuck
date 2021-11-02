extern crate rustyline;

use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::char;
use rustyline::Editor;


enum Answers {
    Exec(String),
    Continue
}

struct Answer(bool, String);


fn intrepret(code: String, regs: &mut [u32;100], ) -> Answers {
    let mut current_reg = 0;
    let mut id = 0;

    let mut loops: Vec<usize> = vec![];
    let mut func: HashMap<u32, usize> = HashMap::new();

    while id < code.len() {
        let sym = code.get(id..id+1).unwrap();
         
        match sym {
            "+" => {regs[current_reg] += 1},
            "-" => {if regs[current_reg] > 0 {regs[current_reg] -= 1} else {regs[current_reg] = 255} },
            ">" => {if current_reg < 99 {current_reg += 1;} else {current_reg=0;}},
            "<" => {if current_reg > 0 {current_reg -= 1;} else {current_reg=99}},
            "." => {print!("{}", char::from_u32(regs[current_reg]).unwrap())},
            "[" => {loops.push(id)},
            "]" => {if regs[current_reg] > 0 {id=loops[loops.len()-1].try_into().unwrap();} else {loops.remove(loops.len()-1);}},
            "(" => {
                if !func.contains_key(&regs[current_reg]) {
                    func.insert(regs[current_reg], id.clone());
                    while code.get(id..id+1).unwrap() != ")" {id += 1;};
                }

                else {
                    let mut new_code = String::new();
                    let mut _close = 0;
                    
                    let sl = id+1..code.len();
                    for i in sl {
                        let _sym: &str = code.get(i..i+1).unwrap();
                        if _sym == ")" {
                            if _close == 0 {
                                id = i;
                                break;
                            }
                            else {_close -= 1};
                        }

                        else if _sym == "(" {_close += 1};

                        new_code.push_str(&_sym);
                    }
                        
                    _close = 0;
                    let sl = *func.get(&regs[current_reg]).unwrap()+1..code.len();
                    for i in sl {
                        let _sym: &str = code.get(i..i+1).unwrap();
                        if _sym == ")" {
                            if _close == 0 {
                                break;
                            }
                            else {_close -= 1};
                        }

                        else if _sym == "(" {_close += 1};

                        new_code.push_str(&_sym);
                    }

                    loop {
                        let ans = intrepret(new_code.clone(), &mut [0;100]);
                        match ans {
                            Answers::Continue => {break;},
                            _ => {}
                        }
                    }
                }
            },

            ")" => {},
            " " | "\n" => {},
            _ => print!("{}: unknown command!", &sym)
        }

        id += 1;
    };

    Answers::Continue
}


fn main() -> rustyline::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let file_name: Option<String> = None;
    

    println!("Run one-line mode!");
    let mut editor = Editor::<()>::new();
    let mut regs = [0;100];
    let mut last_lines: Vec<String> = vec![];

    loop {   
        let code: String = editor.readline(">>> ")?;

        if code.len() < 1 || code == "\n" || code == "exit" { break; }
        if code == "clear" {regs = [0;100]; continue;};
        
        intrepret(code, &mut regs);
        println!("\nEnd");
    }

    Ok(())
}
