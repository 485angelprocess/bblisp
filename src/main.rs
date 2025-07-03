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

#[derive(Debug, PartialEq, Eq)]
enum LispType{
    Imm,
    Int,
    String,
    Error
}

/// Argument with accoman
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
    pub fn num_closures(s: &String) -> (usize, usize){
        let mut openings = 0;
        let mut closings = 0;
        for c in s.chars(){
            if c == '('{
                openings += 1;
            }
            if c == ')'{
                closings += 1;
            }
        }
        (openings, closings)
    }

    fn parse(line: Vec<LispData>) -> Self{
        let mut args: Vec<LispData> = Vec::new();

        let mut counter = 0;
        assert_eq!(line[0].tag, LispType::Imm);
        // Get function name
        let user_line = line[0].s.clone().unwrap();
        let mut split_line = user_line.split(&['(', ',', ')'][..]);
        let op = split_line.next().unwrap();
        println!("Function name {}", op);
        
        // Ok get first argument
        if let Some(arg) = split_line.next(){
            // There is an additional 
            let new_arg = LispData::new_imm(arg.to_string());
            args.push(new_arg);
        }

        // TODO parse further values

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

fn run(c: Call, table: &FunctionTable){
    if let Some(p) = table.get_primitive(&c.op){
        // TODO match args
        match p{
            Primitive::PRINT => println!("{}", c.args[0].as_string()),
            Primitive::EVAL => println!("Eval {}", c.args[0].as_string()),
            _ => println!("Unimplemented {}", c.op)
        }
    }
}

fn main() {
    println!("Hi! ^_^");
    let response = prompt();

    let (opening, closing) = Call::num_closures(&response);

    let mut table = FunctionTable::new();

    // Build up tables
    table.insert_primitive("PRINT", Primitive::PRINT);
    table.insert_primitive("EVAL", Primitive::EVAL);
    table.insert_primitive("READ", Primitive::READ);

    if (opening == closing){
        match opening{
            0 => println!("No function {}", response),
            1 => {
                // parse
                println!("Function {}", response);

                // TODO: keep track of existing returns
                let line = vec![LispData::new_line(response)];

                let c = Call::parse(line);
                c.print();
                run(c, &table);
            },
            _ => {
                // Need to go a level deeper
                println!("Nested functions");
            }
        }
    }
    else{
        println!("Invalid closures");
    }

}
