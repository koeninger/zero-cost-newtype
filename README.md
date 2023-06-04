The newtype pattern in Rust won't always generate the same code as the equivalent primitive type, https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html

This library provides macros to work around that.  It generates a typesafe tuple struct in debug mode, and a type alias in release mode which should perform exactly the same as the primitive type.