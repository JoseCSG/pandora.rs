use crate::compiler::semantic_cube::Type;
use std::collections::HashMap;

pub const START_INT_VALUES: i32 = 1000;
pub const START_FLOAT_VALUES: i32 = 4000;
const START_BOOL_VALUES: i32 = 8000;
const START_CONST_INT_VALUES: i32 = 9000;
const START_CONST_FLOAT_VALUES: i32 = 10000;
const START_CONST_STRING_VALUES: i32 = 11000;

pub const LOCAL_VALUES: i32 = 1000;
const TEMP_VALUES: i32 = 2000;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
enum ConstValue {
    Int(i32),
    Float(f32),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]

pub struct Memory {
    pub values: Vec<Vec<Vec<Value>>>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            values: vec![vec![vec![], vec![]], vec![vec![], vec![]], vec![vec![]]],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueTable {
    pub counters: HashMap<String, i32>,
    var_values: Memory,
    const_values: Vec<Vec<ConstValue>>,
}

impl ValueTable {
    pub fn new() -> Self {
        ValueTable {
            counters: HashMap::new(),
            var_values: Memory::new(),
            const_values: vec![vec![], vec![], vec![]],
        }
    }

    pub fn insert_integer(&mut self, value: i32, scope: &str) -> i32 {
        let key = format!("int_{}", scope);
        let offset = match scope {
            "global" => START_INT_VALUES,
            "local" => START_INT_VALUES + LOCAL_VALUES,
            "temp" => START_INT_VALUES + TEMP_VALUES,
            _ => panic!("Invalid scope"),
        };

        let array_index = match scope {
            "global" => 0,
            "local" => 0,
            "temp" => 1,
            _ => panic!("Invalid lifetime"),
        };

        let address = match scope {
            "global" => {
                self.var_values.values[0][array_index].push(Value::Int(value));
                *self.counters.get(&key).unwrap_or(&0) as i32 + offset
            }
            "temp" => {
                self.var_values.values[0][array_index].push(Value::Int(value));
                *self.counters.get(&key).unwrap_or(&0) as i32 + offset
            }
            "local" => *self.counters.get(&key).unwrap_or(&0) as i32 + offset,
            _ => {
                panic!("Invalid scope");
            }
        };

        *self.counters.entry(key.clone()).or_insert(0) += 1;

        address
    }

    pub fn insert_float(&mut self, value: f32, scope: &str) -> i32 {
        let key = format!("float_{}", scope);
        let offset = match scope {
            "global" => START_FLOAT_VALUES,
            "local" => START_FLOAT_VALUES + LOCAL_VALUES,
            "temp" => START_FLOAT_VALUES + TEMP_VALUES,
            _ => panic!("Invalid scope"),
        };

        let array_index = match scope {
            "global" => 0,
            "local" => 0,
            "temp" => 1,
            _ => panic!("Invalid scope"),
        };

        let address = match scope {
            "global" => {
                self.var_values.values[1][array_index].push(Value::Float(value));
                *self.counters.get(&key).unwrap_or(&0) as i32 + offset
            }
            "temp" => {
                self.var_values.values[1][array_index].push(Value::Float(value));
                *self.counters.get(&key).unwrap_or(&0) as i32 + offset
            }
            "local" => *self.counters.get(&key).unwrap_or(&0) as i32 + offset,
            _ => {
                panic!("Invalid scope");
            }
        };

        *self.counters.entry(key.clone()).or_insert(0) += 1;

        address
    }

    pub fn clear_local_vars(&mut self) {
        *self.counters.entry("int_local".to_string()).or_insert(0) = 0;
        *self.counters.entry("float_local".to_string()).or_insert(0) = 0;
    }

    pub fn insert_bool(&mut self, value: bool) -> i32 {
        let address = self.var_values.values[2][0].len() as i32 + START_BOOL_VALUES;
        self.var_values.values[2][0].push(Value::Bool(value));
        address
    }

    pub fn insert_cte_int(&mut self, value: i32) -> i32 {
        let address = self.const_values[0].len() as i32 + START_CONST_INT_VALUES;
        self.const_values[0].push(ConstValue::Int(value));
        address
    }

    pub fn insert_cte_float(&mut self, value: f32) -> i32 {
        let address = self.const_values[1].len() as i32 + START_CONST_FLOAT_VALUES;
        self.const_values[1].push(ConstValue::Float(value));
        address
    }

    pub fn insert_cte_string(&mut self, value: String) -> i32 {
        let address = self.const_values[2].len() as i32 + START_CONST_STRING_VALUES;
        self.const_values[2].push(ConstValue::String(value));
        address
    }

    pub fn get_string(&mut self, address: i32) -> String {
        let position = address - START_CONST_STRING_VALUES;
        match &self.const_values[2][position as usize] {
            ConstValue::String(value) => value.clone(),
            _ => panic!("Invalid address for string value"),
        }
    }

