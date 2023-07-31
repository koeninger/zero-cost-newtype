#[allow(unused_imports)]
#[macro_use] extern crate macro_attr_2018;
pub use macro_attr_2018::macro_attr;
#[allow(unused_imports)]
#[macro_use] extern crate newtype_derive_2018;
pub use newtype_derive_2018::*;

// Release mode, just a type alias, cast is a no-op
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! newtype {
    {
        $(#[$($attrs:tt)*])*
        $n:ident: $t:ty
    } => {
        pub type $n = $t;

        #[macro_export]
        macro_rules! $n {
            ($x:expr) => {
                $x
            };
        }
        // skip derives in release mode;
        // it wouldn't work for primitive types.
    };
}

#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! cast {
    ($x:expr) => {
        $x
    };
}

// Development mode, tuple struct, cast extracts the underlying type
#[cfg(debug_assertions)]
#[allow(unused_macros)]
#[macro_export]
macro_rules! newtype {
    {
        $(#[$($attrs:tt)*])*
        $n:ident: $t:ty
    } => {
        macro_attr! {
            $(#[$($attrs)*])*
            #[repr(transparent)]
            pub struct $n(pub $t);
        }

        #[macro_export]
        macro_rules! $n {
            ($x:expr) => {
                $n($x)
            };
        }
    };
}

#[cfg(debug_assertions)]
#[allow(unused_macros)]
#[macro_export]
macro_rules! cast {
    ($x:expr) => {
        ($x).0
    };
}

#[cfg(test)]
mod tests {
    mod test_types {
        #[allow(unused_imports)]
        use serde::{Deserialize, Serialize};
        newtype!{
            #[derive(Debug, Clone, Copy, PartialEq, NewtypeAdd!, NewtypeDisplay!, Serialize, Deserialize)]
            Price: i32
        }
    }

    #[test]
    fn it_works() {
        use test_types::*;
        use crate::*;
        let p: Price = Price!(23);
        let p2 = Price!(7);
        assert_eq!(p + p2, Price!(30));

        // won't compile in debug mode
        // let result = 42 + p;

        let result = 42 + cast!(p);
        assert_eq!(result, 65);
    }

    #[test]
    fn serde_ok() {
        use test_types::*;
        use crate::*;
        let p: Price = Price!(23);
        let j = serde_json::to_string(&p).unwrap();
        let d: Price = serde_json::from_str(&j).unwrap();
        assert_eq!(p, d);
    }

}
