use std::marker::PhantomData;

pub trait List {}

pub struct Nil;
pub struct Cons<Car, Cdr>(PhantomData<(Car, Cdr)>);

impl List for Nil {}

impl<Car, Cdr> List for Cons<Car, Cdr> where Cdr: List {}

// ========= Concat =========

pub trait Concat {
    type Output: List;
}

impl<L2: List> Concat for (Nil, L2) {
    type Output = L2;
}

impl<Car, Cdr, L2> Concat for (Cons<Car, Cdr>, L2)
where
    L2: List,
    (Cdr, L2): Concat, // Recursive step
{
    type Output = Cons<Car, <(Cdr, L2) as Concat>::Output>;
}
