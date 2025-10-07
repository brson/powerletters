//! Concise spellings of common Rust operations.

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

/// Power `expect`.
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
#[extension_trait::extension_trait]
pub impl<T> PowerClone<T> for T
where
    T: Clone,
{
    #[allow(non_snake_case)]
    fn C(&self) -> T {
        self.clone()
    }
}

/// Power [`ToOwned`].
#[extension_trait::extension_trait]
pub impl<T> PowerToOwned for T
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
#[extension_trait::extension_trait]
pub impl<T> PowerToString for T
where
    T: ToString + ?Sized,
{
    #[allow(non_snake_case)]
    fn S(&self) -> String {
        ToString::to_string(self)
    }
}

/// Power ignore [`Result`] - kick that `Result` to the curb!
#[extension_trait::extension_trait]
pub impl<T, E> ResultIgnore<T, E> for Result<T, E> {
    #[track_caller]
    #[allow(non_snake_case)]
    fn I(self) {
        let _ = self;
    }
}

/// Power `expect`.
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
