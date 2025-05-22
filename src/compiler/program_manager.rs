use crate::compiler::quadruplets::{Quadruplet, QuadrupletList};
use crate::compiler::semantic_cube::{CuboSemantico, Operator, Type};
use crate::compiler::semantic_tables::FunctionTable;
use crate::compiler::value_table::ValueTable;
use core::panic;
use std::collections::HashMap;

use super::quadruplets::{convert_quad_op_to_code, QuadOperator};

pub struct ProgramManager {
    pub cubo: CuboSemantico,
    pub tabla_funciones: FunctionTable,
    pub quadruplets: QuadrupletList,
    pub value_table: ValueTable,
    pub operand_stack: Vec<i32>,
    pub operator_stack: Vec<Operator>,
    pub polish_vector: Vec<String>,
    pub instruction_pointer: i32,
    pub jumps_stack: Vec<i32>,
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
            instruction_pointer: 0,
            jumps_stack: Vec::new(),
        }
    }

    pub fn new_temp(&mut self, var_type: Type) -> i32 {
        match var_type {
            Type::Int => self.value_table.insert_integer(0, "temp"),
            Type::Float => self.value_table.insert_float(0.0, "temp"),
            Type::Bool => self.value_table.insert_bool(false),
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
            QuadOperator::EndProgram => 13,
        };

        let quad = Quadruplet::new(op_code, arg1, arg2, result);
        self.quadruplets.push(quad);
        self.instruction_pointer = self.instruction_pointer + 1;
    }

    pub fn fill_quad(&mut self, index: i32, result: i32) {
        if let Some(quad) = self.quadruplets.get(index) {
            let quad_op_code = convert_quad_op_to_code(quad.operator);
            match quad_op_code {
                QuadOperator::Goto => {
                    self.quadruplets
                        .set(index, Quadruplet::new(quad.operator, result, None, None));
                }
                QuadOperator::GotoF => {
                    self.quadruplets.set(
                        index,
                        Quadruplet::new(quad.operator, quad.arg1, Some(result), None),
                    );
                }
                _ => {
                    panic!("Invalid quad operator for fill_quad");
                }
            }
        } else {
            panic!("Quadruplet not found at index {}", index);
        }
    }
}
