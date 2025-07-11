// Reader
use regex::{Captures, Regex};
use lazy_static::lazy_static;

use crate::types::{hash_map, BeeErr, BeeRet, BeeVal};
use crate::types::error;

use std::rc::Rc;

use types::BeeVal::{Sym, List, Nil};

#[derive(Debug, Clone)]
struct Reader{
    tokens: Vec<String>,
    pos: usize
}

impl Reader{
    /// Get next token and increment position
    fn get_token(&mut self) -> Result<String, BeeErr>{
        Ok(
            self.tokens
                .get(self.pos)
                .ok_or_else(|| BeeErr::ErrString("underflow".to_string()))?
                .to_string()
        )
    }
    /// Increment
    fn step(&mut self){
        self.pos += 1;
    }
}

/// Get lisp tokens from line of code
fn tokenize(str: &str) -> Vec<String>{
    lazy_static!{
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }
    
    let mut res = vec![];
    for cap in RE.captures_iter(str){
        if cap[1].starts_with(';') {
            // end of line
            continue
        }
        res.push(String::from(&cap[1]));
    }
    res
}

/// Remove things like quotation from string data
fn unescape_str(s: &str) -> String{
    lazy_static!{
        static ref RE: Regex = Regex::new(r#"\\(.)"#).unwrap();
    }
    RE.replace_all(s, |caps: &Captures|{
        if &caps[1] == "n" {"\n"} else{&caps[1]}.to_string()
    }).to_string()
}

/// Read atom
/// This can be data or function
fn read_atom(reader: &mut Reader) -> BeeRet{
    lazy_static!{
        static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }
    let token = reader.get_token()?;
    reader.step();
    match &token[..]{
        "nil" => Ok(BeeVal::Nil),
        "false" => Ok(BeeVal::Bool(false)),
        "true" => Ok(BeeVal::Bool(true)),
        _ => {
            if INT_RE.is_match(&token){
                Ok(BeeVal::Int(token.parse().unwrap()))
            }
            else if STR_RE.is_match(&token){
                Ok(BeeVal::Str(unescape_str(&token[1..token.len() - 1])))
            }
            else if token.starts_with('\"'){
                error("Expected '\"', got EOF")
            }
            else if let Some(keyword) = token.strip_prefix(':'){
                Ok(BeeVal::Str(format!("\u{29e}{}", keyword)))
            }
            else{
                Ok(BeeVal::Sym(token.to_string()))
            }
        }
    }
}

/// Read a sequence until end of sequence reached
/// end is end of sequence token
fn read_seq(reader: &mut Reader, end: &str) -> BeeRet{
    let mut seq: Vec<BeeVal> = vec![];
    
    reader.step();
    
    loop{
        let token = match reader.get_token(){
            Ok(t) => t,
            Err(_) => return error(&format!("Expected '{}', got EOF", end))
        };
        
        if token == end{
            // end of sequence reached
            break;
        }
        // Token inside sequence
        seq.push(read_form(reader)?);
    }
    reader.step();
    match end{
        ")" => Ok(list!(seq)),
        "]" => Ok(vector!(seq)),
        "}" => hash_map(seq),
        _ => error("read_seq unknown end value")
    }
}

/// Read next token
fn read_form(reader: &mut Reader) -> BeeRet{
    let token = reader.get_token()?;
    
    match &token[..]{
        "'" => {
            reader.step();
            Ok(list![Sym("quote".to_string()), read_form(reader)?])
        },
        // Closing without an opening
        ")" => error("Unexpected ')'"),
        "]" => error("Unexpected ']'"),
        "}" => error("Unexpected '}'"),
        // Opening, start reading sequence/go one depth down
        "(" => read_seq(reader, ")"),
        "[" => read_seq(reader, "]"),
        "{" => read_seq(reader, "}"),
        _ => read_atom(reader)
    }
}

/// Read in a lisp string
pub fn read_str(str: &str) -> BeeRet{
    let tokens = tokenize(str);
    
    if tokens.is_empty(){
        panic!("No input");
    }
    
    read_form(&mut Reader{
        pos: 0,
        tokens: tokens
    })
}