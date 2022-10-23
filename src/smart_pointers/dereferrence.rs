pub fn run(){
    normal_dereferrence();
    boxt_dereference();
    my_smart_counter();
    deref_coercion();
}


fn normal_dereferrence(){
    println!("\n-----------\nNormal dereferrence");
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); 
    // note: here, need to use *y, since y is the referrence only, we need to dereferrence it to get what value it is pointing to
}



fn boxt_dereference(){
    println!("\n-----------\n boxt dereferrece");
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); 
    // Box here are implemented deferrence trait 
    // The diffrennce here compares to normal_referrence is that  we set y to be an instance of 
    // a box pointing to a copied value of x rather than a reference pointing to the value of  x
}



struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn my_smart_counter() {
    println!("\n-----------\n My Smart Counter");
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);  This will err, since myBox is not impletmend the Deref trait
    
    // implement the deref trait
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T; // Todo: Cover generic type - Chapter 19
    
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
fn deref_coercion(){
    println!("\n-----------\n Deref Coercion");
    let m = MyBox::new(String::from("Rust"));
    // deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str
    hello(&m);
}