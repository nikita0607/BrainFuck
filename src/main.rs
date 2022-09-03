extern crate rustyline;
extern crate strfmt;

use std::collections::HashMap;
use std::env;
use std::char;
use std::io::Read;
use std::io::stdin;

use strfmt::strfmt;
use rustyline::Editor;

use std::fs::File;


#[derive(Clone, Debug)]
enum Error {
    UnclosedBracket(usize, usize),
    UnopenedBracket(usize, usize)
}

impl Error {
    fn check_brackets(code: &String) -> Vec<Error>{
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

    fn check_errors(code: &String) -> Vec<Error> {
        let mut errors: Vec<Error> = vec![];

        for i in Error::check_brackets(code).into_iter() {
            errors.push(i);
        };

        errors
    }
}


enum Task {
    Function(usize, usize),
    Main(usize)
}


pub mod Lang {

    pub struct TokLang {
        pub start: String,
        pub add: String,
        pub sub: String,
        pub next: String,
        pub prev: String,
        pub input: String,
        pub output: String,
        pub start_loop: String,
        pub end_loop: String,
        pub end: String
    }

    impl TokLang {
        pub fn new(start: String, add: String, sub: String, 
               next: String, prev: String, input: String, output: String,
               start_loop: String, end_loop: String, 
               end: String) -> TokLang {
            
            TokLang { start, add, sub, next, prev, input, output, start_loop, end_loop, end }
        }
    }

    pub enum Lang {
        Rust,
    }
    
    impl Lang {
        pub fn to_tok_lang(lang: Lang) -> TokLang {
            match lang {
                Lang::Rust => { 
                    TokLang::new("
use std::char;
use std::io::stdin;

fn main() {
    let mut regs: Vec<i32> = vec![];
    let mut ind = 0
    let mut inp = String::new()\n".to_string(), 
    "regs[ind] = (regs[ind]+{num})%100\n".to_string(),
    "regs[ind] = (regs[ind]+100-{num})%100\n".to_string(),
    "ind = (ind+{num})%100\n".to_string(),
    "ind = (ind-{num})%100\n".to_string(),
    "stdin().read_line(&mut inp)
    regs[ind] = inp.chars().next_back().unwrap().to_digit()\n".to_string(),
    "println!(\"{}\", char::from_u32(regs[ind]).unwrap())\n".to_string(),
    "while regs[ind] > 0 {\n".to_string(),
    "}\n".to_string(),
    "}\n".to_string()) 
                }
            }
        }

        pub fn to_tok(self) -> TokLang {
            Lang::to_tok_lang(self)
        }
    }
}


#[derive(Debug)]
enum Token {
    Add(i32),
    Sub(i32),
    Next(i32),
    Prev(i32),
    Input,
    Output,
    Str(Box<str>),
    Loop(Vec<Token>)
}


struct Compiler {

}

impl Compiler {
    fn compile(code: &String) -> Vec<Error> {
        let errors: Vec<Error> = Error::check_errors(code);
        if errors.len() > 0 {
            return errors;
        }
        println!("{}", code);

        let mut tokens = Compiler::tokenize(code);
        Compiler::optimize(&mut tokens);
        Compiler::_compile(&mut tokens, 1, true);

        errors
    }

    fn _compile(tokens: &Vec<Token>, deep: usize, main: bool) -> String {
        let token_lang = Lang::Lang::to_tok_lang(Lang::Lang::Rust);

        
        let mut compiled_code = String::new();

        if main { compiled_code += token_lang.start.as_str(); }

        println!("{:?}", tokens);
        
        for token in tokens.iter() {
            println!("{:?}", token);
            compiled_code += "    ".repeat(deep).as_str();

            match token {
                &Token::Add(n) => {compiled_code += format_num(
                        &token_lang.add, n).as_str()},
                &Token::Sub(n) => {compiled_code += format_num(
                        &token_lang.sub, n).as_str()},
                &Token::Next(n) => {compiled_code += format_num(
                        &token_lang.next, n).as_str()}
                Token::Loop(toks) => {
                    compiled_code += token_lang.start_loop.as_str();
                    compiled_code += Compiler::_compile(&toks, 
                                                        deep+1, false).as_str();
                    compiled_code += "    ".repeat(deep).as_str();
                    compiled_code += token_lang.end_loop.as_str();
                },
                _ => {compiled_code += "\n"}
            }
        }
        
        if main { compiled_code += token_lang.end.as_str(); }

        println!("{}", compiled_code);

        compiled_code
   }

