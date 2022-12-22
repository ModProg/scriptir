use std::collections::HashMap;

use budvm::{
    ir::{CodeBlockBuilder, Instruction as Binstruction, LiteralOrSource},
    Environment, Value, VirtualMachine,
};
use derive_more::DebugCustom;
use macro_rules_attribute::{apply, attribute_alias};
use serde::Deserialize;

attribute_alias! {
    #[apply(Ir)] =
        #[derive(Debug)]
        #[derive(Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[cfg_attr(test, derive(PartialEq))];
}

#[apply(Ir)]
pub struct Script {
    pub instructions: Vec<Instruction>,
}

impl Script {
    pub fn compile<E: Environment>(
        &self,
        vm: &mut VirtualMachine<E>,
    ) -> Result<budvm::CodeBlock<E::Intrinsic>, budvm::ir::LinkError> {
        let mut code_block = CodeBlockBuilder::default();
        for instruction in &self.instructions {
            match instruction {
                Instruction::Assign { name, assignment } => todo!(),
                Instruction::Scope { name, instructions } => todo!(),
                Instruction::Exit { scope, value } => {
                    let value = value.compile(&mut code_block);
                    code_block.push(Binstruction::Return(value.into()))
                }
            }
        }
        let code_block = code_block.finish();
        code_block.link(vm)
    }
}

#[apply(Ir)]
pub enum Instruction {
    Assign {
        name: String,
        assignment: Assignable,
    },
    Scope {
        name: String,
        instructions: Vec<Instruction>,
    },
    Exit {
        scope: Option<String>,
        value: Expr,
    },
}

#[apply(Ir)]
pub enum Assignable {
    Function {
        arguments: Vec<String>,
        instructions: Vec<Instruction>,
    },
    Expression(Expr),
}

#[derive(DebugCustom, Deserialize)]
pub enum Expr {
    #[serde(skip)]
    #[debug(fmt = "Dynamic{{args: {args:?}, ...}}")]
    Dynamic {
        operator: Box<dyn Fn(Vec<Value>) -> Value>,
        args: Vec<Expr>,
    },
    #[debug(fmt = "Builtin({_0:?})")]
    Builtin(ExprBuiltin),
    #[debug(fmt = "Literal({_0:?})")]
    Literal(Literal),
}
impl Expr {
    fn compile<T>(&self, code_block: &mut CodeBlockBuilder<T>) -> budvm::ir::LiteralOrSource {
        let tmp = code_block.new_temporary_variable();
        match self {
            Expr::Dynamic { operator, args } => todo!(),
            Expr::Builtin(_) => todo!(),
            Expr::Literal(lit) => {
                let val = match lit {
                    Literal::Bool(_) => todo!(),
                    Literal::Number(num) => budvm::ir::Literal::Real(*num),
                    Literal::String(_) => todo!(),
                    Literal::Map(_) => todo!(),
                    Literal::List(_) => todo!(),
                };
                code_block.push(Binstruction::Load {
                    variable: tmp.clone(),
                    value: LiteralOrSource::Literal(val.into()),
                });
            }
        }
        LiteralOrSource::Variable(tmp)
    }
}

#[cfg(test)]
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Dynamic { .. }, Self::Dynamic { .. }) => {
                unimplemented!("Dynamic Operators cannot be compared")
            }
            (Self::Builtin(l0), Self::Builtin(r0)) => l0 == r0,
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[apply(Ir)]
#[serde(untagged)]
pub enum ExprBuiltin {
    Binary {
        op: BinOp,
        operand: Box<[Expr; 2]>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<[Expr; 1]>,
    },
}

#[apply(Ir)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

#[apply(Ir)]
pub enum UnaryOp {
    Not,
    BitNot,
}

#[apply(Ir)]
#[serde(untagged)]
pub enum Literal {
    Bool(bool),
    Number(f64),
    String(String),
    Map(HashMap<String, Literal>),
    List(Vec<Literal>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let Script { instructions }: Script =
            serde_yaml::from_str(include_str!("../../tests/ir/test.yml")).unwrap();

        assert_eq!(
            instructions,
            vec![Instruction::Exit {
                scope: None,
                value: Expr::Literal(Literal::Number(10.))
            }]
        )
    }
}
