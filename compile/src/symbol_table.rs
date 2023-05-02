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

pub enum RefType {
    ClassRefType(ClassVarDecorator),
    SubroutineRefType(SubroutineVarDecorator),
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
        type_name: Type,
    ) -> Result<(), ParseError> {
        let class_table = self.get_class_table_mut()?;

        let var_number = match decorator {
            ClassVarDecorator::Static => {
                let result = class_table.static_number;
                class_table.static_number += 1;
                result
            }
            ClassVarDecorator::Field => {
                let result = class_table.field_number;
                class_table.field_number += 1;
                result
            }
        };

        swap_result(
            class_table
                .class_vars
                .insert(name, (decorator, type_name, var_number))
                .map(|_| ParseError::DuplicatedClassLevelVariable)
                .ok_or(()),
        )
    }

    #[allow(clippy::unit_arg)]
    pub fn enter_subroutine(
        &mut self,
        subroutine: String,
        decorator: SubroutineDecorator,
        type_name: Option<Type>,
    ) -> Result<(), ParseError> {
        let class_table = self.get_class_table_mut()?;
        swap_result(
            class_table
                .subroutines
                .insert(
                    subroutine.clone(),
                    SubroutineSymbolTable::new(decorator, type_name),
                )
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

    pub fn enter_arg(&mut self, name: String, type_name: Type) -> Result<(), ParseError> {
        let subroutine_table = self.get_subroutine_table_mut()?;
        let arg_number = subroutine_table.arg_number;
        subroutine_table.arg_number = arg_number + 1;
        swap_result(
            subroutine_table
                .vars
                .insert(name, (SubroutineVarDecorator::Arg, type_name, arg_number))
                .map(|_| ParseError::DuplicatedFuncitonLevelVariable)
                .ok_or(()),
        )
    }

    pub fn enter_local(&mut self, name: String, type_name: Type) -> Result<(), ParseError> {
        let subroutine_table = self.get_subroutine_table_mut()?;
        let local_number = subroutine_table.local_number;
        subroutine_table.local_number = local_number + 1;
        swap_result(
            subroutine_table
                .vars
                .insert(
                    name,
                    (SubroutineVarDecorator::Local, type_name, local_number),
                )
                .map(|_| ParseError::DuplicatedFuncitonLevelVariable)
                .ok_or(()),
        )
    }

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

    pub fn lookup_var(
        &self,
        class: &str,
        subroutine: &str,
        name: &str,
    ) -> Result<(RefType, Type, usize), ParseError> {
        let subroutine_table = self.get_subroutine_table(class, subroutine)?;
        let subroutine_level =
            subroutine_table
                .vars
                .get(name)
                .map(|(ref_type, type_name, number)| {
                    (
                        RefType::SubroutineRefType(*ref_type),
                        type_name.clone(),
                        *number,
                    )
                });

        match subroutine_level {
            Some(result) => Ok(result),
            None => {
                let class_table = self.get_class_table(class)?;
                let class_level =
                    class_table
                        .class_vars
                        .get(name)
                        .map(|(ref_type, type_name, number)| {
                            (RefType::ClassRefType(*ref_type), type_name.clone(), *number)
                        });
                class_level.ok_or(ParseError::SymbolNotFound)
            }
        }
    }

    pub fn lookup_subroutine(
        &self,
        class: &str,
        subroutine: &str,
    ) -> Result<(SubroutineDecorator, usize, usize), ParseError> {
        let sub_routine_table = self.get_subroutine_table(class, subroutine)?;
        Ok((
            sub_routine_table.decorator,
            sub_routine_table.arg_number,
            sub_routine_table.local_number,
        ))
    }

    pub fn lookup_class(&self, class: &str) -> Result<usize, ParseError> {
        let class_table = self.get_class_table(class)?;
        Ok(class_table.field_number)
    }
}

pub struct ClassSymbolTable {
    pub class_vars: HashMap<String, (ClassVarDecorator, Type, usize)>,
    pub subroutines: HashMap<String, SubroutineSymbolTable>,
    pub static_number: usize,
    pub field_number: usize,
}

impl ClassSymbolTable {
    fn new() -> Self {
        Self {
            class_vars: HashMap::new(),
            subroutines: HashMap::new(),
            static_number: 0,
            field_number: 0,
        }
    }
}

pub struct SubroutineSymbolTable {
    pub vars: HashMap<String, (SubroutineVarDecorator, Type, usize)>,
    pub decorator: SubroutineDecorator,
    pub type_name: Option<Type>,
    pub arg_number: usize,
    pub local_number: usize,
}

impl SubroutineSymbolTable {
    fn new(decorator: SubroutineDecorator, type_name: Option<Type>) -> Self {
        Self {
            vars: HashMap::new(),
            decorator,
            type_name,
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
