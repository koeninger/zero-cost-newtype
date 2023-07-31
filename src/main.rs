mod types {
    // you want everything in scope because the macro expansion depends on macro_attr_2018
    use zero_cost_newtype::*;

    // in debug mode this is `struct Price(i32)` and a macro Price! as a constructor
    // in release mode `type Price = i32`
    newtype! {
      #[derive(Debug, Clone, Copy, PartialEq, NewtypeAdd!, NewtypeDisplay!)]
      Price: i32
    }
}

fn main() {
    // see issue #52234
    use crate::*;
    use types::Price;
    use zero_cost_newtype::cast;

    // in debug mode this is `= Price(23)`, in release mode `= 23`
    let p: Price = Price!(23);
    let p2 = Price!(7);
    assert_eq!(p + p2, Price!(30));
    println!("display works {p2}");

    // won't compile in debug mode, can't accidentally add unrelated types.
    // in release mode it would compile,  but you're safe as
    // long as you compiled in debug during development.
    //
    // let result = 42 + p;

    // if you need an escape hatch use cast! rather than .0 or .into or .from;
    // it's a no-op in release mode and .0 in debug mode.
    let result = 42 + cast!(p);
    assert_eq!(result, 65);
}
