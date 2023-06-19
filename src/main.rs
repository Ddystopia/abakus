use abakus::{
    list::{Cons, List, Nil},
    peano::{Add, Nat, Sub, S, Z},
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
    (Car, S<Z>): Add,
{
    type Output = Cons<<(Car, S<Z>) as Add>::Output, Cdr>;
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

impl<Car, Cdr> AbDec for (Cons<Car, Cdr>, Z)
where
    Car: Nat,
    Cdr: List,
    (Car, S<Z>): Sub,
{
    type Output = Cons<<(Car, S<Z>) as Sub>::Output, Cdr>;
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

// ======================= Program ===========================

/*
    a1
    a1
    (
        a0
        a2
        s1
    )1
*/

type Registers = Cons<Z, Cons<Z, Cons<Z, Nil>>>;

type M1 = <(Registers, S<Z>) as AbInc>::Output;
type M2 = <(M1, S<Z>) as AbInc>::Output;

struct M3Body;

impl<R> Func for (M3Body, R)
where
    R: List,
    (R, Z): AbInc,
    (<(R, Z) as AbInc>::Output, S<S<Z>>): AbInc,
    (
        <(<(R, Z) as AbInc>::Output, S<S<Z>>) as AbInc>::Output,
        S<Z>,
    ): AbDec,
{
    type Output = <(
        <(<(R, Z) as AbInc>::Output, S<S<Z>>) as AbInc>::Output,
        S<Z>,
    ) as AbDec>::Output;
}

type M3 = <(M2, S<Z>, M3Body) as AbCycle>::Output;

type Computation = M3;

#[cfg(feature = "pretty-print")]
mod pretty_print {
    use abakus::peano::{Nat, S, Z};
    use abakus::list::{Cons, Nil};

    pub trait ToString {
        fn to_string() -> String;
    }

    impl ToString for Nil {
        fn to_string() -> String {
            "Nil".to_string()
        }
    }

    impl<Car, Cdr> ToString for Cons<Car, Cdr>
    where
        Car: ToString,
        Cdr: ToString,
    {
        fn to_string() -> String {
            format!("{}, {}", Car::to_string(), Cdr::to_string())
        }
    }

    impl ToString for Z {
        fn to_string() -> String {
            "0".to_string()
        }
    }

    impl<N: Nat> ToString for S<N>
    where
        N: ToString,
    {
        fn to_string() -> String {
            format!("{}", N::to_string().parse::<u32>().unwrap() + 1)
        }
    }
}

fn main() {
    #[cfg(feature = "pretty-print")] {
        use pretty_print::ToString;
        println!("{}", <Computation as ToString>::to_string());
    } #[cfg(not(feature = "pretty-print"))] {
        println!(
            "{}",
            std::any::type_name::<Computation>()
                .replace("ttti_rs::", "")
                .replace("types::", "")
                .replace("abakus::", "")
                .replace("list::", "")
                .replace("peano::", "")
        );
    }
}
