#![allow(rustdoc::redundant_explicit_links)]

//! This crate provides simple utils for passing `const` values around through generics,
//! in the form of types.
//!
//! The concept of a type const is expressed through the [`Const`](crate::Const) trait, which holds the type and
//! the value of the constant.
//!
//! Passing values that are known at compile time through generics is different from passing
//! them through arguments, for example:
//! ```
//! const fn array_of_const<C: type_const::Const, const N: usize>() -> [C::Type; N] {
//!     [C::VALUE; N] // no `Copy` needed!
//! }
//! assert_eq!(array_of_const::<type_const::DefaultOf<i32>, 3>(), [0; 3]);
//! ```
//!
//! This may also be used to write `const` "functions" in traits without the nightly
//! `const_trait` feature. Note that unlike `const fn`, these can only be evaluated at compile
//! time.
//! ```
//! trait AddConst<Rhs=Self> {
//!     type Output;
//!     type Add<
//!         LhsC: type_const::Const<Type = Self>,
//!         RhsC: type_const::Const<Type = Rhs>,
//!     >: Const<Type = Self::Output>;
//! }
//! ```

/// Describes a type const.
pub trait Const {
    /// The value type of the const
    type Type;
    /// The implementation of the value. Prefer [`value_of`] for accessing this during const
    /// evaluation.
    const VALUE: Self::Type;
}

/// Alias for [`Const::Type`].
pub type TypeOf<C> = <C as Const>::Type;

/// Alias for [`Const::VALUE`]. Prefer this function over accessing the const directly.
///
/// Using the associated constant through this function rather than directly often causes it to only be
/// evaluated when the branch that it is used in is actually executed.
/// This means that it may improve compile times, avoid errors for recursive consts and avoid evaluating
/// panics.
///
/// For example:
/// ```compile_fail
/// # use type_const::*;
/// struct Fallible;
/// impl Const for Fallible {
///     type Type = ();
///     const VALUE: Self::Type = panic!();
/// }
/// const _: () = if false { Fallible::VALUE }; // this gives a compile error
/// ```
/// ```
/// # use type_const::*;
/// # struct Fallible;
/// # impl Const for Fallible {
/// #     type Type = ();
/// #     const VALUE: Self::Type = panic!();
/// # }
/// const _: () = if false { value_of::<Fallible>() };  // this compiles
/// ```
pub const fn value_of<C: Const + ?Sized>() -> C::Type {
    C::VALUE
}

/// A const version of [`Default`].
pub trait DefaultConst: Sized {
    /// The default value for this type. Prefer accessing this by calling [`value_of`] on [`DefaultOf<Self>`].
    const DEFAULT: Self;
}
/// A [`Const`] that evaluates to [`DefaultConst::DEFAULT`].
pub struct DefaultOf<T>(#[allow(dead_code)] fn() -> T);
impl<T: DefaultConst> Const for DefaultOf<T> {
    type Type = T;
    const VALUE: Self::Type = T::DEFAULT;
}

mod impls;