   fn optimize(token: &mut Vec<Token>) {
        let mut ind = 0;

        while ind < token.len()-1 {
            match token.get(ind).unwrap() {
               Token::Add(n) => {
                    if let Token::Add(n2) = token.get(ind+1).unwrap() {
                        token[ind] = Token::Add(n+n2);
                        token.remove(ind+1);
                        continue;
                    }
                    else if let Token::Sub(n2) = token.get(ind+1).unwrap() {
                        if n >= n2 { token[ind] = Token::Add(n-n2); }
                        else { token[ind] = Token::Sub(n2-n); }
                        token.remove(ind+1);
                        continue;
                    }
               }

               _ => {}
            }

            ind += 1;
        }
   }

   fn tokenize(code: &String) -> Vec<Token> {
       let mut tokens: Vec<Token> = vec![];

       let mut ind = 0;

       while ind < code.len() {
            let sym = code.get(ind..ind+1).unwrap();
            println!("{}", code);
            let token = match sym {
                "+" => Token::Add(1),
                "-" => Token::Sub(1),
                ">" => Token::Next(1),
                "<" => Token::Prev(1),
                "," => Token::Input,
                "." => Token::Output,
                "[" => {
                    let mut _open = 0;
                    let mut sec_ind = 0;
                    for i in ind..code.len() {
                        if code.get(i..i+1).unwrap() == "[" {
                            _open += 1;
                        }
                        else if code.get(i..i+1).unwrap() == "]" {
                            _open -= 1;
                            if _open == 0 {
                                sec_ind = i;
                                break;
                            }
                        }
                    }
                    let _ind = ind;
                    ind = sec_ind;

                    Token::Loop(Compiler::tokenize(&code
                                                   .get(_ind+1..sec_ind)
                                                   .unwrap().to_string()))
                },
                _ => Token::Add(0)
            };

            ind += 1;
            tokens.push(token);
        }

        tokens
   }
}


struct Interp {
    _syms: [char; 12]
}

impl Interp {

    fn new() -> Interp {
        Interp {_syms: ['+', '-', '(', ')', '[', ']', 
            '{', '}', '<', '>', ',', '.']}
    }

    fn check_code(&self, code: &String) -> Vec<Error> {
        let mut errors: Vec<Error> = vec![];

        for error in Error::check_brackets(&code).iter() {
            errors.push(error.clone());
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
                "+" => {if regs[current_reg] < 255 { regs[current_reg] += 1 } else { regs[current_reg] = 0 }},
                "-" => {if regs[current_reg] > 0 { regs[current_reg] -= 1} else { regs[current_reg] = 255} },
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
                    if let Some(sym) = input.pop() {
                      regs[current_reg] = sym as u32;
                    } 
                    else {
                      continue;
                    }
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


fn format_num(s: &String, num: i32) -> String {
    let mut map = HashMap::new();
    map.insert("num".to_string(), num);
    strfmt(s, &map).unwrap()
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
    println!("Ctrl+C to exit");

    let mut editor = Editor::<()>::new();
    let mut last_lines: Vec<String> = vec![];

    let interp = Interp::new();

    loop {   
        let code: String = editor.readline(">>> ")?.replace("\u{1b}[", " ");
        
        let errors = interp.check_code(&code);

        if errors.len() > 0 {
            for error in errors.into_iter() {
                print_error(&code, error.clone())   
            }
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
    let mut compile = false;

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
            "-c" => {
                compile = true;
            }
            _ => {}
        }
    }
    
    match file_name {
        Some(_file_name) => {
            let mut file = File::open(_file_name.as_str())
                .expect("File not found!");
            
            let mut code = String::new();
            file.read_to_string(&mut code).expect("Can't read from file!");
            
            if !compile {
                run_code(code);
            }
            else {
                println!("Compile!\n");
                Compiler::compile(&code);
            }
        }
        None => one_line_mode()?
    }
    
    Ok(())
}
