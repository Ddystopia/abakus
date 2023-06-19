#![recursion_limit = "1024"]

pub mod peano {
    // ========= Natural ==========

    use std::marker::PhantomData;

    pub struct Z;
    pub struct S<N: Nat>(PhantomData<N>);

    pub trait Nat {}

    impl Nat for Z {}

    impl<N: Nat> Nat for S<N> {}

    // ========= Saturating Sub ==========

    pub trait Sub {
        type Output: Nat;
    }

    impl Sub for (Z, Z) {
        type Output = Z;
    }

    impl<N: Nat> Sub for (S<N>, S<Z>) {
        type Output = N;
    }

    impl<N: Nat> Sub for (Z, S<N>) {
        type Output = Z;
    }

    impl<N: Nat> Sub for (S<N>, Z) {
        type Output = S<N>;
    }

    impl<N1: Nat, N2: Nat> Sub for (S<N1>, S<S<N2>>)
    where
        (S<N1>, S<N2>): Sub,
        (<(S<N1>, S<N2>) as Sub>::Output, S<Z>): Sub,
    {
        type Output = <(<(S<N1>, S<N2>) as Sub>::Output, S<Z>) as Sub>::Output;
    }

    // ========= Add ==========

    pub trait Add {
        type Output: Nat;
    }

    impl Add for (Z, Z) {
        type Output = Z;
    }

    impl<N: Nat> Add for (Z, S<N>) {
        type Output = S<N>;
    }

    impl<N: Nat> Add for (S<N>, Z) {
        type Output = S<N>;
    }

    impl<N1: Nat, N2: Nat> Add for (S<N1>, S<N2>)
    where
        (N1, N2): Add,
    {
        type Output = S<S<<(N1, N2) as Add>::Output>>;
    }

    // ========= Mul ==========

    pub trait Mul {
        type Output: Nat;
    }

    impl Mul for (Z, Z) {
        type Output = Z;
    }

    impl<N: Nat> Mul for (Z, S<N>) {
        type Output = Z;
    }

    impl<N: Nat> Mul for (S<N>, Z) {
        type Output = Z;
    }

    impl<N1: Nat, N2: Nat> Mul for (S<N1>, S<N2>)
    where
        (N1, S<N2>): Mul,
        (S<N2>, <(N1, S<N2>) as Mul>::Output): Add,
    {
        type Output = <(S<N2>, <(N1, S<N2>) as Mul>::Output) as Add>::Output;
    }
}

pub mod list {
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
}
