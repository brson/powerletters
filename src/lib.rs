//! Concise and visually distinct spellings of common Rust operations.

/// To `String`.
#[allow(non_snake_case)]
pub fn S<T>(s: &T) -> String
where
    T: ToString + ?Sized,
{
    ToString::to_string(s)
}

/// Clone.
#[allow(non_snake_case)]
pub fn C<T>(o: T) -> T
where
    T: Clone,
{
    o.clone()
}

/// To owned.
#[allow(non_snake_case)]
pub fn O<T>(o: &T) -> T::Owned
where
    T: ToOwned + ?Sized,
{
    ToOwned::to_owned(o)
}

/// Expect ok or some.
#[track_caller]
#[allow(non_snake_case)]
pub fn X<T>(v: T) -> T::Output
where
    T: QuickExpect,
{
    v.X()
}

/// Ignore result.
#[allow(non_snake_case)]
pub fn I<T, E>(r: Result<T, E>) {
    let _ = r;
}

/// Helper trait for the `X` function.
pub trait QuickExpect {
    type Output;
    #[allow(non_snake_case)]
    fn X(self) -> Self::Output;
}

impl<T> QuickExpect for Option<T> {
    type Output = T;

    #[track_caller]
    fn X(self) -> T {
        match self {
            Some(v) => v,
            None => panic!("impossible `None` option"),
        }
    }
}

impl<T, E: std::error::Error> QuickExpect for Result<T, E> {
    type Output = T;

    #[track_caller]
    fn X(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => panic!("impossible `Err` result: {e}"),
        }
    }
}

/// To `String`.
#[extension_trait::extension_trait]
pub impl<T> QuickToString for T
where
    T: ToString + ?Sized,
{
    #[allow(non_snake_case)]
    fn S(&self) -> String {
        ToString::to_string(self)
    }
}

/// Clone.
#[extension_trait::extension_trait]
pub impl<T> QuickClone<T> for T
where
    T: Clone,
{
    #[allow(non_snake_case)]
    fn C(&self) -> T {
        self.clone()
    }
}

/// To owned.
#[extension_trait::extension_trait]
pub impl<T> QuickToOwned for T
where
    T: ToOwned,
{
    type Owned = T::Owned;

    #[allow(non_snake_case)]
    fn O(&self) -> Self::Owned {
        ToOwned::to_owned(self)
    }
}

/// Expect ok or some.
#[extension_trait::extension_trait]
pub impl<T> OptionExpect<T> for Option<T> {
    #[track_caller]
    #[allow(non_snake_case)]
    fn X(self) -> T {
        match self {
            Some(v) => v,
            None => panic!("impossible `None` option"),
        }
    }
}

/// Expect ok or some.
#[extension_trait::extension_trait]
pub impl<T, E> ResultExpect<T, E> for Result<T, E>
where
    E: std::error::Error,
{
    #[track_caller]
    #[allow(non_snake_case)]
    fn X(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => panic!("impossible `Err` result: {e}"),
        }
    }
}

/// Ignore a `Result`.
///
/// This is nice because the common idiom of `let _ = ...` is untyped
/// and can accidentally be applied to non-`Result` types like `Future`s.
#[extension_trait::extension_trait]
pub impl<T, E> ResultIgnore<T, E> for Result<T, E> {
    #[track_caller]
    #[allow(non_snake_case)]
    fn I(self) {
        let _ = self;
    }
}
