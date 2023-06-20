use crate::{
    list::{Cons, List},
    peano::{Nat, S, Z},
};

// ======================= Abacus ========================

// ======================= Inc ===========================

pub trait AbInc {
    type Output: List;
}

impl<Car, Cdr> AbInc for (Cons<Car, Cdr>, Z)
where
    Car: Nat,
    Cdr: List,
{
    type Output = Cons<S<Car>, Cdr>;
}

impl<Car, Cdr, N: Nat> AbInc for (Cons<Car, Cdr>, S<N>)
where
    Car: Nat,
    (Cdr, N): AbInc,
{
    type Output = Cons<Car, <(Cdr, N) as AbInc>::Output>;
}

// ======================= Dec ===========================

pub trait AbDec {
    type Output: List;
}

impl<Cdr> AbDec for (Cons<Z, Cdr>, Z)
where
    Cdr: List,
{
    type Output = Cons<S<Z>, Cdr>;
}

impl<N, Cdr> AbDec for (Cons<S<N>, Cdr>, Z)
where
    N: Nat,
    Cdr: List,
{
    type Output = Cons<N, Cdr>;
}

impl<Car, Cdr, N: Nat> AbDec for (Cons<Car, Cdr>, S<N>)
where
    Car: Nat,
    (Cdr, N): AbDec,
{
    type Output = Cons<Car, <(Cdr, N) as AbDec>::Output>;
}

// ======================= Cycle ===========================

pub trait Func {
    type Output: List;
}

pub trait AbCycle {
    type Output: List;
}

impl<R, RN, F> AbCycle for (R, RN, F)
where
    (R, RN, F, R, RN): AbCycleHelper,
{
    type Output = <(R, RN, F, R, RN) as AbCycleHelper>::Output;
}

pub trait AbCycleHelper {
    type Output: List;
}

// Find register to cycle on
impl<Car, Cdr, R, F, N, RN> AbCycleHelper for (R, RN, F, Cons<Car, Cdr>, S<N>)
where
    R: List,
    N: Nat,
    (F, R): Func,
    (R, RN, F, Cdr, N): AbCycleHelper,
{
    type Output = <(R, RN, F, Cdr, N) as AbCycleHelper>::Output;
}

// Base case: 0 iterations, nothing to to
impl<Cdr, R, F, RN> AbCycleHelper for (R, RN, F, Cons<Z, Cdr>, Z)
where
    R: List,
    (F, R): Func,
{
    type Output = R;
}

// Do the cycle
impl<Cdr, R, F, N, RN> AbCycleHelper for (R, RN, F, Cons<S<N>, Cdr>, Z)
where
    R: List,
    N: Nat,
    (F, R): Func,
    (
        <(F, R) as Func>::Output,
        RN,
        F,
        <(F, R) as Func>::Output,
        RN,
    ): AbCycleHelper,
{
    type Output = <(
        <(F, R) as Func>::Output,
        RN,
        F,
        <(F, R) as Func>::Output,
        RN,
    ) as AbCycleHelper>::Output;
}
