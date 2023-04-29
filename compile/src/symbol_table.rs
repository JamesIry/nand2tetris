use crate::ast::*;
use std::collections::HashMap;

pub struct SymbolTable {
    pub classes: HashMap<String, ClassSymbolTable>,
    last_class: String,
    last_subroutine: String,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubroutineVarDecorator {
    Arg,
    Local,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            last_class: "".to_string(),
            last_subroutine: "".to_string(),
        }
    }

    #[allow(clippy::unit_arg)]
    pub fn enter_class(&mut self, class: String) -> Result<(), ParseError> {
        swap_result(
            self.classes
                .insert(class.clone(), ClassSymbolTable::new())
                .map(|_| ParseError::DuplicatedClass)
                .ok_or({
                    self.last_class = class;
                }),
        )
    }

    fn get_class_table_mut(&mut self) -> Result<&mut ClassSymbolTable, ParseError> {
        self.classes
            .get_mut(&self.last_class)
            .ok_or(ParseError::ClassNotFound)
    }

    pub fn enter_class_var(
        &mut self,
        name: String,
        decorator: ClassVarDecorator,
    ) -> Result<(), ParseError> {
        let class_table = self.get_class_table_mut()?;

        let field_number = match decorator {
            ClassVarDecorator::Static => 0,
            ClassVarDecorator::Field => {
                let result = class_table.field_number;
                class_table.field_number += 1;
                result
            }
        };

        swap_result(
            class_table
                .class_vars
                .insert(name, (decorator, field_number))
                .map(|_| ParseError::DuplicatedClassLevelVariable)
                .ok_or(()),
        )
    }

    #[allow(clippy::unit_arg)]
    pub fn enter_subroutine(
        &mut self,
        subroutine: String,
        decorator: SubroutineDecorator,
    ) -> Result<(), ParseError> {
        let class_table = self.get_class_table_mut()?;
        swap_result(
            class_table
                .subroutines
                .insert(subroutine.clone(), SubroutineSymbolTable::new(decorator))
                .map(|_| ParseError::DuplicatedSubroutine)
                .ok_or(self.last_subroutine = subroutine),
        )
    }

    fn get_subroutine_table_mut(&mut self) -> Result<&mut SubroutineSymbolTable, ParseError> {
        let last_subroutine = self.last_subroutine.clone();
        let class_table = self.get_class_table_mut()?;

        class_table
            .subroutines
            .get_mut(&last_subroutine)
            .ok_or(ParseError::ClassNotFound)
    }

    pub fn enter_arg(&mut self, name: String) -> Result<(), ParseError> {
        let subroutine_table = self.get_subroutine_table_mut()?;
        let arg_number = subroutine_table.arg_number;
        subroutine_table.arg_number = arg_number + 1;
        swap_result(
            subroutine_table
                .vars
                .insert(name, (SubroutineVarDecorator::Arg, arg_number))
                .map(|_| ParseError::DuplicatedFuncitonLevelVariable)
                .ok_or(()),
        )
    }

    pub fn enter_local(&mut self, name: String) -> Result<(), ParseError> {
        let subroutine_table = self.get_subroutine_table_mut()?;
        let local_number = subroutine_table.local_number;
        subroutine_table.local_number = local_number + 1;
        swap_result(
            subroutine_table
                .vars
                .insert(name, (SubroutineVarDecorator::Local, local_number))
                .map(|_| ParseError::DuplicatedFuncitonLevelVariable)
                .ok_or(()),
        )
    }
    /*
    fn get_class_table(&self, class: &str) -> Result<&ClassSymbolTable, ParseError> {
        self.classes.get(class).ok_or(ParseError::ClassNotFound)
    }

    fn get_subroutine_table(
        &self,
        class: &str,
        subroutine: &str,
    ) -> Result<&SubroutineSymbolTable, ParseError> {
        let class_table = self.get_class_table(class)?;

        class_table
            .subroutines
            .get(subroutine)
            .ok_or(ParseError::ClassNotFound)
    }

    fn lookup(
        &self,
        class: &str,
        subroutine: &str,
        name: &str,
    ) -> Result<(RefType, usize), ParseError> {
        let class_table = self.get_class_table(class)?;

        let top_level = class_table
            .class_vars
            .get(class)
            .map(|(ref_type, number)| (RefType::ClassRefType(*ref_type), *number));

        match top_level {
            Some(result) => Ok(result),
            None => {
                let subroutine_table = self.get_subroutine_table(class, subroutine)?;
                let subroutine_level = subroutine_table
                    .vars
                    .get(name)
                    .map(|(ref_type, number)| (RefType::SubroutineRefType(*ref_type), *number));
                subroutine_level.ok_or(ParseError::SymbolNotFound)
            }
        }
    }
    */
}

pub struct ClassSymbolTable {
    pub class_vars: HashMap<String, (ClassVarDecorator, usize)>,
    pub subroutines: HashMap<String, SubroutineSymbolTable>,
    field_number: usize,
}

impl ClassSymbolTable {
    fn new() -> Self {
        Self {
            class_vars: HashMap::new(),
            subroutines: HashMap::new(),
            field_number: 0,
        }
    }
}

pub struct SubroutineSymbolTable {
    pub vars: HashMap<String, (SubroutineVarDecorator, usize)>,
    pub decorator: SubroutineDecorator,
    arg_number: usize,
    local_number: usize,
}

impl SubroutineSymbolTable {
    fn new(decorator: SubroutineDecorator) -> Self {
        Self {
            vars: HashMap::new(),
            decorator,
            arg_number: 0,
            local_number: 0,
        }
    }
}

fn swap_result<E, T>(x: Result<E, T>) -> Result<T, E> {
    match x {
        Ok(e) => Err(e),
        Err(t) => Ok(t),
    }
}
