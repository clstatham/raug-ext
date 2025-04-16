use raug::prelude::*;

use crate::prelude::*;

macro_rules! choose_node_generics {
    ($graph:expr, $signal_type:expr => $node_type:ident => $($options:ty)*) => {
        match $signal_type {
            $(
                t if t == <$options>::signal_type() => $graph.add($node_type::<$options>::default()),
            )*
            _ => panic!("Unsupported signal type: {:?}", $signal_type),
        }
    };
}

pub trait OutputExt {
    fn powf(&self, b: impl IntoOutput) -> Node;
    fn powi(&self, b: impl IntoOutput) -> Node;
    fn sqrt(&self) -> Node;
    fn sin(&self) -> Node;
    fn cos(&self) -> Node;
    fn tan(&self) -> Node;
    fn asin(&self) -> Node;
    fn acos(&self) -> Node;
    fn atan(&self) -> Node;
    fn sinh(&self) -> Node;
    fn cosh(&self) -> Node;
    fn tanh(&self) -> Node;
    fn atan2(&self, b: impl IntoOutput) -> Node;
    fn hypot(&self, b: impl IntoOutput) -> Node;
    fn abs(&self) -> Node;
    fn ceil(&self) -> Node;
    fn floor(&self) -> Node;
    fn round(&self) -> Node;
    fn trunc(&self) -> Node;
    fn fract(&self) -> Node;
    fn recip(&self) -> Node;
    fn signum(&self) -> Node;
    fn max(&self, b: impl IntoOutput) -> Node;
    fn min(&self, b: impl IntoOutput) -> Node;
    fn clamp(&self, min: impl IntoOutput, max: impl IntoOutput) -> Node;

    fn cast<T: Signal + CastTo<U> + Default, U: Signal + Default>(&self) -> Node;
    fn some(&self) -> Node;
    fn unwrap_or(&self, b: impl IntoOutput) -> Node;
}

macro_rules! generic_binary_op_impl {
    ($self:ident, $b:ident, $op:ident => $($options:ty)*) => {{
        let this_node = $self.node();
        let graph = this_node.graph();
        let b = $b.into_output(graph);
        assert_eq!(
            $self.signal_type(),
            b.signal_type(),
            "Signal types must match for this operation",
        );
        let node = choose_node_generics!(graph, $self.signal_type() => $op => $($options)*);
        node.input(0).connect($self);
        node.input(1).connect(b);
        node
    }};
}

macro_rules! specific_binary_op_impl {
    ($self:ident, $b:ident, $op:ident => $type:ident) => {{
        let this_node = $self.node();
        let graph = this_node.graph();
        assert_eq!(
            $self.signal_type(),
            $type::signal_type(),
            "Signal type must be {} for this operation",
            stringify!($type),
        );
        let b = $b.into_output(graph);
        assert_eq!(
            $self.signal_type(),
            b.signal_type(),
            "Signal types must match for this operation",
        );
        let node = graph.add($op::default());
        node.input(0).connect($self);
        node.input(1).connect(b);
        node
    }};
}

macro_rules! generic_unary_op_impl {
    ($self:ident, $op:ident => $($options:ty)*) => {{
        let this_node = $self.node();
        let graph = this_node.graph();
        let node = choose_node_generics!(graph, $self.signal_type() => $op => $($options)*);
        node.input(0).connect($self);
        node
    }};
}

macro_rules! specific_unary_op_impl {
    ($self:ident, $op:ident => $type:ident) => {{
        let this_node = $self.node();
        let graph = this_node.graph();
        assert_eq!(
            $self.signal_type(),
            $type::signal_type(),
            "Signal type must be {} for this operation",
            stringify!($type),
        );
        let node = graph.add($op::default());
        node.input(0).connect($self);
        node
    }};
}

impl OutputExt for Output {
    #[inline]
    #[track_caller]
    fn powf(&self, b: impl IntoOutput) -> Node {
        specific_binary_op_impl!(self, b, Powf => f32)
    }

    #[inline]
    #[track_caller]
    fn powi(&self, b: impl IntoOutput) -> Node {
        let this_node = self.node();
        let graph = this_node.graph();
        let b = b.into_output(graph);
        assert_eq!(
            self.signal_type(),
            f32::signal_type(),
            "LHS Signal type must be f32 for this operation",
        );
        assert_eq!(
            b.signal_type(),
            i64::signal_type(),
            "RHS Signal type must be i64 for this operation",
        );
        let node = graph.add(Powi::default());
        node.input(0).connect(self);
        node.input(1).connect(b);
        node
    }

    #[inline]
    #[track_caller]
    fn sqrt(&self) -> Node {
        specific_unary_op_impl!(self, Sqrt => f32)
    }

    #[inline]
    #[track_caller]
    fn sin(&self) -> Node {
        specific_unary_op_impl!(self, Sin => f32)
    }

