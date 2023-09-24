use std::{cell::RefCell, rc::Rc};

use petgraph::{stable_graph::DefaultIx, stable_graph::NodeIndex, Directed, Direction, Graph};

use crate::parser::ast::definition::{Block, Statement};

use super::{CFGEdge, CFGNode, CFG};

fn translate_statement(cfg: &mut CFG, last: NodeIndex, stmt: &Statement) -> NodeIndex {
    match stmt {
        Statement::If(stmt) => {
            let mut branches = vec![];

            let true_branch = stmt.block.statements.first().unwrap();
            let true_node = cfg.add_node(CFGNode {
                statements: vec![true_branch.clone()],
            });

            cfg.add_edge(
                last,
                true_node,
                CFGEdge::Conditional(stmt.condition.clone()),
            );

            branches.push(true_node);

            for elseif_block in stmt.elseif_blocks.iter() {
                let condition = elseif_block.condition.clone();
                let branch = elseif_block.block.statements.first().unwrap();

                let node = cfg.add_node(CFGNode {
                    statements: vec![branch.clone()],
                });
                cfg.add_edge(last, node, CFGEdge::Conditional(condition));

                branches.push(node);
            }

            if let Some(else_block) = &stmt.else_block {
                let else_branch = else_block.statements.first().unwrap();

                let else_node = cfg.add_node(CFGNode {
                    statements: vec![else_branch.clone()],
                });
                cfg.add_edge(last, else_node, CFGEdge::Unconditional());

                branches.push(else_node);
            }

            let merge_node = cfg.add_node(CFGNode { statements: vec![] });

            for branch in branches.iter() {
                cfg.add_edge(*branch, merge_node, CFGEdge::Unconditional());
            }

            merge_node
        }
        Statement::While(stmt) => {
            let condition = &stmt.condition;

            let condition_node = cfg.add_node(CFGNode { statements: vec![] });
            cfg.add_edge(last, condition_node, CFGEdge::Unconditional());

            let block = stmt.block.statements.iter().next().unwrap().clone();
            let node = cfg.add_node(CFGNode {
                statements: vec![block],
            });

            cfg.add_edge(
                condition_node,
                node,
                CFGEdge::Conditional(condition.clone()),
            );
            cfg.add_edge(node, condition_node, CFGEdge::Unconditional());

            let merge = cfg.add_node(CFGNode { statements: vec![] });

            cfg.add_edge(condition_node, merge, CFGEdge::Unconditional());

            merge
        }
        _ => {
            let node = cfg.add_node(CFGNode {
                statements: vec![stmt.clone()],
            });

            cfg.add_edge(last, node, CFGEdge::Unconditional());

            node
        }
    }
}

fn translate_statements(statements: Vec<Statement>) -> CFG {
    let mut graph = CFG::new();

    let mut stmts = statements.iter();

    let mut last = graph.add_node(CFGNode { statements: vec![] });

    while let Some(stmt) = stmts.next() {
        let new_last = translate_statement(&mut graph, last, stmt);

        // let last_neighbors_count = graph.neighbors_directed(last, Direction::Outgoing).count();

        last = new_last;
    }

    graph
}

pub fn translate(block: &Block) -> CFG {
    translate_statements(block.statements.clone())
}
