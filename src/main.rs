#![feature(plugin)]
#![plugin(oak)]
#![allow(non_snake_case)]

extern crate oak_runtime;
use oak_runtime::*;

grammar! cool {
    #![show_api]

    spacing = [" \n\r\t"]* -> (^)
    spc = spacing

    ID = !digit !caps ["0-9A-Za-z_"]+ > to_string
    TYPE = !digit &caps ["0-9A-Za-z_"]+ > to_string

    integer = digit+ spacing > to_number
    digit = ["0-9"]
    caps = ["A-Z"]

    add_op = "+" spacing
    sub_op = "-" spacing
    mul_op = "*" spacing
    div_op = "/" spacing
    lparen = "(" spacing
    rparen = ")" spacing

    program = spc (class ";")+

    class = "class" spc TYPE (spc "inherits" spc TYPE)? spc "{" spc (feature ";")* "}"

    feature
        = ID spc "(" spc (formal (spc "," spc formal spc)*)? spc ")" ":" TYPE "{" expr "}" spc -> (^)
        / ID spc ":" spc TYPE ("<-" expr)? spc -> (^)

    formal = ID ":" TYPE

    expr
        = term (term_op term)*                     > fold_left

    term
        = (factor factor_op)* factor               > fold_right

    term_op
        = add_op > add_bin_op
        / sub_op > sub_bin_op

    factor
        = ID "<-" expr                             > assign_expr
        / "if" expr "then" expr "else" expr "fi"   > if_expr
        / "while" expr "loop" expr "pool"          > while_expr
        / "new" TYPE                               > new_expr
        / "isvoid" expr                            > isvoid_expr
        / "~" expr                                 > neg_expr
        / "not" expr                               > not_expr
        / integer                                  > number_expr
        / "true"                                   > true_expr
        / "false"                                  > false_expr
        / lparen expr rparen

    factor_op
        = mul_op > mul_bin_op
        / div_op > div_bin_op

    use std::str::FromStr;
    use self::Expression::*;
    use self::BinOp::*;

    pub type PExpr = Box<Expression>;

    #[derive(Debug)]
    pub enum Expression {
        Assign(String, PExpr),
        If(PExpr, PExpr, PExpr),
        While(PExpr, PExpr),
        New(String),
        IsVoid(PExpr),
        Neg(PExpr),
        Not(PExpr),
        Variable(String),
        Number(u32),
        Boolean(bool),
        BinaryExpr(BinOp, PExpr, PExpr),
        // LetIn(String, PExpr, PExpr)
    }

    #[derive(Debug)]
    pub enum BinOp {
        Add, Sub, Mul, Div
    }

    fn to_string(raw_text: Vec<char>) -> String {
        raw_text.into_iter().collect()
    }

    fn to_number(raw_text: Vec<char>) -> u32 {
        u32::from_str(&*to_string(raw_text)).unwrap()
    }

    fn number_expr(value: u32) -> PExpr {
        Box::new(Number(value))
    }

    fn assign_expr(id: String, expr: PExpr) -> PExpr {
        Box::new(Assign(id, expr))
    }

    fn if_expr(cond: PExpr, then: PExpr, else_: PExpr) -> PExpr {
        Box::new(If(cond, then, else_))
    }

    fn while_expr(cond: PExpr, body: PExpr) -> PExpr {
        Box::new(While(cond, body))
    }

    fn new_expr(id: String) -> PExpr {
        Box::new(New(id))
    }

    fn isvoid_expr(expr: PExpr) -> PExpr {
        Box::new(IsVoid(expr))
    }

    fn neg_expr(expr: PExpr) -> PExpr {
        Box::new(Neg(expr))
    }

    fn not_expr(expr: PExpr) -> PExpr {
        Box::new(Not(expr))
    }

    fn true_expr() -> PExpr {
        Box::new(Boolean(true))
    }

    fn false_expr() -> PExpr {
        Box::new(Boolean(false))
    }

    // fn method_expr() -> PFeature {
    //     Box::new(Feature{ One })
    // }

    // fn var_expr() -> PFeature {
    //     Box::new(Feature{ One })
    // }

    fn fold_left(head: PExpr, rest: Vec<(BinOp, PExpr)>) -> PExpr {
        rest.into_iter().fold(head,
            |accu, (op, expr)| Box::new(BinaryExpr(op, accu, expr)))
    }

    fn fold_right(front: Vec<(PExpr, BinOp)>, last: PExpr) -> PExpr {
        front.into_iter().rev().fold(last,
            |accu, (expr, op)| Box::new(BinaryExpr(op, expr, accu)))
    }

    fn add_bin_op() -> BinOp { Add }
    fn sub_bin_op() -> BinOp { Sub }
    fn mul_bin_op() -> BinOp { Mul }
    fn div_bin_op() -> BinOp { Div }
}

#[test]
fn parse_class_decl() {
    let test = "class Name {}";
    let state = cool::recognize_class(test.into_state());
    assert!(state.is_successful());
}

#[test]
fn parse_class_inheritance_decl() {
    let test = "class Name inherits Base {}";
    let state = cool::recognize_class(test.into_state());
    assert!(state.is_successful());
}

#[test]
fn parse_simple_expr() {
    let test = "1 + 1";
    let state = cool::recognize_expr(test.into_state());
    assert!(state.is_successful());
}

#[test]
fn parse_complex_expr() {
    let test = "1 + (2 * 3 + 4)";
    let state = cool::recognize_expr(test.into_state());
    assert!(state.is_successful());
}

#[test]
fn parse_advanced_expr() {
    let test = "1 + if true then 1 else 0 fi";
    let state = cool::recognize_expr(test.into_state());
    assert!(state.is_successful());
}

#[test]
fn parse_advanced_expr2() {
    let test = "val <- not if true then 1 else 0 fi";
    let state = cool::recognize_expr(test.into_state());
    assert!(state.is_successful());
}

fn main() {
    println!("Hello, world!");
}
