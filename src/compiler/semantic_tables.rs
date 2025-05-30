use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VarValue {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableInfo {
    pub name: String,
    pub value: VarValue,
    pub var_type: VarType,
    pub address: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam {
    pub var_type: VarType,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub vars: VariableTable,
    pub vars_amount: Vec<Vec<i32>>,
    pub start_address: i32,
}

pub type VariableTable = HashMap<String, VariableInfo>;
pub type FunctionTable = HashMap<String, FunctionInfo>;