    #[inline]
    #[track_caller]
    fn cos(&self) -> Node {
        specific_unary_op_impl!(self, Cos => f32)
    }

    #[inline]
    #[track_caller]
    fn tan(&self) -> Node {
        specific_unary_op_impl!(self, Tan => f32)
    }

    #[inline]
    #[track_caller]
    fn asin(&self) -> Node {
        specific_unary_op_impl!(self, Asin => f32)
    }

    #[inline]
    #[track_caller]
    fn acos(&self) -> Node {
        specific_unary_op_impl!(self, Acos => f32)
    }

    #[inline]
    #[track_caller]
    fn atan(&self) -> Node {
        specific_unary_op_impl!(self, Atan => f32)
    }

    #[inline]
    #[track_caller]
    fn sinh(&self) -> Node {
        specific_unary_op_impl!(self, Sinh => f32)
    }

    #[inline]
    #[track_caller]
    fn cosh(&self) -> Node {
        specific_unary_op_impl!(self, Cosh => f32)
    }

    #[inline]
    #[track_caller]
    fn tanh(&self) -> Node {
        specific_unary_op_impl!(self, Tanh => f32)
    }

    #[inline]
    #[track_caller]
    fn atan2(&self, b: impl IntoOutput) -> Node {
        specific_binary_op_impl!(self, b, Atan2 => f32)
    }

    #[inline]
    #[track_caller]
    fn hypot(&self, b: impl IntoOutput) -> Node {
        specific_binary_op_impl!(self, b, Hypot => f32)
    }

    #[inline]
    #[track_caller]
    fn abs(&self) -> Node {
        specific_unary_op_impl!(self, Abs => f32)
    }

    #[inline]
    #[track_caller]
    fn ceil(&self) -> Node {
        specific_unary_op_impl!(self, Ceil => f32)
    }

    #[inline]
    #[track_caller]
    fn floor(&self) -> Node {
        specific_unary_op_impl!(self, Floor => f32)
    }

    #[inline]
    #[track_caller]
    fn round(&self) -> Node {
        specific_unary_op_impl!(self, Round => f32)
    }

    #[inline]
    #[track_caller]
    fn trunc(&self) -> Node {
        specific_unary_op_impl!(self, Trunc => f32)
    }

    #[inline]
    #[track_caller]
    fn fract(&self) -> Node {
        specific_unary_op_impl!(self, Fract => f32)
    }

    #[inline]
    #[track_caller]
    fn recip(&self) -> Node {
        specific_unary_op_impl!(self, Recip => f32)
    }

    #[inline]
    #[track_caller]
    fn signum(&self) -> Node {
        specific_unary_op_impl!(self, Signum => f32)
    }

    #[inline]
    #[track_caller]
    fn max(&self, b: impl IntoOutput) -> Node {
        generic_binary_op_impl!(self, b, Max => f32 i64)
    }

    #[inline]
    #[track_caller]
    fn min(&self, b: impl IntoOutput) -> Node {
        generic_binary_op_impl!(self, b, Min => f32 i64)
    }

    #[inline]
    #[track_caller]
    fn clamp(&self, min: impl IntoOutput, max: impl IntoOutput) -> Node {
        let this_node = self.node();
        let graph = this_node.graph();
        let min = min.into_output(graph);
        let max = max.into_output(graph);
        assert_eq!(
            self.signal_type(),
            min.signal_type(),
            "Signal types must match for this operation",
        );
        assert_eq!(
            self.signal_type(),
            max.signal_type(),
            "Signal types must match for this operation",
        );
        let node = choose_node_generics!(graph, self.signal_type() => Clamp => f32 i64);
        node.input(0).connect(self);
        node.input(1).connect(min);
        node.input(2).connect(max);
        node
    }

    #[inline]
    #[track_caller]
    fn cast<T: Signal + CastTo<U> + Default, U: Signal + Default>(&self) -> Node {
        let this_node = self.node();
        let graph = this_node.graph();
        assert_eq!(
            self.signal_type(),
            T::signal_type(),
            "Signal type must be {} for this operation",
            stringify!(T),
        );
        let node = graph.add(Cast::<T, U>::default());
        node.input(0).connect(self);
        node
    }

    #[inline]
    #[track_caller]
    fn some(&self) -> Node {
        generic_unary_op_impl!(self, Some => f32 i64)
    }

    #[inline]
    #[track_caller]
    fn unwrap_or(&self, b: impl IntoOutput) -> Node {
        let this_node = self.node();
        let graph = this_node.graph();
        let b = b.into_output(graph);
        assert_eq!(
            self.signal_type(),
            Option::<f32>::signal_type(),
            "LHS Signal type must be Option<f32> for this operation",
        );
        assert_eq!(
            b.signal_type(),
            f32::signal_type(),
            "RHS Signal type must be f32 for this operation",
        );
        let node = graph.add(UnwrapOr::<f32>::default());
        node.input(0).connect(self);
        node.input(1).connect(b);
        node
    }
}

