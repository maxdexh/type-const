use crate::{Const, DefaultConst};

macro_rules! tuple_doc {
    ($count:literal $T:ident; $item:item) => {
        #[cfg_attr(docsrs, doc(fake_variadic))]
        #[cfg_attr(docsrs, doc = ::core::concat!("This trait is implemented for tuples up to ", $count, " items long."))]
        $item
    };
    ($count:literal $_:ident $($T:ident)+; $item:item) => {
        #[cfg_attr(docsrs, doc(hidden))]
        $item
    };
}
macro_rules! impl_tuple {
    ($count:literal $_:ident) => {};
    ($count:literal $_:ident $($T:ident)+) => {
        impl_tuple! { $count $($T)* }
        tuple_doc! {
            $count $($T)*;
            /// Tuples of [`Const`](crate::Const) evaluate to a tuple of the values
            impl<$($T: $crate::Const),*> $crate::Const for ($($T,)*) {
                type Type = ($($T::Type,)*);
                const VALUE: Self::Type = ($($T::VALUE,)*);
            }
        }
        impl<$($T: $crate::DefaultConst),*> $crate::DefaultConst for ($($T,)*) {
            const DEFAULT: Self = ($($T::DEFAULT,)*);
        }
    };
}
impl_tuple! {
    12
    __
    A B C
    D E F
    G H I
    J K L
}
impl Const for () {
    type Type = Self;
    const VALUE: Self::Type = ();
}
impl DefaultConst for () {
    const DEFAULT: Self = ();
}

/// `[Const; N]` evaluates to `[VALUE; N]`
impl<C: Const, const N: usize> Const for [C; N] {
    type Type = [C::Type; N];
    const VALUE: Self::Type = [C::VALUE; N];
}

/// ```rust_analyzer_brace_hint
/// defaults! {}
/// ```
macro_rules! defaults {
    (
        {$($generics:tt)*}
        $(#[$meta:meta])*
        $Self:ty = $default:expr $(;)?
    ) => {
        $(#[$meta])*
        impl<$($generics)*> $crate::DefaultConst for $Self {
            const DEFAULT: Self = $default;
        }
    };
    (
        $generics:tt
        $(
            $(#[$meta:meta])*
            $Self:ty = $default:expr
        );* $(;)?
    ) => {
        $(defaults! { $generics $(#[$meta])* $Self = $default })*
    };
}
defaults! {
    {T: DefaultConst, const N: usize}
    [T; N] = [T::DEFAULT; N]
}
defaults! {
    {T: DefaultConst}
    core::mem::ManuallyDrop<T> = Self::new(T::DEFAULT);
    core::cmp::Reverse<T> = Self(T::DEFAULT);
    core::num::Saturating<T> = Self(T::DEFAULT);
    core::num::Wrapping<T> = Self(T::DEFAULT);
    core::panic::AssertUnwindSafe<T> = Self(T::DEFAULT);
}
defaults! {
    {T}
    Option<T> = None;
    &[T] = &[];
}
defaults! {
    {T: ?Sized}
    core::marker::PhantomData<T> = Self
}
defaults! {
    {}
    &str = "";
    &core::ffi::CStr = c"";
    core::fmt::Error = Self;
    core::ops::RangeFull = Self;
    core::time::Duration = Self::ZERO;
}

macro_rules! nums {
    ($($t:ty),* $(,)?) => {$(
        defaults! {
            {}
            $t = 0 as _
        }
    )*};
}
nums! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
}
