pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type Stack = Vec<Value>;
type Word = String;

pub struct Forth(Stack);

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Operator{
    Mult,
    Div,
    Add,
    Sub,
}


#[derive(Debug, PartialEq)]
pub enum Expression {
    Val (Value),
    Arith (Operator),
    Manip (Word),
    Def (Word),
}

impl Forth {
    pub fn new() -> Forth {
        Forth(Vec::new())
    }

    pub fn stack(&self) -> &[Value] {
        let Forth(stack) = self;
        &stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        unimplemented!("result of evaluating '{}'", input)
    }

    fn parse_expr(input: &str) ->std::result::Result<&str, Error>{
        unimplemented!("parsing string as Expression")
    }

    fn eval_expr(&mut self, expr: Expression) -> Result{
        unimplemented!("evaluating expression")
    }
}
