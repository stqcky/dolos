use std::fmt::{self};

use petgraph::{stable_graph::DefaultIx, Directed, Graph};

use crate::parser::ast::definition::{Expression, Statement};

pub mod translator;
pub mod visualization;

#[derive(Clone)]
pub enum CFGEdge {
    Conditional(Expression),
    Unconditional(),
}

impl fmt::Debug for CFGEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conditional(expr) => {
                write!(f, "{}", expr)?;
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct CFGNode {
    statements: Vec<Statement>,
}

impl fmt::Debug for CFGNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in self.statements.iter() {
            write!(f, "{}", stmt)?;
        }

        Ok(())
    }
}

pub type CFG = Graph<CFGNode, CFGEdge, Directed, DefaultIx>;
