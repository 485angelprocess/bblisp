extern crate regex;
extern crate lazy_static;
extern crate rustyline;

use std::collections::HashMap;

use std::rc::Rc;

#[macro_use]
mod types;

mod reader;
mod printer;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::{error, format_error, func, BeeArgs, BeeErr, BeeRet, BeeVal};

pub type Env = HashMap<String, BeeVal>;

fn read(str: &str) -> BeeRet{
    reader::read_str(str)
}

fn eval(ast: &BeeVal, env: &Env) -> BeeRet{
    match ast{
        BeeVal::Sym(sym) => Ok(env
                                    .get(sym)
                                    .ok_or_else(|| BeeErr::ErrString(format!("'{}' not found", sym)))?
                                    .clone()),
        BeeVal::Vector(v, _) => {
            let mut lst: BeeArgs = vec![];
            for a in v.iter(){
                lst.push(eval(a, env)?);
            }
            Ok(vector!(lst))
        },
        BeeVal::List(l, _) => {
            if l.is_empty(){
                return Ok(ast.clone());
            }
            let a0 = &l[0];
            let f = eval(a0, env)?;
            let mut args: BeeArgs = vec![];
            for i in 1..l.len(){
                args.push(eval(&l[i], env)?);
            }
            f.apply(args)
        }
        _ => Ok(ast.clone())
    }
}

fn print(ast: &BeeVal) -> String{
    ast.to_string()
}

fn rep(str: &str, env: &Env) -> Result<String, BeeErr>{
    let ast = read(str)?;
    let exp = eval(&ast, env)?;
    Ok(print(&exp))
}

fn int_op(op: fn(i64, i64) -> i64, a: BeeArgs) -> BeeRet{
    match (a[0].clone(), a[1].clone()){
        (BeeVal::Int(a0), BeeVal::Int(a1)) => Ok(BeeVal::Int(op(a0, a1))),
        _ => error("invalid int_op args")
    }
}

fn main() {
    println!("Hi! ^_^");
    
    let mut rl = Editor::<(), rustyline::history::DefaultHistory>::new().unwrap();
    if rl.load_history(".bee-history").is_err(){
        eprintln!("No previous history");
    }
    
    let mut env = Env::default();
    env.insert("+".to_string(), func(|a: BeeArgs| int_op(|i, j| i + j, a)));
    
    loop{
        let readline = rl.readline("user> ");
        match readline{
            Ok(line) => {
                let _ = rl.add_history_entry(&line);
                rl.save_history(".bee-history").unwrap();
                if !line.is_empty(){
                    match rep(&line, &env){
                        Ok(out) => {
                            println!("{}", out);
                        },
                        Err(e) => println!("Error: {}", format_error(e))
                    }
                }
            },
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) =>{
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
