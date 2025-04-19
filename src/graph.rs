use raug::prelude::*;

pub trait GraphExt {
    fn channel<T: Signal + Default>(&self) -> (Node, Node);
}

impl GraphExt for Graph {
    fn channel<T: Signal + Default>(&self) -> (Node, Node) {
        let (tx, rx) = crate::processors::util::signal_channel::<T>();
        (self.add(tx), self.add(rx))
    }
}
