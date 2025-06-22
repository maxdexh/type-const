//! A small collection of useful consts.

use std::mem::MaybeUninit;

use crate::Const;

/// Evaluates to [`Some(C::VALUE)`](Option)
pub struct IntoSome<C>(C);
impl<C: Const> Const for IntoSome<C> {
    type Type = Option<C::Type>;
    const VALUE: Self::Type = Some(C::VALUE);
}

/// Evaluates to [`MaybeUninit::<T>::uninit()`](MaybeUninit)
pub struct UninitOf<T>(T);
impl<T> Const for UninitOf<T> {
    type Type = MaybeUninit<T>;
    const VALUE: Self::Type = MaybeUninit::uninit();
}

/// Evaluates to [`MaybeUninit::new(C::VALUE)`](MaybeUninit)
pub struct IntoInit<C>(C);
impl<C: Const> Const for IntoInit<C> {
    type Type = MaybeUninit<C::Type>;
    const VALUE: Self::Type = MaybeUninit::new(C::VALUE);
}
