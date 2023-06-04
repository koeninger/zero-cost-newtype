// Release mode, just a type alias, cast is a no-op
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! newtype {
    ($n:ident, $t:ty) => {
        type $n = $t;
        macro_rules! $n {
            ($x:expr) => {
                $x
            };
        }
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
    ($n:ident, $t:ty) => {
        #[repr(transparent)]
        struct $n($t);
        macro_rules! $n {
            ($x:expr) => {
                $n($x)
            };
        }
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
    #[test]
    fn it_works() {
        newtype!(Price, i32);
        let p = Price!(23);

        // won't compile in debug mode
        // let result = 42 + p;

        let result = 42 + cast!(p);
        assert_eq!(result, 65);
    }
}
