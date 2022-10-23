pub fn run(){
    dangling_referrence();
    life_time_in_function();
}


fn dangling_referrence(){
    println!("\n-----------\n Non generic");

    let _r;

    {
        let _x = 5;
        //_r = &_x; // This will err since x doesnt live long enough
        _r =5;
    }

    println!("This will err since x doest live long enough");
}


fn life_time_in_function() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    let _result: &str;
    {
        let string2 = String::from("xyz");
        _result = longest(string1.as_str(), string2.as_str()); // wont compile: 
        // Rust was told that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in.
    }
    //_result;
}

/// Normally, we could write longest(x: String, y: String) -> String() to return the longest string
/// But we don't want to take ownership of the strings, so in this case lifetime is needed 
/// In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments
/// when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints
fn longest<'a>(x: & 'a str, y: & 'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
