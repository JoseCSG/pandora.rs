use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
    Bool,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
}

pub struct CuboSemantico {
    pub res_operaciones: HashMap<(Type, Operator, Type), Type>,
}

impl CuboSemantico {
    pub fn new() -> Self {
        let mut res_operaciones = HashMap::new();

        // Operaciones entre enteros
        res_operaciones.insert((Type::Int, Operator::Add, Type::Int), Type::Int);
        res_operaciones.insert((Type::Int, Operator::Subtract, Type::Int), Type::Int);
        res_operaciones.insert((Type::Int, Operator::Multiply, Type::Int), Type::Int);
        res_operaciones.insert((Type::Int, Operator::Divide, Type::Int), Type::Int);
        res_operaciones.insert((Type::Int, Operator::LessThan, Type::Int), Type::Bool);
        res_operaciones.insert((Type::Int, Operator::GreaterThan, Type::Int), Type::Bool);
        res_operaciones.insert((Type::Int, Operator::Equal, Type::Int), Type::Bool);
        res_operaciones.insert((Type::Int, Operator::NotEqual, Type::Int), Type::Bool);

        // Operaciones entre flotantes
        res_operaciones.insert((Type::Float, Operator::Add, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::Subtract, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::Multiply, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::Divide, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::LessThan, Type::Float), Type::Bool);
        res_operaciones.insert(
            (Type::Float, Operator::GreaterThan, Type::Float),
            Type::Bool,
        );
        res_operaciones.insert((Type::Float, Operator::Equal, Type::Float), Type::Bool);
        res_operaciones.insert((Type::Float, Operator::NotEqual, Type::Float), Type::Bool);

        // Operaciones entre enteros y flotantes
        res_operaciones.insert((Type::Int, Operator::Add, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::Add, Type::Int), Type::Float);

        res_operaciones.insert((Type::Int, Operator::Subtract, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::Subtract, Type::Int), Type::Float);

        res_operaciones.insert((Type::Int, Operator::Multiply, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::Multiply, Type::Int), Type::Float);

        res_operaciones.insert((Type::Int, Operator::Divide, Type::Float), Type::Float);
        res_operaciones.insert((Type::Float, Operator::Divide, Type::Int), Type::Float);

        res_operaciones.insert((Type::Int, Operator::LessThan, Type::Float), Type::Bool);
        res_operaciones.insert((Type::Float, Operator::LessThan, Type::Int), Type::Bool);

        res_operaciones.insert((Type::Int, Operator::GreaterThan, Type::Float), Type::Bool);
        res_operaciones.insert((Type::Float, Operator::GreaterThan, Type::Int), Type::Bool);

        res_operaciones.insert((Type::Int, Operator::Equal, Type::Float), Type::Bool);
        res_operaciones.insert((Type::Float, Operator::Equal, Type::Int), Type::Bool);

        res_operaciones.insert((Type::Int, Operator::NotEqual, Type::Float), Type::Bool);
        res_operaciones.insert((Type::Float, Operator::NotEqual, Type::Int), Type::Bool);

        res_operaciones.insert((Type::Bool, Operator::Add, Type::Int), Type::Error);
        res_operaciones.insert((Type::Bool, Operator::Multiply, Type::Int), Type::Error);
        res_operaciones.insert((Type::Bool, Operator::Subtract, Type::Int), Type::Error);
        res_operaciones.insert((Type::Bool, Operator::Divide, Type::Int), Type::Error);

        res_operaciones.insert((Type::Bool, Operator::Add, Type::Float), Type::Error);
        res_operaciones.insert((Type::Bool, Operator::Multiply, Type::Float), Type::Error);
        res_operaciones.insert((Type::Bool, Operator::Subtract, Type::Float), Type::Error);
        res_operaciones.insert((Type::Bool, Operator::Divide, Type::Float), Type::Error);

        res_operaciones.insert((Type::Int, Operator::Add, Type::Bool), Type::Error);
        res_operaciones.insert((Type::Int, Operator::Multiply, Type::Bool), Type::Error);
        res_operaciones.insert((Type::Int, Operator::Subtract, Type::Bool), Type::Error);
        res_operaciones.insert((Type::Int, Operator::Divide, Type::Bool), Type::Error);

        res_operaciones.insert((Type::Float, Operator::Add, Type::Bool), Type::Error);
        res_operaciones.insert((Type::Float, Operator::Multiply, Type::Bool), Type::Error);
        res_operaciones.insert((Type::Float, Operator::Subtract, Type::Bool), Type::Error);
        res_operaciones.insert((Type::Float, Operator::Divide, Type::Bool), Type::Error);

        Self { res_operaciones }
    }

    pub fn get_type(&self, t1: Type, op: Operator, t2: Type) -> Type {
        match self.res_operaciones.get(&(t1, op, t2)) {
            Some(t) => t.clone(),
            None => Type::Error,
        }
    }
}
