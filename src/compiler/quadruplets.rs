use crate::compiler::semantic_cube::Operator;
use crate::utils::queue::Queue;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Quadruplet {
    pub operator: i32,
    pub arg1: i32,
    pub arg2: Option<i32>,
    pub result: Option<i32>,
}

impl Display for Quadruplet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let op_str = match self.operator {
            1 => "GOTO",
            2 => "GOTOV",
            3 => "GOTOF",
            4 => "ASSIGN",
            5 => "SUM",
            6 => "MINUS",
            7 => "TIMES",
            8 => "DIV",
            9 => "GT",
            10 => "LT",
            11 => "NE",
            12 => "PRINT",
            13 => "MEMORY",
            14 => "PARAM",
            15 => "GOSUB",
            16 => "ENDFUNC",
            17 => "ENDPROGRAM",
            _ => "INVALID QUAD OP",
        };

        let arg2_str = match self.arg2 {
            Some(val) => val.to_string(),
            None => "_".to_string(),
        };

        let result_str = match self.result {
            Some(val) => val.to_string(),
            None => "_".to_string(),
        };

        write!(
            f,
            "({: <6} {: <5} {: <5} {: <5})",
            op_str, self.arg1, arg2_str, result_str
        )
    }
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
    Memory,
    GoSub,
    Param,
    EndFunc,
    EndProgram,
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
13 - end_program
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
    pub fn len(&self) -> i32 {
        self.quadruplets.len() as i32
    }

    pub fn get(&self, index: i32) -> Option<&Quadruplet> {
        self.quadruplets.get(index as usize)
    }

    pub fn set(&mut self, index: i32, value: Quadruplet) {
        self.quadruplets.set(index as usize, value);
    }

    pub fn print_elements(&self) {
        self.quadruplets.print_elements();
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

pub fn convert_quad_op_to_code(op: i32) -> QuadOperator {
    match op {
        1 => QuadOperator::Goto,
        2 => QuadOperator::GotoV,
        3 => QuadOperator::GotoF,
        4 => QuadOperator::Assign,
        5 => QuadOperator::Add,
        6 => QuadOperator::Subtract,
        7 => QuadOperator::Multiply,
        8 => QuadOperator::Divide,
        9 => QuadOperator::GreaterThan,
        10 => QuadOperator::LessThan,
        11 => QuadOperator::NotEqual,
        12 => QuadOperator::Print,
        13 => QuadOperator::Memory,
        14 => QuadOperator::Param,
        15 => QuadOperator::GoSub,
        16 => QuadOperator::EndFunc,
        17 => QuadOperator::EndProgram,
        _ => panic!("Invalid operator"),
    }
}