    pub fn get_int(&self, address: i32, memory: Option<&Memory>) -> i32 {
        let mut array_index = 0;
        let mut position = address - START_INT_VALUES;
        let mut int_type = "var";

        if address >= START_INT_VALUES + LOCAL_VALUES && address < START_INT_VALUES + TEMP_VALUES {
            position = position - LOCAL_VALUES;
            let value = match memory {
                Some(mem) => match mem.values[0][0][position as usize] {
                    Value::Int(val) => val,
                    _ => panic!("Invalid local address: {}", address),
                },
                None => 0,
            };
            return value;
        } else if address >= START_INT_VALUES + TEMP_VALUES && address < START_FLOAT_VALUES {
            array_index = 1;
            position = position - TEMP_VALUES;
        } else if address >= START_CONST_INT_VALUES && address < START_CONST_FLOAT_VALUES {
            int_type = "const";
            position = address - START_CONST_INT_VALUES;
        }

        match int_type {
            "var" => match self.var_values.values[0][array_index][position as usize] {
                Value::Int(value) => value,
                _ => panic!("Invalid address"),
            },
            "const" => match self.const_values[0][position as usize] {
                ConstValue::Int(value) => value,
                _ => panic!("Invalid address"),
            },
            _ => panic!("Invalid address"),
        }
    }

    pub fn set_int(&mut self, address: i32, value: i32, memory: Option<&mut Memory>) {
        let mut array_index = 0;
        let mut position = address - START_INT_VALUES;
        if address >= START_INT_VALUES + LOCAL_VALUES && address < START_INT_VALUES + TEMP_VALUES {
            array_index = 0;
            position = position - LOCAL_VALUES;
            match memory {
                Some(val) => {
                    val.values[0][array_index][position as usize] = Value::Int(value);
                }
                None => {}
            }
            return;
        } else if address >= START_INT_VALUES + TEMP_VALUES {
            array_index = 1;
            position = position - TEMP_VALUES;
        }
        self.var_values.values[0][array_index][position as usize] = Value::Int(value);
    }

    pub fn get_float(&self, address: i32, memory: Option<&Memory>) -> f32 {
        let mut array_index = 0;
        let mut position = address - START_FLOAT_VALUES;
        let mut float_type = "var";
        if address >= START_FLOAT_VALUES + LOCAL_VALUES
            && address < START_FLOAT_VALUES + TEMP_VALUES
        {
            array_index = 0;
            position = position - LOCAL_VALUES;
            let value = match memory {
                Some(mem) => match mem.values[1][array_index][position as usize] {
                    Value::Float(val) => val,
                    _ => panic!("Invalid local address: {}", address),
                },
                None => 0 as f32,
            };
            return value;
        } else if address >= START_FLOAT_VALUES + TEMP_VALUES && address < START_CONST_FLOAT_VALUES
        {
            array_index = 1;
            position = position - TEMP_VALUES;
        } else if address >= START_CONST_FLOAT_VALUES {
            float_type = "const";
            position = address - START_CONST_FLOAT_VALUES;
        }

        match float_type {
            "var" => match self.var_values.values[1][array_index][position as usize] {
                Value::Float(value) => value,
                _ => panic!("Invalid address"),
            },
            "const" => match self.const_values[1][position as usize] {
                ConstValue::Float(value) => value,
                _ => panic!("Invalid address"),
            },
            _ => panic!("Invalid address"),
        }
    }

    pub fn set_float(&mut self, address: i32, value: f32, memory: Option<&mut Memory>) {
        let mut array_index = 0;
        let mut position = address - START_FLOAT_VALUES;
        if address >= START_FLOAT_VALUES + LOCAL_VALUES
            && address < START_FLOAT_VALUES + TEMP_VALUES
        {
            array_index = 0;
            position = position - LOCAL_VALUES;
            match memory {
                Some(val) => {
                    val.values[1][array_index][position as usize] = Value::Float(value);
                }
                None => {}
            }
            return;
        } else if address >= START_FLOAT_VALUES + TEMP_VALUES {
            array_index = 1;
            position = position - TEMP_VALUES;
        }
        self.var_values.values[1][array_index][position as usize] = Value::Float(value);
    }

    pub fn get_bool(&self, address: i32) -> bool {
        match self.var_values.values[2][0][(address - START_BOOL_VALUES) as usize] {
            Value::Bool(value) => value,
            _ => panic!("Invalid address"),
        }
    }

    pub fn set_bool(&mut self, address: i32, value: bool) {
        self.var_values.values[2][0][(address - START_BOOL_VALUES) as usize] = Value::Bool(value);
    }

    pub fn get_var_type(&self, address: i32) -> Type {
        if address >= START_INT_VALUES && address < START_FLOAT_VALUES {
            Type::Int
        } else if address >= START_FLOAT_VALUES && address < START_BOOL_VALUES {
            Type::Float
        } else if address >= START_BOOL_VALUES && address < START_CONST_INT_VALUES {
            Type::Bool
        } else if address >= START_CONST_INT_VALUES && address < START_CONST_FLOAT_VALUES {
            Type::Int
        } else if address >= START_CONST_FLOAT_VALUES && address < START_CONST_STRING_VALUES {
            Type::Float
        } else if address >= START_CONST_STRING_VALUES {
            Type::String
        } else {
            Type::Error
        }
    }
}
