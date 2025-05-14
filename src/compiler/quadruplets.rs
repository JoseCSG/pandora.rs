use crate::compiler::semantic_cube::Operator;
use crate::utils::queue::Queue;

#[derive(Debug, Clone)]
pub struct Quadruplet {
    operator: i32,
    arg1: i32,
    arg2: Option<i32>,
    result: Option<i32>,
}

pub enum QuadOperator {
    Goto,
    GotoV,
    GotoF,
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    NotEqual,
    Print,
}

/*
1 - GOTO
2 - GOTOV
3 - GOTOF
4 - =
5 - +
6 - -
7 - *
8 - /
9 - >
10 - <
11 - !=
12 - print
*/

impl Quadruplet {
    pub fn new(operator: i32, arg1: i32, arg2: Option<i32>, result: Option<i32>) -> Self {
        Quadruplet {
            operator,
            arg1,
            arg2,
            result,
        }
    }
}

#[derive(Debug)]
pub struct QuadrupletList {
    quadruplets: Queue<Quadruplet>,
}

impl QuadrupletList {
    pub fn new() -> Self {
        QuadrupletList {
            quadruplets: Queue::new(),
        }
    }
    pub fn push(&mut self, quadruplet: Quadruplet) {
        self.quadruplets.push(quadruplet);
    }
    pub fn pop(&mut self) {
        self.quadruplets.pop();
    }
}

pub fn convert_semantic_op_to_quad_op(op: Operator) -> QuadOperator {
    match op {
        Operator::Add => QuadOperator::Add,
        Operator::Subtract => QuadOperator::Subtract,
        Operator::Multiply => QuadOperator::Multiply,
        Operator::Divide => QuadOperator::Divide,
        Operator::GreaterThan => QuadOperator::GreaterThan,
        Operator::LessThan => QuadOperator::LessThan,
        Operator::NotEqual => QuadOperator::NotEqual,
        _ => panic!("Invalid operator"),
    }
}
