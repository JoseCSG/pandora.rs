use crate::compiler::quadruplets::{Quadruplet, QuadrupletList};
use crate::compiler::semantic_cube::{CuboSemantico, Operator, Type};
use crate::compiler::semantic_tables::FunctionTable;
use crate::compiler::value_table::ValueTable;
use std::collections::HashMap;

use super::quadruplets::QuadOperator;

pub struct ProgramManager {
    pub cubo: CuboSemantico,
    pub tabla_funciones: FunctionTable,
    pub quadruplets: QuadrupletList,
    pub value_table: ValueTable,
    pub operand_stack: Vec<i32>,
    pub operator_stack: Vec<Operator>,
    pub polish_vector: Vec<String>,
}

impl ProgramManager {
    pub fn new() -> Self {
        ProgramManager {
            cubo: CuboSemantico::new(),
            tabla_funciones: HashMap::new(),
            quadruplets: QuadrupletList::new(),
            value_table: ValueTable::new(),
            operand_stack: Vec::new(),
            operator_stack: Vec::new(),
            polish_vector: Vec::new(),
        }
    }

    pub fn new_temp(&mut self, var_type: Type) -> i32 {
        match var_type {
            Type::Int => self.value_table.insert_integer(0, "temp"),
            Type::Float => self.value_table.insert_float(0.0, "temp"),
            _ => panic!("Invalid type"),
        }
    }

    pub fn create_quad(
        &mut self,
        operator: QuadOperator,
        arg1: i32,
        arg2: Option<i32>,
        result: Option<i32>,
    ) {
        let op_code = match operator {
            QuadOperator::Goto => 1,
            QuadOperator::GotoV => 2,
            QuadOperator::GotoF => 3,
            QuadOperator::Assign => 4,
            QuadOperator::Add => 5,
            QuadOperator::Subtract => 6,
            QuadOperator::Multiply => 7,
            QuadOperator::Divide => 8,
            QuadOperator::GreaterThan => 9,
            QuadOperator::LessThan => 10,
            QuadOperator::NotEqual => 11,
            QuadOperator::Print => 12,
        };

        let quad = Quadruplet::new(op_code, arg1, arg2, result);
        self.quadruplets.push(quad);
    }
}
