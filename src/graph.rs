use std::fmt::Debug;

use raug::prelude::*;

pub trait GraphExt {
    fn channel<T: Signal + Default + Debug>(&self) -> (Node, Node);
}

impl GraphExt for Graph {
    fn channel<T: Signal + Default + Debug>(&self) -> (Node, Node) {
        let (tx, rx) = crate::processors::util::signal_channel::<T>();
        (self.node(tx), self.node(rx))
    }
}
