# Formality

An efficient programming language featuring formal proofs.

## Features

- **Formal proofs:** Formality's type system allows it to [prove theorems](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence) about its own programs.

- **Optimality:** Formality performs beta-reduction (lambda substitution) [optimally](https://www.amazon.com/Implementation-Functional-Programming-Languages-Theoretical/dp/0521621127).

- **No garbage-collection:** Formality doesn't require a garbage collection, making it resource-efficient.

- **EVM-compatibility:** Formality can be compiled to the EVM to run [Ethereum](https://www.ethereum.org/) smart-contracts.

- **GPU-compatibility:** Formality can also compile to CUDA / OpenCL and run in thousands of cores, making it *very fast*.

- **Simplicity:** The entire implementation is ~2k LOC and aims to be kept simple.

## How?

*Theorem proving* is possible thanks to dependent functions and inductive datatypes, similarly to [Coq](https://coq.inria.fr/refman/language/cic.html), [Agda](https://github.com/agda/agda) and other proof assistants. To guarantee mathematical meaningfulness, Formality is compiled to [Cedille-core](https://github.com/maiavictor/cedille-core), a minimalist type theory which acts as a termination and consistency checker.

*Optimality*, no *garbage-collection*, *EVM* and *GPU* compatibility are all possible due to compilation to the [symmetric interaction calculus](https://github.com/MaiaVictor/symmetric-interaction-calculus), a lightweight computing model that combines good aspects of the Turing Machine and the Lambda Calculus. In order for this to work, Formality enforces some compile-time restrictions based on Elementary Affine Logic.

## Example

Here are some random datatypes and functions to show the syntax, and a 1-LOC proof that `2 + 2 = 4`.

```haskell
data Empty : Type

data Unit : Type
| void    : Unit

data Bool : Type
| true    : Bool
| false   : Bool

data Nat : Type
| succ   : (n : Nat) -> Nat
| zero   : Nat

data Eq : (A : Type, x : A, y : A) -> Type
| refl  : (A : Type, x : A) -> Eq(A, x, x)

data Vect : (A : Type, n : Nat) -> Type
| cons    : (A : Type, n : Nat, x : A, xs : Vect(A, n)) -> Vect(A, Nat.succ(n))
| nil     : (A : Type) -> Vect(A, Nat.zero)

let the(P : Type, x : P) =>
    x

let not(b : Bool) =>
    case b
    | true  => Bool.false
    | false => Bool.true
    : Bool

let add(a : Nat, b : Nat) =>
    case a
    | succ(pred) => Nat.succ(add(pred, b))
    | zero       => b
    : Nat

let pred(a : Nat) =>
    case a
    | succ(pred) => pred
    | zero       => Nat.zero
    : Nat

let EFQ(P : Type, f : Empty) =>
    case f : P

let induction
    ( P : (n : Nat) -> Type
    , s : (n : Nat, p : P(n)) -> P(Nat.succ(n))
    , z : P(Nat.zero)
    , n : Nat) =>
    case n
    | succ(pred) => s(pred, induction(P, s, z, pred))
    | zero       => z
    : P(self)

let tail(A : Type, n : Nat, vect : Vect(A, Nat.succ(n))) =>
    case vect
    | cons(A, n, x, xs) => xs
    | nil(A)            => Vect.nil(A)
    : (A, n) => Vect(A, pred(n))

let head(A : Type, n : Nat, vect : Vect(A, Nat.succ(n))) =>
    case vect
    | cons(A, n, x, xs) => x
    | nil(A)            => Unit.void
    : (A, n) => case n
        | succ(m) => A
        | zero    => Unit
        : Type

let two
    Nat.succ(Nat.succ(Nat.zero))

let four
    Nat.succ(Nat.succ(Nat.succ(Nat.succ(Nat.zero))))

let two_plus_two_is_four
    the(Eq(Nat, add(two, two), four), Eq.refl(Nat, four))

two_plus_two_is_four
```

Soon, I'll explain how to prove cooler things, and write a tutorial on how to make a "DAO" Smart Contract that is provably "unhackable", in the sense its internal balance always matches the sum of its users balances.

## Done

- Formality syntax (parser / stringifier)

- Formality type checker

- Formality interpreter

- [Symmetric Interaction Calculus (syntax, runtime)](https://github.com/maiavictor/symmetric-interaction-calculus)

- [Cedille-core](https://github.com/maiavictor/cedille-core)

- [GPU evaluator prototype](https://github.com/maiavictor/absal-rs/tree/parallel-test-3) and [concept](https://github.com/maiavictor/absal-ex)

* Sans bugs, incremental improvements, minor missing features, etc.

## To do

- Elementary Affine Logic checks

- Cedille compilation (port to Rust?)

- Symmetric Interaction Calculus compilation and decompilation

- EVM compilation

- Complete CUDA / OpenCL evaluator
