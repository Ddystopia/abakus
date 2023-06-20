use crate::{
    list::{Cons, List},
    peano::{Nat, S, Z},
};

// ======================= Abacus ========================

// ======================= Inc ===========================

pub trait Inc {
    type Output: List;
}

impl<Car, Cdr> Inc for (Cons<Car, Cdr>, Z)
where
    Car: Nat,
    Cdr: List,
{
    type Output = Cons<S<Car>, Cdr>;
}

impl<Car, Cdr, N: Nat> Inc for (Cons<Car, Cdr>, S<N>)
where
    Car: Nat,
    (Cdr, N): Inc,
{
    type Output = Cons<Car, <(Cdr, N) as Inc>::Output>;
}

// ======================= Dec ===========================

pub trait Dec {
    type Output: List;
}

impl<Cdr> Dec for (Cons<Z, Cdr>, Z)
where
    Cdr: List,
{
    type Output = Cons<S<Z>, Cdr>;
}

impl<N, Cdr> Dec for (Cons<S<N>, Cdr>, Z)
where
    N: Nat,
    Cdr: List,
{
    type Output = Cons<N, Cdr>;
}

impl<Car, Cdr, N: Nat> Dec for (Cons<Car, Cdr>, S<N>)
where
    Car: Nat,
    (Cdr, N): Dec,
{
    type Output = Cons<Car, <(Cdr, N) as Dec>::Output>;
}

// ======================= Cycle ===========================

pub trait Func {
    type Output: List;
}

pub trait Cycle {
    type Output: List;
}

impl<R, RN, F> Cycle for (R, RN, F)
where
    (R, RN, F, R, RN): CycleHelper,
{
    type Output = <(R, RN, F, R, RN) as CycleHelper>::Output;
}

pub trait CycleHelper {
    type Output: List;
}

// Find register to cycle on
impl<Car, Cdr, R, F, N, RN> CycleHelper for (R, RN, F, Cons<Car, Cdr>, S<N>)
where
    R: List,
    N: Nat,
    (F, R): Func,
    (R, RN, F, Cdr, N): CycleHelper,
{
    type Output = <(R, RN, F, Cdr, N) as CycleHelper>::Output;
}

// Base case: 0 iterations, nothing to to
impl<Cdr, R, F, RN> CycleHelper for (R, RN, F, Cons<Z, Cdr>, Z)
where
    R: List,
    (F, R): Func,
{
    type Output = R;
}

// Do the cycle
impl<Cdr, R, F, N, RN> CycleHelper for (R, RN, F, Cons<S<N>, Cdr>, Z)
where
    R: List,
    N: Nat,
    (F, R): Func,
    (<(F, R) as Func>::Output, RN, F): Cycle,
{
    type Output = <(<(F, R) as Func>::Output, RN, F) as Cycle>::Output;
}
