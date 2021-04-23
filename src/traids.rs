/// Based in [Haskell Functor](https://wiki.haskell.org/Functor)
///
/// # Example
///
/// ```
/// #![allow(incomplete_features)]
/// #![feature(generic_associated_types)]
/// use ca_comonad::traids::Functor;
///
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum Maybe<T> {
///    Just(T),
///    Nothing,
/// }
///
/// impl<A> Functor<A> for Maybe<A> {
///    type F<T> = Maybe<T>;
///
///    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::F<B> {
///        match self {
///            Maybe::Just(x) => Maybe::Just(f(x)),
///            Maybe::Nothing => Maybe::Nothing,
///        }
///    }
/// }
///
///     let j = Maybe::Just(1);
///     let n = Maybe::Nothing;
///     let f = |x| x+1;
///
///     assert_eq!(j.fmap(f), Maybe::Just(2));
///     assert_eq!(n.fmap(f), Maybe::Nothing);
///
/// ```
pub trait Functor<A> {
    type F<T>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::F<B>;
}

