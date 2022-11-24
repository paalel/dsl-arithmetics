use crate::Equation::{Expression, Value};
use crate::Operator::{Minus, Multiplication, Plus};
use std::env;

const OPERATORS: [char; 4] = ['+', '-', '*', 'x'];
const SYMBOLS: [char; 12] = ['(', ')', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

enum Operator {
    Plus,
    Minus,
    Multiplication,
}
fn operator_from_char(c: &char) -> Operator {
    match c {
        '+' => Plus,
        '-' => Minus,
        '*' | 'x' => Multiplication,
        _ => panic!("No operator match!"),
    }
}

enum Equation {
    Expression {
        op: Operator,
        left: Box<Equation>,
        right: Box<Equation>,
    },
    Value(i32),
}
impl Equation {
    fn solve(&self) -> i32 {
        match self {
            Expression { op, left, right } => match op {
                Plus => left.solve() + right.solve(),
                Minus => left.solve() - right.solve(),
                Multiplication => left.solve() * right.solve(),
            },
            Value(v) => *v,
        }
    }
}
fn build(mut problem: Vec<char>) -> Equation {
    // Build equation structure. An Equation is at tree of expressions where an expression is
    // either a binary operator pointing to two nested expressions or a value end node.
    let length = problem.len();
    if problem[0] == '(' && problem[length - 1] == ')' {
        problem = problem[1..length - 1].to_vec();
    }

    let mut i = 0;
    while i < length {
        // Split binary opertor
        if OPERATORS.contains(&problem[i]) {
            return Expression {
                op: operator_from_char(&problem[i]),
                left: Box::new(build(problem[..i].to_vec())),
                right: Box::new(build(problem[(i + 1)..].to_vec())),
            };
        }
        // Group parenthesis to make sure it is correctly identified as input to a binary operator.
        if problem[i] == '(' {
            let mut j = i;
            while j < length {
                if problem[j] == ')' {
                    i = j;
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
    let s: String = problem.into_iter().collect();
    let v = s.parse().unwrap();
    return Value(v);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = &args[1];

    // TODO:
    // - Verify that the input is correct.
    // - Consider operator order? Instead of traversing the string the array it could also make
    // sense to split by symbols.
    // - Add tests

    // Check for allowed symbols
    if !problem
        .chars()
        .all(|c| OPERATORS.contains(&c) || SYMBOLS.contains(&c))
    {
        panic!("The only supported operators are: {:?}", OPERATORS)
    }

    // Transform into a Equation object
    let equation = build(problem.chars().collect());

    // Solve and print
    println!("{}={:?}", problem, equation.solve());
}
