// Release mode, just a type alias, cast is a no-op
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! newtype {
    ($n:ident, $t:ty, #[$m:meta], $impls:block) => {
        type $n = $t;
        macro_rules! $n {
            ($x:expr) => {
                $x
            };
        }
        // skip meta and impls in release mode;
        // it wouldn't work for primitive types.
    };
}

#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! cast {
    ($x:expr) => {
        $x
    };
}

// Development mode, tuple struct, cast extracts the underlying type
#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! newtype {
    ($n:ident, $t:ty, #[$m:meta], $impls:block) => {
        #[$m]
        #[repr(transparent)]
        struct $n($t);
        macro_rules! $n {
            ($x:expr) => {
                $n($x)
            };
        }
        $impls
    };
}

#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! cast {
    ($x:expr) => {
        ($x).0
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::ops::Add;

    #[allow(dead_code)]
    #[test]
    fn it_works() {
        newtype!(Price, i32, #[derive(Debug, Clone, Copy, PartialEq)], {
            impl Add for Price {
                type Output = Self;
                fn add(self, other: Self) -> Self {
                    Price(self.0 + other.0)
                }
            }
        });
        let p = Price!(23);
        let p2 = Price!(7);
        assert_eq!(p + p2, Price!(30));

        // won't compile in debug mode
        // let result = 42 + p;

        let result = 42 + cast!(p);
        assert_eq!(result, 65);
    }
}
