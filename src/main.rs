#![recursion_limit = "1024"]

mod peano;
mod list;
mod abacus;

use abacus::{AbInc, Func, AbDec, AbCycle};

use crate::{
    list::{Cons, List, Nil},
    peano::{S, Z},
};

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
    ): AbDec
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
    use crate::peano::{Nat, S, Z};
    use crate::list::{Cons, Nil};

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
