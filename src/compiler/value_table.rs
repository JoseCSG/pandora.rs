use crate::compiler::semantic_cube::Type;
use std::collections::HashMap;

const START_INT_VALUES: i32 = 1000;
const START_FLOAT_VALUES: i32 = 4000;
const START_BOOL_VALUES: i32 = 8000;
const START_CONST_INT_VALUES: i32 = 9000;
const START_CONST_FLOAT_VALUES: i32 = 10000;
const START_CONST_STRING_VALUES: i32 = 11000;

const LOCAL_VALUES: i32 = 1000;
const TEMP_VALUES: i32 = 2000;

#[derive(Debug, Clone, PartialEq)]
enum Value {
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
pub struct ValueTable {
    counters: HashMap<String, i32>,
    var_values: Vec<Vec<Vec<Value>>>,
    const_values: Vec<Vec<ConstValue>>,
}

impl ValueTable {
    pub fn new() -> Self {
        ValueTable {
            counters: HashMap::new(),
            var_values: vec![
                vec![vec![], vec![], vec![]],
                vec![vec![], vec![], vec![]],
                vec![vec![]],
            ],
            const_values: vec![vec![], vec![], vec![]],
        }
    }

    pub fn insert_integer(&mut self, value: i32, lifetime: &str) -> i32 {
        let key = format!("int_{}", lifetime);
        let offset = match lifetime {
            "global" => START_INT_VALUES,
            "local" => START_INT_VALUES + LOCAL_VALUES,
            "temp" => START_INT_VALUES + TEMP_VALUES,
            _ => panic!("Invalid lifetime"),
        };

        let array_index = match lifetime {
            "global" => 0,
            "local" => 1,
            "temp" => 2,
            _ => panic!("Invalid lifetime"),
        };

        let address = self.var_values[0][array_index].len() as i32 + offset;
        self.var_values[0][array_index].push(Value::Int(value));
        *self.counters.entry(key).or_insert(0) += 1;
        address
    }

    pub fn insert_float(&mut self, value: f32, lifetime: &str) -> i32 {
        let key = format!("float_{}", lifetime);
        let offset = match lifetime {
            "global" => START_FLOAT_VALUES,
            "local" => START_FLOAT_VALUES + LOCAL_VALUES,
            "temp" => START_FLOAT_VALUES + TEMP_VALUES,
            _ => panic!("Invalid lifetime"),
        };

        let array_index = match lifetime {
            "global" => 0,
            "local" => 1,
            "temp" => 2,
            _ => panic!("Invalid lifetime"),
        };

        let address = self.var_values[1][array_index].len() as i32 + offset;
        self.var_values[1][array_index].push(Value::Float(value));
        *self.counters.entry(key).or_insert(0) += 1;
        address
    }

    pub fn insert_bool(&mut self, value: bool) -> i32 {
        let address = self.var_values[2][0].len() as i32 + START_BOOL_VALUES;
        self.var_values[2][0].push(Value::Bool(value));
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

    pub fn clear_bool(&mut self) {
        self.var_values[2].clear();
    }

    pub fn get_int(&self, address: i32) -> i32 {
        let mut array_index = 0;
        let mut position = address - START_INT_VALUES;
        let mut int_type = "var";
        if address >= START_INT_VALUES + LOCAL_VALUES && address < START_INT_VALUES + TEMP_VALUES {
            array_index = 1;
            position = position - LOCAL_VALUES;
        } else if address >= START_INT_VALUES + TEMP_VALUES && address < START_FLOAT_VALUES {
            array_index = 2;
            position = position - TEMP_VALUES;
        } else if address >= START_CONST_INT_VALUES && address < START_CONST_FLOAT_VALUES {
            int_type = "const";
            position = address - START_CONST_INT_VALUES;
        }

        match int_type {
            "var" => match self.var_values[0][array_index][position as usize] {
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

    pub fn set_int(&mut self, address: i32, value: i32) {
        let mut array_index = 0;
        let mut position = address - START_INT_VALUES;
        if address >= START_INT_VALUES + LOCAL_VALUES && address < START_INT_VALUES + TEMP_VALUES {
            array_index = 1;
            position = position - LOCAL_VALUES;
        } else if address >= START_INT_VALUES + TEMP_VALUES {
            array_index = 2;
            position = position - TEMP_VALUES;
        }
        self.var_values[0][array_index][position as usize] = Value::Int(value);
    }

    pub fn get_float(&self, address: i32) -> f32 {
        let mut array_index = 0;
        let mut position = address - START_FLOAT_VALUES;
        let mut float_type = "var";
        if address >= START_FLOAT_VALUES + LOCAL_VALUES
            && address < START_FLOAT_VALUES + TEMP_VALUES
        {
            array_index = 1;
            position = position - LOCAL_VALUES;
        } else if address >= START_FLOAT_VALUES + TEMP_VALUES && address < START_CONST_FLOAT_VALUES
        {
            array_index = 2;
            position = position - TEMP_VALUES;
        } else if address >= START_CONST_FLOAT_VALUES {
            float_type = "const";
            position = address - START_CONST_FLOAT_VALUES;
        }

        match float_type {
            "var" => match self.var_values[1][array_index][position as usize] {
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

    pub fn set_float(&mut self, address: i32, value: f32) {
        let mut array_index = 0;
        let mut position = address - START_FLOAT_VALUES;
        if address >= START_FLOAT_VALUES + LOCAL_VALUES
            && address < START_FLOAT_VALUES + TEMP_VALUES
        {
            array_index = 1;
            position = position - LOCAL_VALUES;
        } else if address >= START_FLOAT_VALUES + TEMP_VALUES {
            array_index = 2;
            position = position - TEMP_VALUES;
        }
        self.var_values[1][array_index][position as usize] = Value::Float(value);
    }

    pub fn get_bool(&self, address: i32) -> bool {
        match self.var_values[2][0][(address - START_BOOL_VALUES) as usize] {
            Value::Bool(value) => value,
            _ => panic!("Invalid address"),
        }
    }

    pub fn set_bool(&mut self, address: i32, value: bool) {
        self.var_values[2][0][(address - START_BOOL_VALUES) as usize] = Value::Bool(value);
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
