use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
type Version = usize;

#[derive(Debug, Clone)]
enum Term {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Def(Version),
    Val(Value),
}
#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}
#[derive(Default)]
pub struct Forth {
    stk: Vec<Value>,
    latest: HashMap<String, Version>,
    definitions: Vec<Vec<Term>>,
}
impl Forth {
    fn parse_term(&mut self, input: &str) -> Result<Term, Error> {
        use Term::*;
        match input.to_lowercase().as_str() {
            word if self.latest.contains_key(word) => Ok(Def(self.latest[word])),
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "dup" => Ok(Dup),
            "drop" => Ok(Drop),
            "swap" => Ok(Swap),
            "over" => Ok(Over),
            word if word.parse::<Value>().is_ok() => Ok(Val(word.parse().unwrap())),
            _ => Err(Error::UnknownWord),
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn stack(&self) -> &[Value] {
        self.stk.as_slice()
    }
    fn new_def<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> ForthResult {
        if let Some(name) = iter.next() {
            let name = name.to_lowercase();
            if name.parse::<Value>().is_ok() {
                return Err(Error::InvalidWord);
            }
            let mut cmd = vec![];
            while let Some(x) = iter.next() {
                if x == ";" {
                    self.latest.insert(name, self.definitions.len());
                    self.definitions.push(cmd);
                    return Ok(());
                } else {
                    cmd.push(self.parse_term(x)?);
                }
            }
        }
        return Err(Error::InvalidWord);
    }
    fn pop(&mut self) -> Result<Value, Error> {
        if let Some(x) = self.stk.pop() {
            Ok(x)
        } else {
            Err(Error::StackUnderflow)
        }
    }
    fn arith(&mut self, op: impl FnOnce(Value, Value) -> Value) -> ForthResult {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stk.push(op(a, b));
        Ok(())
    }
    fn eval_term(&mut self, term: Term) -> ForthResult {
        use Term::*;
        match term {
            Add => self.arith(std::ops::Add::add)?,
            Sub => self.arith(std::ops::Sub::sub)?,
            Mul => self.arith(std::ops::Mul::mul)?,
            Div => {
                let b = self.pop()?;
                let a = self.pop()?;
                if b == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.stk.push(a / b);
            }
            Dup => {
                let a = self.pop()?;
                self.stk.push(a);
                self.stk.push(a);
            }
            Drop => {
                self.pop()?;
            }
            Swap => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.stk.push(a);
                self.stk.push(b);
            }
            Over => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.stk.push(b);
                self.stk.push(a);
                self.stk.push(b);
            }
            Val(n) => self.stk.push(n),
            Def(n) => {
                for i in self.definitions[n].clone() {
                    self.eval_term(i)?;
                }
            }
        }
        Ok(())
    }
    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut iter = input.split_whitespace();
        while let Some(word) = iter.next() {
            if word == ":" {
                self.new_def(&mut iter)?;
            } else {
                let term = self.parse_term(word)?;
                self.eval_term(term)?;
            }
        }
        Ok(())
    }
}
