#[allow(unused_imports)]
#[macro_use] extern crate newtype_derive;
#[allow(unused_imports)]
#[macro_use] extern crate custom_derive;


// Release mode, just a type alias, cast is a no-op
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! newtype {
    { $n:ident $t:ty [$($m:ident),*] } => {
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
    { $n:ident $t:ty [$($m:ident),*] } => {
        custom_derive! {
            #[derive($($m),*)]
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
        newtype!{
            Price i32 [Debug, Clone, Copy, PartialEq, NewtypeAdd]
        }
    }
    #[allow(dead_code)]
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
}
