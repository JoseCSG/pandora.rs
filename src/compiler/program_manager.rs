use crate::compiler::quadruplets::{Quadruplet, QuadrupletList};
use crate::compiler::semantic_cube::{CuboSemantico, Operator, Type};
use crate::compiler::semantic_tables::FunctionTable;
use crate::compiler::value_table::ValueTable;
use crate::Stack;
use core::panic;
use std::collections::HashMap;

use super::quadruplets::{convert_quad_op_to_code, QuadOperator};
use super::value_table::{Memory, Value};

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
    pub curr_function: Stack<String>,
    pub memory_stack: Stack<Memory>,
    pub function_ids: HashMap<i32, String>,
    position_before_fcall: Stack<i32>,
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
            curr_function: Stack::new(),
            memory_stack: Stack::new(),
            function_ids: HashMap::new(),
            position_before_fcall: Stack::new(),
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
            QuadOperator::Memory => 13,
            QuadOperator::Param => 14,
            QuadOperator::GoSub => 15,
            QuadOperator::EndFunc => 16,
            QuadOperator::EndProgram => 17,
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

    pub fn run_program(&mut self) {
        if self.quadruplets.len() == 0 {
            panic!("There's no quadruplets")
        }

        self.instruction_pointer = 0;
        while self.instruction_pointer < self.quadruplets.len() as i32 {
            let quad = self.quadruplets.get(self.instruction_pointer).unwrap();
            let op_code: QuadOperator = convert_quad_op_to_code(quad.operator);

            match op_code {
                QuadOperator::Add => {
                    let arg1_type = self.value_table.get_var_type(quad.arg1);
                    let arg2_type = self.value_table.get_var_type(quad.arg2.unwrap());

                    if arg1_type == Type::Int && arg2_type == Type::Int {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 + arg2;
                        self.value_table.set_int(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Float {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 + arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Int && arg2_type == Type::Float {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 as f32 + arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Int {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 + arg2 as f32;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    }
                }
                QuadOperator::Subtract => {
                    let arg1_type = self.value_table.get_var_type(quad.arg1);
                    let arg2_type = self.value_table.get_var_type(quad.arg2.unwrap());

                    if arg1_type == Type::Int && arg2_type == Type::Int {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 - arg2;
                        self.value_table.set_int(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Float {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 - arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Int && arg2_type == Type::Float {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 as f32 - arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Int {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 - arg2 as f32;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    }
                }
                QuadOperator::Multiply => {
                    let arg1_type = self.value_table.get_var_type(quad.arg1);
                    let arg2_type = self.value_table.get_var_type(quad.arg2.unwrap());

                    if arg1_type == Type::Int && arg2_type == Type::Int {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 * arg2;
                        self.value_table.set_int(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Float {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 * arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Int && arg2_type == Type::Float {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 as f32 * arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Int {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 * arg2 as f32;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    }
                }
                QuadOperator::Divide => {
                    let arg1_type = self.value_table.get_var_type(quad.arg1);
                    let arg2_type = self.value_table.get_var_type(quad.arg2.unwrap());

                    if arg1_type == Type::Int && arg2_type == Type::Int {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 / arg2;
                        self.value_table.set_int(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Float {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 / arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Int && arg2_type == Type::Float {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 as f32 / arg2;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    } else if arg1_type == Type::Float && arg2_type == Type::Int {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 / arg2 as f32;
                        self.value_table.set_float(
                            quad.result.unwrap(),
                            result,
                            self.memory_stack.top_mut(),
                        );
                    }
                }
                QuadOperator::GreaterThan => {
                    let arg1_type = self.value_table.get_var_type(quad.arg1);
                    let arg2_type = self.value_table.get_var_type(quad.arg2.unwrap());

                    if arg1_type == Type::Int && arg2_type == Type::Int {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 > arg2;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    } else if arg1_type == Type::Float && arg2_type == Type::Float {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 > arg2;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    } else if arg1_type == Type::Int && arg2_type == Type::Float {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 as f32 > arg2;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    } else if arg1_type == Type::Float && arg2_type == Type::Int {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 > arg2 as f32;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    }
                }
                QuadOperator::LessThan => {
                    let arg1_type = self.value_table.get_var_type(quad.arg1);
                    let arg2_type = self.value_table.get_var_type(quad.arg2.unwrap());

                    if arg1_type == Type::Int && arg2_type == Type::Int {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 < arg2;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    } else if arg1_type == Type::Float && arg2_type == Type::Float {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 < arg2;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    } else if arg1_type == Type::Int && arg2_type == Type::Float {
                        let arg1 = self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = (arg1 as f32) < arg2;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    } else if arg1_type == Type::Float && arg2_type == Type::Int {
                        let arg1 = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        let arg2 = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        let result = arg1 < arg2 as f32;
                        self.value_table.set_bool(quad.result.unwrap(), result);
                    }
                }
                QuadOperator::GotoF => {
                    let var_value = self.value_table.get_bool(quad.arg1);
                    if var_value == false {
                        self.instruction_pointer = quad.arg2.unwrap();
                        continue;
                    }
                }
                QuadOperator::Goto => {
                    self.instruction_pointer = quad.arg1;
                    continue;
                }
                QuadOperator::Assign => {
                    let var_type = self.value_table.get_var_type(quad.arg1);
                    let result_type = self.value_table.get_var_type(quad.arg2.unwrap());

                    if var_type == Type::Int && result_type == Type::Int {
                        let value = self
                            .value_table
                            .get_int(quad.arg2.unwrap(), self.memory_stack.top());
                        self.value_table
                            .set_int(quad.arg1, value, self.memory_stack.top_mut());
                    } else if var_type == Type::Float && result_type == Type::Float {
                        let value = self
                            .value_table
                            .get_float(quad.arg2.unwrap(), self.memory_stack.top());
                        self.value_table
                            .set_float(quad.arg1, value, self.memory_stack.top_mut());
                    }
                }
                QuadOperator::Print => {
                    let var_type = self.value_table.get_var_type(quad.arg1);
                    let next_quad_op = convert_quad_op_to_code(
                        self.quadruplets
                            .get(self.instruction_pointer + 1)
                            .unwrap()
                            .operator,
                    );

                    match var_type {
                        Type::Int => {
                            let value =
                                self.value_table.get_int(quad.arg1, self.memory_stack.top());
                            match next_quad_op {
                                QuadOperator::Print => print!("{}", value),
                                _ => println!("{}", value),
                            }
                        }
                        Type::Float => {
                            let value = self
                                .value_table
                                .get_float(quad.arg1, self.memory_stack.top());
                            match next_quad_op {
                                QuadOperator::Print => print!("{}", value),
                                _ => println!("{}", value),
                            }
                        }
                        Type::String => {
                            let value = self.value_table.get_string(quad.arg1);
                            match next_quad_op {
                                QuadOperator::Print => print!("{}", value),
                                _ => println!("{}", value),
                            }
                        }
                        _ => {
                            panic!(
                                "Invalid Type for print statement at quad {}",
                                self.instruction_pointer
                            );
                        }
                    }
                }
                QuadOperator::Memory => {
                    let function_name = self.function_ids.get(&quad.arg1).unwrap();
                    let function_info =
                        &self.tabla_funciones.get(function_name).unwrap().vars_amount;
                    let local_int_amount = function_info[0];
                    let local_float_amount = function_info[1];

                    let curr_memory = Memory {
                        values: vec![
                            vec![vec![Value::Int(0); local_int_amount as usize]],
                            vec![vec![Value::Float(0.0); local_float_amount as usize]],
                        ],
                    };
                    self.memory_stack.push(curr_memory);
                }
                QuadOperator::Param => {
                    if quad.arg2.unwrap() >= 5000 {
                        let var_value = self
                            .value_table
                            .get_float(quad.arg1, self.memory_stack.top());
                        self.value_table.set_float(
                            quad.arg2.unwrap(),
                            var_value,
                            self.memory_stack.top_mut(),
                        );
                    } else {
                        let var_value =
                            self.value_table.get_int(quad.arg1, self.memory_stack.top());
                        self.value_table.set_int(
                            quad.arg2.unwrap(),
                            var_value,
                            self.memory_stack.top_mut(),
                        );
                    }
                }
                QuadOperator::GoSub => {
                    let function_name = self.function_ids.get(&quad.arg1).unwrap();
                    let function_start_address = &self
                        .tabla_funciones
                        .get(function_name)
                        .unwrap()
                        .start_address;
                    self.position_before_fcall
                        .push(self.instruction_pointer + 1);
                    self.instruction_pointer = *function_start_address;
                    continue;
                }
                QuadOperator::EndFunc => {
                    self.memory_stack.pop();
                    self.instruction_pointer = self.position_before_fcall.pop().unwrap();
                    continue;
                }
                QuadOperator::EndProgram => {
                    println!("System exited with a value 0");
                }
                _ => {
                    panic!("Invalid quad operator");
                }
            }
            self.instruction_pointer += 1
        }
    }
}
