use crate::compiler::semantic_cube::CuboSemantico;
use crate::compiler::semantic_tables::FunctionTable;
use crate::utils::stack::Stack;
use std::collections::HashMap;

pub struct ProgramManager {
    pub cubo: CuboSemantico,
    pub tabla_funciones: FunctionTable,
    pub scope_stack: Stack<String>,
}

impl ProgramManager {
    pub fn new() -> Self {
        ProgramManager {
            cubo: CuboSemantico::new(),
            tabla_funciones: HashMap::new(),
            scope_stack: Stack::new(),
        }
    }
}
