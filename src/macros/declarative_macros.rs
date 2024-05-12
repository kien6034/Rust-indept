/// less powerful, but provide an easy use interface for creating macros to remove dup code

/// use macro_rules! <name of macro>{<Body>}
/// Create a macro to add two number.
/// It doesnot add two number, it simply relace itself with the name of the macro, `add`, and the body of the macro
macro_rules! add {
    // macth like arm for macro
    ($a:expr, $b:expr) => {
  // macro expand to this code
         {
 // $a and $b will be templated using the value/variable provided to macro
            $a + $b
         }
    };
    // Second arm, to handle 1 expression
    ($a:expr) => {
        {
            $a
        }
    };
}

pub fn run() {
    // call to macro, $a=1 and $b=2
    println!("{}", add!(1, 2));
    println!("{}", add!(0));
}
