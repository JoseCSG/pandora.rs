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
    total_int_values: i32,
    total_float_values: i32,
    var_values: Vec<Vec<Value>>,
    const_values: Vec<ConstValue>,
}

impl ValueTable {
    pub fn new() -> Self {
        ValueTable {
            total_int_values: 0,
            total_float_values: 0,
            var_values: vec![vec![], vec![], vec![]],
            const_values: vec![],
        }
    }

    pub fn insert_integer(&mut self, value: i32, lifetime: &str) -> i32 {
        let address = match lifetime {
            "global" => {
                self.var_values[0].insert(0, Value::Int(value));
                self.total_int_values += 1;
                self.var_values[0].len() as i32 + START_INT_VALUES
            }
            "local" => {
                self.var_values[0].push(Value::Int(value));
                self.total_int_values += 1;
                self.var_values[0].len() as i32 + START_INT_VALUES + LOCAL_VALUES
            }
            "temp" => {
                self.var_values[0].push(Value::Int(value));
                self.total_int_values += 1;
                self.var_values[0].len() as i32 + START_INT_VALUES + TEMP_VALUES
            }
            _ => panic!("Invalid lifetime"),
        };
        self.var_values[0].push(Value::Int(value));
        self.total_int_values += 1;
        address
    }

    pub fn insert_float(&mut self, value: f32, lifetime: &str) -> i32 {
        let address = match lifetime {
            "global" => {
                self.var_values[1].insert(0, Value::Float(value));
                self.total_int_values += 1;
                self.var_values[1].len() as i32 + START_FLOAT_VALUES
            }
            "local" => {
                self.var_values[1].push(Value::Float(value));
                self.total_int_values += 1;
                self.var_values[1].len() as i32 + START_FLOAT_VALUES + LOCAL_VALUES
            }
            "temp" => {
                self.var_values[1].push(Value::Float(value));
                self.total_int_values += 1;
                self.var_values[1].len() as i32 + START_FLOAT_VALUES + TEMP_VALUES
            }
            _ => panic!("Invalid lifetime"),
        };

        self.var_values[1].push(Value::Float(value));
        self.total_float_values += 1;
        address
    }

    pub fn insert_bool(&mut self, value: bool) -> i32 {
        self.var_values[2].push(Value::Bool(value));
        self.var_values[2].len() as i32 + START_BOOL_VALUES
    }

    pub fn insert_cte_int(&mut self, value: i32) -> i32 {
        self.const_values.push(ConstValue::Int(value));
        self.const_values.len() as i32 + START_CONST_INT_VALUES
    }

    pub fn insert_cte_float(&mut self, value: f32) -> i32 {
        self.const_values.push(ConstValue::Float(value));
        self.const_values.len() as i32 + START_CONST_FLOAT_VALUES
    }

    pub fn insert_cte_string(&mut self, value: String) -> i32 {
        self.const_values.push(ConstValue::String(value));
        self.const_values.len() as i32 + START_CONST_STRING_VALUES
    }

    pub fn clear_temp_int(&mut self) {
        self.var_values[0] = self.var_values[0][0..self.total_int_values as usize].to_vec();
    }

    pub fn clear_temp_float(&mut self) {
        self.var_values[1] = self.var_values[1][0..self.total_float_values as usize].to_vec();
    }

    pub fn clear_bool(&mut self) {
        self.var_values[2].clear();
    }

    pub fn get_int(&self, address: i32) -> i32 {
        match self.var_values[0][(address - START_INT_VALUES) as usize] {
            Value::Int(value) => value,
            _ => panic!("Invalid address"),
        }
    }

    pub fn get_float(&self, address: i32) -> f32 {
        match self.var_values[1][(address - START_FLOAT_VALUES) as usize] {
            Value::Float(value) => value,
            _ => panic!("Invalid address"),
        }
    }

    pub fn get_bool(&self, address: i32) -> bool {
        match self.var_values[2][(address - START_BOOL_VALUES) as usize] {
            Value::Bool(value) => value,
            _ => panic!("Invalid address"),
        }
    }
}
