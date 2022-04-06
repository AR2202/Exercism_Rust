use intmap::IntMap;
use std::collections::HashMap;
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub type Names = HashMap<String, u64>;
type Dict = IntMap<Expression>;
type Stack = Vec<Value>;

#[derive(Debug, PartialEq, Clone)]
pub struct Definition(String, String);

#[derive(Debug, PartialEq)]
pub struct Forth(Stack, Dict, Names);

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Word {
    Dup,
    Drop,
    Swap,
    Over,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Mult,
    Div,
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Val(Value),
    Arith(Operator),
    Manip(Word),
    Def(Definition),
    Address(u64),
    Func(Vec<Expression>),
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}
impl Forth {
    pub fn new() -> Forth {
        let initial_names: Names = HashMap::from([
            (String::from("dup"), 1),
            (String::from("drop"), 2),
            (String::from("swap"), 3),
            (String::from("over"), 4),
            (String::from("+"), 5),
            (String::from("-"), 6),
            (String::from("*"), 7),
            (String::from("/"), 8),
        ]);
        let initial_defs: Dict = IntMap::from_iter(
            [
                (1, Expression::Manip(Word::Dup)),
                (2, Expression::Manip(Word::Drop)),
                (3, Expression::Manip(Word::Swap)),
                (4, Expression::Manip(Word::Over)),
                (5, Expression::Arith(Operator::Add)),
                (6, Expression::Arith(Operator::Sub)),
                (7, Expression::Arith(Operator::Mult)),
                (8, Expression::Arith(Operator::Div)),
            ]
            .into_iter(),
        );
        Forth(Vec::new(), initial_defs, initial_names)
    }

    pub fn stack(&self) -> &[Value] {
        let Forth(stack, _, _) = self;
        stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if !is_valid_def(input) {
            return Err(Error::InvalidWord);
        }
        let inputs = split_on_def(input);
        let expr: std::result::Result<Vec<Expression>, Error> = inputs
            .iter()
            .enumerate()
            .filter(|(_, &s)| !s.is_empty())
            .fold(Ok(Vec::new()), |v_acc, (i, s)| {
                if i % 2 == 0 {
                    let words = s.split_whitespace();
                    let maybe_vec: std::result::Result<Vec<Expression>, Error> = words
                        .filter(|&s| !s.is_empty())
                        .map(|word| self.parse_expr(word))
                        .collect();
                    match maybe_vec {
                        Err(e) => Err(e),
                        Ok(v) => v_acc.map(|vacc| [vacc, v].concat()),
                    }
                } else {
                    let parseresult = self.parse_def(s);
                    match parseresult {
                        Err(e) => Err(e),
                        _ => v_acc,
                    }
                }
            });
        match expr {
            Err(e) => return Err(e),
            Ok(expr_vec) => {
                for exp in expr_vec.iter() {
                    let evalresult = self.eval_expr(exp);
                    if let Err(e) = evalresult {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn parse_expr(&self, input: &str) -> std::result::Result<Expression, Error> {
        let Forth(_, dict, names) = self;

        if let Ok(i) = input.parse::<i32>() {
            Ok(Expression::Val(i))
        } else {
            let lookup = match names.get(&input.to_lowercase()) {
                None => return Err(Error::UnknownWord),
                Some(&address) => match dict.get(address) {
                    None => return Err(Error::UnknownWord),

                    Some(_expr) => Expression::Address(address),
                },
            };
            Ok(lookup)
        }
    }

    fn parse_def(&mut self, input: &str) -> Result {
        let mut words = input.split_whitespace().filter(|&s| !s.is_empty());
        let new_word = match words.next() {
            None => return Err(Error::InvalidWord),
            Some(word) => {
                if let Ok(_i) = word.parse::<i32>() {
                    return Err(Error::InvalidWord);
                } else {
                    word.to_string()
                }
            }
        };

        let defs: std::result::Result<Vec<Expression>, Error> =
            words.map(|word| self.parse_expr(word)).collect();
        let unwrapped_defs = match defs {
            Err(e) => return Err(e),
            Ok(def) => def,
        };
        let Forth(_, dict, names) = self;
        let nextaddress = (names.len() + 1) as u64;
        names.insert(new_word.to_lowercase(), nextaddress);
        dict.insert(nextaddress, Expression::Func(unwrapped_defs));
        Ok(())
    }

    fn eval_expr(&mut self, expr: &Expression) -> Result {
        let Forth(stack, dict, _) = self;
        let cloned_dict = dict.clone();
        match expr {
            Expression::Val(a) => stack.push(*a),
            Expression::Def(_) => return Ok(()),
            Expression::Address(address) => match cloned_dict.get(*address) {
                None => return Err(Error::UnknownWord),

                Some(new_expr) => return self.eval_expr(new_expr),
            },
            Expression::Func(v) => return v.iter().try_for_each(|e| self.eval_expr(e)),

            Expression::Arith(op) => {
                let b = stack.pop();
                let a = stack.pop();
                if let (Some(a), Some(b)) = (a, b) {
                    match op {
                        Operator::Add => stack.push(a + b),
                        Operator::Mult => stack.push(a * b),
                        Operator::Sub => stack.push(a - b),
                        Operator::Div => {
                            if b == 0 {
                                return Err(Error::DivisionByZero);
                            } else {
                                stack.push(a / b);
                            }
                        }
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }

            Expression::Manip(word) => {
                let a = stack.pop();
                if let Some(a) = a {
                    match word {
                        Word::Drop => return Ok(()),
                        Word::Dup => {
                            stack.push(a);
                            stack.push(a)
                        }
                        Word::Swap => {
                            let b = stack.pop();
                            if let Some(b) = b {
                                stack.push(a);
                                stack.push(b);
                            } else {
                                return Err(Error::StackUnderflow);
                            }
                        }
                        Word::Over => {
                            let b = stack.pop();

                            if let Some(b) = b {
                                stack.push(b);
                                stack.push(a);
                                stack.push(b);
                            } else {
                                return Err(Error::StackUnderflow);
                            }
                        }
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
        }
        Ok(())
    }
}
pub fn split_on_def(input: &str) -> Vec<&str> {
    input
        .split(": ")
        .map(|substr| substr.split(" ;").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .concat()
}
pub fn is_valid_def(input: &str) -> bool {
    let mut defs = input.split(':');
    defs.next();
    defs.all(|s| s.contains(';'))
}
