The newtype pattern in Rust won't always generate the same code as the equivalent primitive type, https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html

This library provides macros to work around that.  It generates a typesafe tuple struct in debug mode, and a type alias in release mode which should perform exactly the same as the primitive type.

All of the operators from [newtype_derive](https://docs.rs/newtype_derive/0.1.6/newtype_derive/index.html#overview) are available.

Example:

```
// you want everything in scope because the macros use custom_derive
use zero_cost_newtype::*;

// in debug mode this is `struct Price(i32)`, in release mode `type Price = i32`
newtype! { Price i32 [Debug, Clone, Copy, PartialEq, NewtypeAdd] }

// in debug mode this is `= Price(23)`, in release mode `= 23` 
let p: Price = Price!(23);
let p2 = Price!(7);

assert_eq!(p + p2, Price!(30));

// won't compile in debug mode, can't accidentally add unrelated types.
// in release mode it would compile,  but you're safe as
// long as you compiled once in debug during development.
//
// let result = 42 + p;

// use cast! rather than .0 or .into or .from; it's a no-op in release mode.
let result = 42 + cast!(p);
assert_eq!(result, 65);
```
