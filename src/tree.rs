use std::marker::PhantomData;
use object_pool::Pool;
use std::borrow::{Borrow, BorrowMut};

struct StaticTree<F,A,B,O>
    where A:Fold<O>, B:Fold<O>, F:Function<O> {
    op1:A,
    op2:B,
    f:PhantomData<F>,
    b:PhantomData<O>
}

trait Function<O> {
    fn call(rop:&mut O, op1:&O, op2:&O);
}

trait Fold<O> {
    fn fold(&self, b:&mut O, pool:&Pool<O>);
}

impl<F,A,B,O> Fold<O> for StaticTree<F,A,B,O>
    where A:Fold<O>, B:Fold<O>, F:Function<O> {
    fn fold(&self, rop:&mut O, pool:&Pool<O>) {
        let mut op1 = pool.try_pull().unwrap();
        let mut op2 = pool.try_pull().unwrap();

        self.op1.fold(op1.borrow_mut(), pool);
        self.op2.fold(op2.borrow_mut(), pool);

        F::call(rop, op1.borrow(), op2.borrow());
    }
}