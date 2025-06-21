# type-const

This crate provides simple utils for passing `const` values around through generics,
in the form of types.

The concept of a type const is expressed through the [`Const`] trait, which holds the type and
the value of the constant.

Passing values that are known at compile time through generics is different from passing
them through arguments, for example:
```rust
const fn array_of_const<C: type_const::Const, const N: usize>() -> [C::Type; N] {
    [C::VALUE; N] // no `Copy` needed!
}
assert_eq!(array_of_const::<type_const::DefaultOf<i32>, 3>(), [0; 3]);
```

This may also be used to write `const` "functions" in traits without the nightly
`const_trait` feature. Note that unlike `const fn`, these can only be evaluated at compile
time.
```rust
trait AddConst<Rhs=Self> {
    type Output;
    type Add<
        LhsC: type_const::Const<Type = Self>,
        RhsC: type_const::Const<Type = Rhs>,
    >: Const<Type = Self::Output>;
}
```

License: MIT
