# even-odd

For when modulo is just too hard

## Installation

Install with cargo.

`cargo add even-odd-traits`

## Usage

Abstract away remainder logic when checking even-ness or odd-ness! Pre-imlpemented for primitive numeric types.

```rust
assert!(0.is_even());
assert!(!0.is_odd());

assert!(!1.is_even());
assert!(1.is_odd());

assert!(2.is_even());
assert!(!2.is_odd());
```

Simply bring the `IsEven` and/or `IsOdd` trait(s) into scope:

```rust
use even_odd::{IsEven, IsOdd};
```

And then call the `is_even` and/or `is_odd` method(s) on any type for which it/they is/are implemented.

## Credit

Heavily inspired by [is-odd](https://github.com/nukeop/is-odd), with even functionality added.
