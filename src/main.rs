use std::{collections::HashMap, fmt::Display, io::Write};

pub enum Primitive{
    READ,
    EVAL,
    PRINT,
    rep
}

pub struct FunctionTable{
    prim: HashMap<String, Primitive>
}

impl FunctionTable{
    pub fn new() -> Self{
        Self{
            prim: HashMap::new()
        }
    }
    pub fn insert_primitive(&mut self, s: &'static str, p: Primitive){
        self.prim.insert(s.to_string(), p);
    }

    pub fn get_primitive(&self, s: &String) -> Option<&Primitive>{
        self.prim.get(s)
    }
}

fn input(s: &'static str) -> String{
    print!("{}", s);
    std::io::stdout().flush();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn prompt() -> String{
    input("> ")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LispType{
    Imm,
    Int,
    String,
    Error
}

/// Argument with accoman
#[derive(Clone)]
struct LispData{
    pub tag: LispType,
    pub s: Option<String>,
    pub d: Option<isize>
}

impl LispData{
    /// Create and parse argument from user written program
    pub fn new_imm(s: String) -> Self{
        // Parse user written data
        if s.contains('"'){
            let sp = s.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
            Self{
                tag: LispType::String,
                s: Some(sp.to_string()),
                d: Some(sp.len().try_into().unwrap())
            }
        }
        else{
            Self{
                tag: LispType::Int,
                s: None,
                d: Some(s.parse::<isize>().expect("Integer Conversion"))
            }
        }
    }

    /// Just add tag to user input
    pub fn new_line(s: String) -> Self{
        Self{
            tag: LispType::Imm,
            s: Some(s),
            d: None
        }
    }

    /// Get first inner enclosure
    pub fn get_inner(v: &Vec<LispData>) -> (Vec<LispData>, Vec<LispData>, Vec<LispData>){
        let mut inner = Vec::new();
        let mut substr = String::new();

        let mut post = false;

        let mut before = Vec::new();
        let mut after = Vec::new();

        let mut depth = 0;

        for d in v{
            match d.tag{
                LispType::Imm => {
                    for c in d.s.as_ref().unwrap().chars(){
                         substr.push(c);
                        
                        if c == '('{
                            if depth == 0 && !post{
                                before.push(LispData::new_line(substr.clone()));
                                substr.clear();
                            }
                            depth += 1;
                        }
                        else if c == ')'{
                            depth -= 1;
                            if depth == 0{
                                // Keep track of other values
                                if post{
                                    after.push(LispData::new_line(substr.clone()));
                                }
                                substr.clear();
                            }
                            if depth == 1{
                                if substr.len() > 0 {
                                    inner.push(LispData::new_line(substr.clone()));
                                    substr.clear();
                                    for i in &inner{
                                        println!("inner: {}", i.as_string());
                                    }
                                    post = true;
                                }
                            }
                        }
                    }
                },
                _ => {
                    if depth == 0{
                        // KEep track of before and after
                        if post{
                            if substr.len() > 0{
                                // Push new substring
                                after.push(LispData::new_line(substr.clone()));
                                substr.clear();
                            }
                            // Push data representation
                            after.push(d.clone());
                        }
                        else{
                            if substr.len() > 0{
                                // Push new substring
                                before.push(LispData::new_line(substr.clone()));
                                substr.clear();
                            }
                            // Push data representation
                            before.push(d.clone());
                        }
                    }
                    else{
                        if substr.len() > 0{
                            // Push new substring
                            inner.push(LispData::new_line(substr.clone()));
                            substr.clear();
                        }
                        // Push data representation
                        inner.push(d.clone());
                    }
                }
            }
        }
        return (inner, before, after);
    }

    pub fn as_string(&self) -> String{
        match self.tag{
            LispType::Imm => format!("{}", self.s.clone().unwrap()),
            LispType::String => format!("\"{}\"", self.s.clone().unwrap()),
            LispType::Int => format!("d{}", self.d.unwrap()),
            _ => "Unknown".to_string()
        }
    }
}

struct Call{
    pub op: String,
    pub args: Vec<LispData>
}

impl Call{
    /// Get number of of opening and closing parantheses
    pub fn num_closures(data: &Vec<LispData>) -> (usize, usize){
        let mut openings = 0;
        let mut closings = 0;

        for d in data{
            match d.tag{
                LispType::Imm => {
                    // Parse any written stuff and find parantheses
                    for c in d.s.as_ref().unwrap().chars(){
                        if c == '('{
                            openings += 1;
                        }
                        if c == ')'{
                            closings += 1;
                        }
                    }
                },
                _ => ()
            }
        }
        
        (openings, closings)
    }

    /// Get a function call with arguments from line of code
    fn parse(line: &Vec<LispData>) -> Self{
        let mut args: Vec<LispData> = Vec::new();

        let mut counter = 0;
        assert_eq!(line[0].tag, LispType::Imm);
        // Get function name
        let user_line = line[0].s.clone().unwrap();
        let mut split_line = user_line.split(&['(', ',', ')'][..]);
        let op = split_line.next().unwrap();
        println!("Function name {}", op);
        
        let mut i = 0;

        while counter < line.len() && i < 100{
            match line[counter].tag{
                // Get arguments from existing values
                LispType::Imm => {
                    if let Some(arg) = split_line.next(){
                        if arg.len() == 0{
                            counter += 1;
                        }
                        else{
                            // There is an additional 
                            //println!("Parsing arg {}", arg.to_string());
                            let new_arg = LispData::new_imm(arg.to_string());
                            args.push(new_arg);
                        }
                    }
                    else{
                        counter += 1;
                    }
                },
                _ => {
                    // Already parsed data
                    args.push(line[counter].clone());
                    counter += 1;
                }
            }
            i += 1;
        }
        
        Self{
            op: op.to_string(),
            args: args
        }
    }

    fn print(&self){
        // Display function and args nicely
        print!("{}: ", self.op);
        for a in &self.args{
            print!("{} ", a.as_string())
        }
        println!(" ");
    }
}

/// Print output and return first parameter
fn lisp_print(args: &Vec<LispData>) -> Vec<LispData>{
    println!("{}", args[0].as_string());
    vec![args[0].clone()]
}

/// Eval STUB
fn lisp_eval(args: &Vec<LispData>) -> Vec<LispData>{
    println!("Eval: {}", args[0].as_string());
    vec![args[0].clone()]
}

fn run(c: Call, table: &FunctionTable) -> Vec<LispData>{
    if let Some(p) = table.get_primitive(&c.op){
        // TODO match args
        return match p{
            Primitive::PRINT => lisp_print(&c.args),
            Primitive::EVAL => lisp_eval(&c.args),
            _ => {
                println!("Unimplemented {}", c.op);
                Vec::new()
            }
        }
    }

    Vec::new()
}

fn main() {
    println!("Hi! ^_^");

    let mut table = FunctionTable::new();

    // Build up tables
    table.insert_primitive("PRINT", Primitive::PRINT);
    table.insert_primitive("EVAL", Primitive::EVAL);
    table.insert_primitive("READ", Primitive::READ);

    let response = vec![LispData::new_line(prompt())];

    // Add response to line
    run_line(&response, &table, 0);
}

/// Go through a line
fn run_line(response: &Vec<LispData>, table: &FunctionTable, depth: usize) -> Vec<LispData>{
    if depth > 100{
        panic!("Too much nesting");
    }

    let mut r = response.clone();

    for i in 0..100{
        let (opening, closing) = Call::num_closures(&r);

        println!("{}: Number of openings {}", i, opening);

        if (opening == closing){
            match opening{
                0 => panic!("No function"),
                1 => {
                    // parse
                    let c = Call::parse(&r);
                    c.print();
                    let result = run(c, table);
                    return result;
                },
                _ => {
                    // Need to go a level deeper
                    println!("Nested functions");
                    let (inner, before, after) = LispData::get_inner(response);
                    let result = run_line(&inner, &table, depth + 1);

                    print!("Before: ");
                    for d in &before{
                        print!("{}, ", d.as_string());
                    }
                    println!(" ");

                    r = [before, result, after].concat();

                    print!("Result of nest: ");
                    for d in &r{
                        print!("{}, ", d.as_string());
                    }
                    println!(" ");
                }
            }
        }
        else{
            panic!("Invalid closures");
        }
    }
    Vec::new()
}
