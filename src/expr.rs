use crate::utils::{extract_whitespace, extract_tag, take_while};
struct Number(pub i32);

impl Number {
    fn parse(s: &str) -> Result<(&str, Self), &str> {
        let (taken, s) = take_while(s, |c| c.is_ascii_digit());
        
        taken.parse::<i32>().map(|n| (s, Number(n))).map_err(|_| "Expected an integer")
    }
}

enum Op {
    Add(),
    Sub(),
    Mult(),
    Div(),
}

impl Op {
    fn parse(s: &str) -> Result<(&str, Self), &str> {
        extract_tag(s, "+").map(|s| (s, Op::Add()))
            .or_else(|_| extract_tag(s, "-").map(|s| (s, Op::Sub())))
            .or_else(|_| extract_tag(s, "*").map(|s| (s, Op::Mult())))
            .or_else(|_| extract_tag(s, "/").map(|s| (s, Op::Div())))
    }
}

pub struct Expr {
    lhs: Number,
    rhs: Number,
    op: Op,
}

impl Expr{
    pub fn parse(s: &str) -> Result<(&str, Expr), &str>{
        let s = extract_whitespace(s).0;
        let (s, lhs) = Number::parse(s)?;

        let s = extract_whitespace(s).0;
        let (s, op) = Op::parse(s)?;

        let s = extract_whitespace(s).0;
        let (s, rhs) = Number::parse(s)?;


        Ok((s, Self {
            lhs,
            op,
            rhs
        }))
    }

    pub fn eval(&self) -> Number{
        match self.op {
            Op::Add() => self.lhs + self.rhs,
            _ => todo!(),
        }
    }

}