impl OutputExt for Node {
    #[inline]
    #[track_caller]
    fn powf(&self, b: impl IntoOutput) -> Node {
        self.assert_single_output("powf");
        self.output(0).powf(b)
    }

    #[inline]
    #[track_caller]
    fn powi(&self, b: impl IntoOutput) -> Node {
        self.assert_single_output("powi");
        self.output(0).powi(b)
    }

    #[inline]
    #[track_caller]
    fn sqrt(&self) -> Node {
        self.assert_single_output("sqrt");
        self.output(0).sqrt()
    }

    #[inline]
    #[track_caller]
    fn sin(&self) -> Node {
        self.assert_single_output("sin");
        self.output(0).sin()
    }

    #[inline]
    #[track_caller]
    fn cos(&self) -> Node {
        self.assert_single_output("cos");
        self.output(0).cos()
    }

    #[inline]
    #[track_caller]
    fn tan(&self) -> Node {
        self.assert_single_output("tan");
        self.output(0).tan()
    }

    #[inline]
    #[track_caller]
    fn asin(&self) -> Node {
        self.assert_single_output("asin");
        self.output(0).asin()
    }

    #[inline]
    #[track_caller]
    fn acos(&self) -> Node {
        self.assert_single_output("acos");
        self.output(0).acos()
    }

    #[inline]
    #[track_caller]
    fn atan(&self) -> Node {
        self.assert_single_output("atan");
        self.output(0).atan()
    }

    #[inline]
    #[track_caller]
    fn sinh(&self) -> Node {
        self.assert_single_output("sinh");
        self.output(0).sinh()
    }

    #[inline]
    #[track_caller]
    fn cosh(&self) -> Node {
        self.assert_single_output("cosh");
        self.output(0).cosh()
    }

    #[inline]
    #[track_caller]
    fn tanh(&self) -> Node {
        self.assert_single_output("tanh");
        self.output(0).tanh()
    }

    #[inline]
    #[track_caller]
    fn atan2(&self, b: impl IntoOutput) -> Node {
        self.assert_single_output("atan2");
        self.output(0).atan2(b)
    }

    #[inline]
    #[track_caller]
    fn hypot(&self, b: impl IntoOutput) -> Node {
        self.assert_single_output("hypot");
        self.output(0).hypot(b)
    }

    #[inline]
    #[track_caller]
    fn abs(&self) -> Node {
        self.assert_single_output("abs");
        self.output(0).abs()
    }

    #[inline]
    #[track_caller]
    fn ceil(&self) -> Node {
        self.assert_single_output("ceil");
        self.output(0).ceil()
    }

    #[inline]
    #[track_caller]
    fn floor(&self) -> Node {
        self.assert_single_output("floor");
        self.output(0).floor()
    }

    #[inline]
    #[track_caller]
    fn round(&self) -> Node {
        self.assert_single_output("round");
        self.output(0).round()
    }

    #[inline]
    #[track_caller]
    fn trunc(&self) -> Node {
        self.assert_single_output("trunc");
        self.output(0).trunc()
    }

    #[inline]
    #[track_caller]
    fn fract(&self) -> Node {
        self.assert_single_output("fract");
        self.output(0).fract()
    }

    #[inline]
    #[track_caller]
    fn recip(&self) -> Node {
        self.assert_single_output("recip");
        self.output(0).recip()
    }

    #[inline]
    #[track_caller]
    fn signum(&self) -> Node {
        self.assert_single_output("signum");
        self.output(0).signum()
    }

    #[inline]
    #[track_caller]
    fn max(&self, b: impl IntoOutput) -> Node {
        self.assert_single_output("max");
        self.output(0).max(b)
    }

    #[inline]
    #[track_caller]
    fn min(&self, b: impl IntoOutput) -> Node {
        self.assert_single_output("min");
        self.output(0).min(b)
    }

    #[inline]
    #[track_caller]
    fn clamp(&self, min: impl IntoOutput, max: impl IntoOutput) -> Node {
        self.assert_single_output("clamp");
        self.output(0).clamp(min, max)
    }

    #[inline]
    #[track_caller]
    fn cast<T: Signal + CastTo<U> + Default, U: Signal + Default>(&self) -> Node {
        self.assert_single_output("cast");
        self.output(0).cast::<T, U>()
    }

    #[inline]
    #[track_caller]
    fn some(&self) -> Node {
        self.assert_single_output("some");
        self.output(0).some()
    }

    #[inline]
    #[track_caller]
    fn unwrap_or(&self, b: impl IntoOutput) -> Node {
        self.assert_single_output("unwrap_or");
        self.output(0).unwrap_or(b)
    }
}
