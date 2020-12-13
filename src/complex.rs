use crate::tree::{Fold, Operator, StaticTree};
use object_pool::Pool;
use std::borrow::Borrow;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Default)]
struct Complex {
    re: f64,
    im: f64,
}

impl<'a> Operator<Complex> for &'a Complex {
    #[inline]
    fn call(&self, rop: &mut Complex, _: &Complex, _: &Complex) {
        (*rop).clone_from(self);
    }
}

impl<'a> Fold<Complex> for &'a Complex {
    #[inline]
    fn fold(&mut self) -> &Complex {
        &self
    }
}

macro_rules! bin_op_tree_tree {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<A, B, C, D, E, F> $trait<StaticTree<A, B, C, $object_type>>
            for StaticTree<D, E, F, $object_type>
        where
            A: Operator<$object_type>,
            B: Fold<$object_type>,
            C: Fold<$object_type>,
            D: Operator<$object_type>,
            E: Fold<$object_type>,
            F: Fold<$object_type>,
        {
            type Output = StaticTree<
                $operator_type,
                StaticTree<D, E, F, $object_type>,
                StaticTree<A, B, C, $object_type>,
                $object_type,
            >;

            fn $trait_method(self, rhs: StaticTree<A, B, C, $object_type>) -> Self::Output {
                StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}
macro_rules! bin_op_tree_object {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<'a, A, B, C> $trait<&'a $object_type> for StaticTree<A, B, C, $object_type>
        where
            A: Operator<$object_type>,
            B: Fold<$object_type>,
            C: Fold<$object_type>,
        {
            type Output = StaticTree<
                $operator_type,
                StaticTree<A, B, C, $object_type>,
                &'a $object_type,
                $object_type,
            >;

            fn $trait_method(self, rhs: &'a $object_type) -> Self::Output {
                StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}
macro_rules! bin_op_object_tree {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<'a, A, B, C> $trait<StaticTree<A, B, C, $object_type>> for &'a $object_type
        where
            A: Operator<$object_type>,
            B: Fold<$object_type>,
            C: Fold<$object_type>,
        {
            type Output = StaticTree<
                $operator_type,
                &'a $object_type,
                StaticTree<A, B, C, $object_type>,
                $object_type,
            >;

            fn $trait_method(self, rhs: StaticTree<A, B, C, $object_type>) -> Self::Output {
                StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}
macro_rules! bin_op_object_object {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<'a> $trait for &'a $object_type {
            type Output =
                StaticTree<$operator_type, &'a $object_type, &'a $object_type, $object_type>;

            fn $trait_method(self, rhs: &'a $object_type) -> Self::Output {
                StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! bin_op {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        bin_op_tree_tree!(
            $trait,
            $trait_method,
            $operator_type,
            $operator_constructor,
            $object_type
        );
        bin_op_tree_object!(
            $trait,
            $trait_method,
            $operator_type,
            $operator_constructor,
            $object_type
        );
        bin_op_object_tree!(
            $trait,
            $trait_method,
            $operator_type,
            $operator_constructor,
            $object_type
        );
        bin_op_object_object!(
            $trait,
            $trait_method,
            $operator_type,
            $operator_constructor,
            $object_type
        );
    };
}

#[macro_export]
macro_rules! bin_struct {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $object_type:ident,
    $rop:ident,
    $op1:ident,
    $op2:ident,
    $body:block) => {
        struct $operator_type();

        impl Operator<$object_type> for $operator_type {
            #[inline]
            fn call(&self, $rop:&mut $object_type, $op1:&$object_type, $op2:&$object_type) $body
        }

        bin_op!($trait, $trait_method, $operator_type, $operator_type(), $object_type);
    };
}

bin_struct!(Add, add, ComplexAdd, Complex, rop, op1, op2, {
    rop.re = op1.re + op2.re;
    rop.im = op1.im + op2.im
});

bin_struct!(Sub, sub, ComplexSub, Complex, rop, op1, op2, {
    rop.re = op1.re - op2.re;
    rop.im = op1.im - op2.im
});

#[test]
fn test() {
    let a = Complex { re: 1.0, im: 1.0 };
    let b = Complex { re: 2.0, im: 3.0 };
    let pool = Pool::new(10, || Complex { re: 0.0, im: 0.0 });

    let mut c = &a + &b + (&a + &a) + &a;

    assert_eq!(c.fold(), &Complex { re: 6.0, im: 7.0 });

    let mut c = &a + &b - (&a + &a) + &a;

    assert_eq!(c.fold(), &Complex { re: 2.0, im: 3.0 });

    let mut c = &a + &b + &a - &a + &a;

    assert_eq!(c.fold(), &Complex { re: 4.0, im: 5.0 });

    println!("te");
}
