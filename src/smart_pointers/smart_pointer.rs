
pub fn run(){
    simple();
    cons_list();
}


fn simple(){
    println!("\n running simple box example");
    let b = Box::new(5);
    println!("b = {}", b);
}

/// ```
/// enum List {
///     Cons(i32, List),
///     Nil
/// }
/// ```
/// This will return err since we the compiler can't know the size of List 
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};
fn cons_list() {
    println!("\n running coin list example");
    //let list = Cons(1, Cons(2, Cons(3, Nil)));
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}