use object_pool::Pool;
use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;

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

pub struct DynamicTree<O> {
    pub op1: Box<dyn Fold<O>>,
    pub op2: Box<dyn Fold<O>>,
    pub f: Box<dyn Operator<O>>,
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

pub trait Law<O> {
    fn call(&self, rop: &mut O, op: &Vec<O>);
}
