pub mod term;
pub mod syntax;
use term::*;
use std::string::*;

fn get_result<T>(name : Vec<u8>, result : Result<T, String>) -> T {
    match result {
        Ok(term) => term,
        Err(err) => {
            println!("[Error on `{}`]\n{}", String::from_utf8_lossy(&name), err);
            std::process::exit(0);
        }
    }
}

fn main() {
    let (val, defs) = get_result(b"main".to_vec(), syntax::term_from_string_slice("
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
    "));
    println!("[Term]\n{}", syntax::term_to_string(&val, &mut Vec::new(), true));
    println!("");

    for (nam, def) in &defs {
        get_result(nam.to_vec(), syntax::infer_with_string_error(&def, &defs, false, true));
    }

    let mut typ : Term = get_result(b"main".to_vec(), syntax::infer_with_string_error(&val, &defs, false, true));
    reduce(&mut typ, &defs, true);
    println!("[Type]\n{}", syntax::term_to_string(&typ, &mut Vec::new(), true));
    println!("");

    let mut nor : Term = val.clone();
    reduce(&mut nor, &defs, true);
    println!("[Norm]\n{}", syntax::term_to_string(&nor, &mut Vec::new(), true));

}
