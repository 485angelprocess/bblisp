use std::rc::Rc;

/// Type definitions

/// Lisp data type
#[derive(Clone)]
pub enum BeeVal{
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Sym(String),
    List(Rc<Vec<BeeVal>>, Rc<BeeVal>),
    Vector(Rc<Vec<BeeVal>>, Rc<BeeVal>),
    Func(fn(BeeArgs) -> BeeRet, Rc<BeeVal>)
}

impl BeeVal{
    pub fn to_string(&self) -> String{
        self.pr_str(true)
    }
}

pub enum BeeErr{
    ErrString(String),
    ErrBeeVal(BeeVal)
}

pub type BeeArgs = Vec<BeeVal>;
pub type BeeRet = Result<BeeVal, BeeErr>;

macro_rules! list {
  ($seq:expr) => {{
    List(Rc::new($seq),Rc::new(Nil))
  }};
  [$($args:expr),*] => {{
    let v: Vec<BeeVal> = vec![$($args),*];
    List(Rc::new(v),Rc::new(Nil))
  }}
}

macro_rules! vector {
    ($seq:expr) => {{
        BeeVal::Vector(Rc::new($seq), Rc::new(BeeVal::Nil))   
    }};
    [$($args:expr),*]=> {{
        let v: Vec<BeeVal> = vec![$($args),*];
        BeeVal::Vector(Rc::new(v), Rc::new(BeeVal::Nil))
    }}
}

pub fn hash_map(args: BeeArgs) -> BeeRet{
    todo!("Implement hash map");
}

impl BeeVal{
    
    /// Run function with arguments
    pub fn apply(&self, args: BeeArgs) -> BeeRet{
        match self{
            BeeVal::Func(f, _) => f(args),
            _ => error("Attempt to call non function")
        }
    }
}

/// Create error message
pub fn error(s: &str) -> BeeRet{
    Err(BeeErr::ErrString(s.to_string()))
}

/// Format error into string message
pub fn format_error(e: BeeErr) -> String{
    match e{
        BeeErr::ErrString(s) => s,
        BeeErr::ErrBeeVal(mv) => mv.pr_str(true)
    }
}

/// From lambda to lisp data
pub fn func(f: fn(BeeArgs) -> BeeRet) -> BeeVal{
    BeeVal::Func(f, Rc::new(BeeVal::Nil))
}