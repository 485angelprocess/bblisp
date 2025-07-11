use crate::types::BeeVal;

fn escape_str(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

impl BeeVal {
    pub fn pr_str(&self, print_readably: bool) -> String {
        match self {
            BeeVal::Nil => String::from("nil"),
            BeeVal::Bool(true) => String::from("true"),
            BeeVal::Bool(false) => String::from("false"),
            BeeVal::Int(i) => format!("{}", i),
            //Float(f)    => format!("{}", f),
            BeeVal::Str(s) => {
                if let Some(keyword) = s.strip_prefix('\u{29e}') {
                    format!(":{}", keyword)
                } else if print_readably {
                    format!("\"{}\"", escape_str(s))
                } else {
                    s.clone()
                }
            }
            BeeVal::Sym(s) => s.clone(),
            BeeVal::List(l, _) => pr_seq(l, print_readably, "(", ")", " "),
            BeeVal::Vector(l, _) => pr_seq(l, print_readably, "[", "]", " "),
            BeeVal::Func(_,_) => String::from("#<builtin>")
        }
    }
}

pub fn pr_seq(
    seq: &[BeeVal],
    print_readably: bool,
    start: &str,
    end: &str,
    join: &str
) -> String {
    let strs: Vec<String> = seq.iter().map(|x| x.pr_str(print_readably)).collect();
    format!("{}{}{}", start, strs.join(join), end)
}