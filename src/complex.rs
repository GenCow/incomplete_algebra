use crate::tree::Fold;

use crate::bin_struct;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Default)]
struct Complex {
    re: f64,
    im: f64,
}

impl<'a> Fold<Complex> for &'a Complex {
    #[inline]
    fn fold(&mut self) -> &Complex {
        &self
    }
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

    let mut c = &a + &b + (&a + &a) + &a;

    assert_eq!(c.fold(), &Complex { re: 6.0, im: 7.0 });

    let mut c = &a + &b - (&a + &a) + &a;

    assert_eq!(c.fold(), &Complex { re: 2.0, im: 3.0 });

    let mut c = &a + &b + &a - &a + &a;

    assert_eq!(c.fold(), &Complex { re: 4.0, im: 5.0 });
}
