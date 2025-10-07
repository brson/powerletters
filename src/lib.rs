//! # Power Letters for Rust
//!
//! Concise spellings of common Rust operations:
//!
//! - `C` - `Clone`
//! - `O` - `ToOwned`
//! - `S` - `ToString`
//! - `I` - Ignore `Result`
//! - `X` - `expect` for `Result` and `Option`
//!
//! All operations are provided as both functions and methods.
//! Sometimes one reads better than the other.
//!
//!
//! ### Power `Clone`
//!
//! ```
//! use powerletters::*;
//!
//! let bagostuff = vec!["a", "b", "c"];
//! let newbag = bagostuff.C();
//!
//! // or
//! let newbag = C(&bagostuff);
//! ```
//!
//!
//! ### Power `ToOwned`
//!
//! ```
//! use powerletters::*;
//! use std::path::Path;
//!
//! let yourpath = Path::new("chill");
//! let mypath = yourpath.O();
//!
//! // or
//! let mypath = O(yourpath);
//! ```
//!
//!
//! ### Power `ToString`
//!
//! ```
//! use powerletters::*;
//!
//! let s: String = S("foo");
//!
//! // or
//! let s: String = "foo".S();
//! ```
//!
//!
//! ### Power ignore `Result` - kick that `Result` to the curb!
//!
//! ```
//! use powerletters::*;
//! # use std::io::{self, Write};
//! # fn write_logline(s: &str) -> io::Result<()> { Ok(()) }
//! # let logline = "test";
//!
//! write_logline(&logline).I();
//!
//! // or
//! I(write_logline(&logline));
//! ```
//!
//! Note this is superior to `let _ = ...`
//! because the `let` version is untyped and can
//! introduce unintended bugs like ignoring futures.
//!
//!
//! ### Power `expect` for `Result` and `Option`.
//!
//! ```
//! use powerletters::*;
//! # use std::io;
//!
//! let maybe_thing = Some("thing");
//! let thing = maybe_thing.X(); // like `.expect("some baloney")`
//! let good_thing: Result<_, io::Error> = Ok("thing");
//! let thing = good_thing.X();
//!
//! // or
//! let maybe_thing = Some("thing");
//! let thing = X(maybe_thing);
//! let good_thing: Result<_, io::Error> = Ok("thing");
//! let thing = X(good_thing);
//! ```

/// Power [`Clone`].
#[allow(non_snake_case)]
pub fn C<T>(o: &T) -> T
where
    T: Clone,
{
    o.clone()
}

/// Power [`ToOwned`].
#[allow(non_snake_case)]
pub fn O<T>(o: &T) -> T::Owned
where
    T: ToOwned + ?Sized,
{
    ToOwned::to_owned(o)
}

/// Power [`ToString`].
#[allow(non_snake_case)]
pub fn S<T>(s: &T) -> String
where
    T: ToString + ?Sized,
{
    ToString::to_string(s)
}

/// Power ignore [`Result`] - kick that `Result` to the curb!
#[allow(non_snake_case)]
pub fn I<T, E>(r: Result<T, E>) {
    let _ = r;
}

/// Power `expect` for [`Result`] and [`Option`].
#[track_caller]
#[allow(non_snake_case)]
pub fn X<T>(v: T) -> T::Output
where
    T: PowerExpect,
{
    v.X()
}

//                //
// -------------- //
//                //

// ^ those are barbells, for power


/// Power [`Clone`].
pub trait PowerClone {
    #[allow(non_snake_case)]
    fn C(&self) -> Self;
}

impl<T> PowerClone for T
where
    T: Clone,
{
    #[allow(non_snake_case)]
    fn C(&self) -> T {
        self.clone()
    }
}

/// Power [`ToOwned`].
pub trait PowerToOwned {
    type Owned;
    #[allow(non_snake_case)]
    fn O(&self) -> Self::Owned;
}

impl<T> PowerToOwned for T
where
    T: ToOwned + ?Sized,
{
    type Owned = T::Owned;

    #[allow(non_snake_case)]
    fn O(&self) -> Self::Owned {
        ToOwned::to_owned(self)
    }
}

/// Power [`ToString`].
pub trait PowerToString {
    #[allow(non_snake_case)]
    fn S(&self) -> String;
}

impl<T> PowerToString for T
where
    T: ToString + ?Sized,
{
    #[allow(non_snake_case)]
    fn S(&self) -> String {
        ToString::to_string(self)
    }
}

/// Power ignore [`Result`] - kick that `Result` to the curb!
pub trait ResultIgnore {
    #[allow(non_snake_case)]
    fn I(self);
}

impl<T, E> ResultIgnore for Result<T, E> {
    #[track_caller]
    #[allow(non_snake_case)]
    fn I(self) {
        let _ = self;
    }
}

/// Power `expect` for [`Result`] and [`Option`].
pub trait PowerExpect {
    type Output;
    #[allow(non_snake_case)]
    fn X(self) -> Self::Output;
}

impl<T> PowerExpect for Option<T> {
    type Output = T;

    #[track_caller]
    fn X(self) -> T {
        match self {
            Some(v) => v,
            None => panic!("impossible `None` option"),
        }
    }
}

impl<T, E: std::error::Error> PowerExpect for Result<T, E> {
    type Output = T;

    #[track_caller]
    fn X(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => panic!("impossible `Err` result: {e}"),
        }
    }
}
