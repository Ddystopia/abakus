#![recursion_limit = "1024"]

mod abacus;
mod list;
mod peano;

use abacus::{Cycle, Dec, Func, Inc};
use crate::list::{Cons, List, Nil};
use crate::peano::{S, Z};

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

type M1 = <(Registers, S<Z>) as Inc>::Output;
type M2 = <(M1, S<Z>) as Inc>::Output;

struct M3Body;

impl<R> Func for (M3Body, R)
where
    R: List,
    (R, Z): Inc,
    (<(R, Z) as Inc>::Output, S<S<Z>>): Inc,
    (<(<(R, Z) as Inc>::Output, S<S<Z>>) as Inc>::Output, S<Z>): Dec,
{
    type Output = <(<(<(R, Z) as Inc>::Output, S<S<Z>>) as Inc>::Output, S<Z>) as Dec>::Output;
}

type M3 = <(M2, S<Z>, M3Body) as Cycle>::Output;

type Computation = M3;

#[cfg(feature = "pretty-print")]
mod pretty_print {
    use crate::list::{Cons, Nil};
    use crate::peano::{Nat, S, Z};

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
    #[cfg(feature = "pretty-print")]
    {
        use pretty_print::ToString;
        println!("{}", <Computation as ToString>::to_string());
    }
    #[cfg(not(feature = "pretty-print"))]
    {
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
