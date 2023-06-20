# Abakus Machine

This is implementation of Turing-Complete Abacus Machine on the top of Rusts
typesystem.

## Example Program

```rust
/*
    a1
    a1
    (
        a0
        a2
        s1
    )1
*/


// Here we are defining registers to use. By the definition, Abakus Machine
// could have infinite abount of registers, but finite at the time of execution.
// Virtually it means, that we could choose arbitrary large finite number of
// registers.

type Registers = Cons<Z, Cons<Z, Cons<Z, Nil>>>;

// Takes registers and number of register to incremet
// Outputs modified registers
type M1 = <(Registers, S<Z>) as Inc>::Output;
// As you can see, it takes registers that are output from the last command.
type M2 = <(M1, S<Z>) as Inc>::Output;

// Defining type for body of out loop.
struct M3Body;

// Defining the body of out loop.
impl<R> Func for (M3Body, R)
where
    R: List, // Sanity check
    (R, Z): Inc, // Increment R0
    (<(R, Z) as Inc>::Output, S<S<Z>>): Inc, // Inc R2
    (<(<(R, Z) as Inc>::Output, S<S<Z>>) as Inc>::Output, S<Z>): Dec, // Dec R1
{
    // Computation is happening in the `where` clause, here we are just reading
    type Output = <(<(<(R, Z) as Inc>::Output, S<S<Z>>) as Inc>::Output, S<Z>) as Dec>::Output;
}

// Passing Registers, number of register to cycle on and body of the loop to
// the Cycle, and taking new registers from output.
//
// We could've been pasted this statement inside another loops body, so we
// could create nested loops.
type M3 = <(M2, S<Z>, M3Body) as Cycle>::Output;

type Computation = M3;

```

## Run

To run it, you must enter this command. This comes in at runtime to print
results. We could print them out with a compilation error, so it's not even
output the binary code, but that's a sign of schizophrenia.

```bash
cargo -q run
```

### Pretty-Print

To enable pretty printing of lists and numbers, use feature "pretty-print".

```bash
cargo -q run --features pretty-print
```

It will convert printed types on output like this

```rust
Cons<S<S<Z>>, Cons<Z, Cons<S<S<Z>>, Nil>>>
```

To string like this (at runtime)

```rust
2, 0, 2, Nil
```

## Thanks

I learned how to do type programming from that repo:
[insou22/typing-the-technical-interview-rust](https://github.com/insou22/typing-the-technical-interview-rust)
