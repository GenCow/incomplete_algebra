pub struct StaticTree<F, A, B, O>
where
    A: Fold<O>,
    B: Fold<O>,
    F: Operator<O>,
{
    pub op1: A,
    pub op2: B,
    pub f: F,
    pub o: O,
}

pub trait Operator<O> {
    fn call(&self, rop: &mut O, op1: &O, op2: &O);
}

pub trait Fold<O> {
    fn fold(&mut self) -> &O;
}

impl<F, A, B, O> Fold<O> for StaticTree<F, A, B, O>
where
    A: Fold<O>,
    B: Fold<O>,
    F: Operator<O>,
{
    #[inline]
    fn fold(&mut self) -> &O {
        let op1 = self.op1.fold();
        let op2 = self.op2.fold();

        self.f.call(&mut self.o, op1, op2);

        &self.o
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bin_op_tree_tree {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<A, B, C, D, E, F> $trait<crate::tree::StaticTree<A, B, C, $object_type>>
            for crate::tree::StaticTree<D, E, F, $object_type>
        where
            A: crate::tree::Operator<$object_type>,
            B: crate::tree::Fold<$object_type>,
            C: crate::tree::Fold<$object_type>,
            D: crate::tree::Operator<$object_type>,
            E: crate::tree::Fold<$object_type>,
            F: crate::tree::Fold<$object_type>,
        {
            type Output = crate::tree::StaticTree<
                $operator_type,
                crate::tree::StaticTree<D, E, F, $object_type>,
                crate::tree::StaticTree<A, B, C, $object_type>,
                $object_type,
            >;

            fn $trait_method(
                self,
                rhs: crate::tree::StaticTree<A, B, C, $object_type>,
            ) -> Self::Output {
                crate::tree::StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bin_op_tree_object {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<'a, A, B, C> $trait<&'a $object_type>
            for crate::tree::StaticTree<A, B, C, $object_type>
        where
            A: crate::tree::Operator<$object_type>,
            B: crate::tree::Fold<$object_type>,
            C: crate::tree::Fold<$object_type>,
        {
            type Output = crate::tree::StaticTree<
                $operator_type,
                crate::tree::StaticTree<A, B, C, $object_type>,
                &'a $object_type,
                $object_type,
            >;

            fn $trait_method(self, rhs: &'a $object_type) -> Self::Output {
                crate::tree::StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bin_op_object_tree {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<'a, A, B, C> $trait<crate::tree::StaticTree<A, B, C, $object_type>>
            for &'a $object_type
        where
            A: crate::tree::Operator<$object_type>,
            B: crate::tree::Fold<$object_type>,
            C: crate::tree::Fold<$object_type>,
        {
            type Output = crate::tree::StaticTree<
                $operator_type,
                &'a $object_type,
                crate::tree::StaticTree<A, B, C, $object_type>,
                $object_type,
            >;

            fn $trait_method(
                self,
                rhs: crate::tree::StaticTree<A, B, C, $object_type>,
            ) -> Self::Output {
                crate::tree::StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bin_op_object_object {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $operator_constructor:expr,
    $object_type:ident) => {
        impl<'a> $trait for &'a $object_type {
            type Output = crate::tree::StaticTree<
                $operator_type,
                &'a $object_type,
                &'a $object_type,
                $object_type,
            >;

            fn $trait_method(self, rhs: &'a $object_type) -> Self::Output {
                crate::tree::StaticTree {
                    op1: self,
                    op2: rhs,
                    f: $operator_constructor,
                    o: Default::default(),
                }
            }
        }
    };
}

#[macro_export(local_inner_macros)]
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

#[macro_export(local_inner_macros)]
macro_rules! bin_struct {
    ($trait:ident,
    $trait_method:ident,
    $operator_type:ident,
    $object_type:ident,
    $rop:ident,
    $op1:ident,
    $op2:ident,
    $body:block) => {
        pub struct $operator_type();

        impl crate::tree::Operator<$object_type> for $operator_type {
            #[inline]
            fn call(&self, $rop:&mut $object_type, $op1:&$object_type, $op2:&$object_type) $body
        }

        bin_op!($trait, $trait_method, $operator_type, $operator_type(), $object_type);
    };
}
