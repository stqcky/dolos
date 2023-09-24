use std::{cell::RefCell, fs};

use log::error;
use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraph,
    stable_graph::{DefaultIx, NodeIndex},
};

use super::{CFGEdge, CFGNode, CFG};

pub fn visualize(cfg: CFG) {
    let dot = format!("{:?}", Dot::with_config(&cfg, &[]));

    if let Err(e) = fs::write("cfg.dot", dot) {
        error!("could not write cfg.dot: {e}");
    }
}
