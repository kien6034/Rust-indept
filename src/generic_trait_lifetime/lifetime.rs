pub fn run(){
    dangling_referrence();
    life_time_in_function();
    struct_with_lifetime();
    method_with_lifetime();
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
        _result = longest(string1.as_str(), string2.as_str()); // wont compile when uncomment _result
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



fn _will_fail_function<'a>(x: &'a str, _y: &str ) -> &'a str {
    let _result = String::from("really long string");
    //_result.as_str()  // This will err: Cannot return referrence to the data owned by the funcc
    x
}



/// LIFETIME IN STRUCT 

// This life time annotation means that an instance of ImportantExecpt cannot outlive the referrence it holds in part field 
struct ImportantExcerpt <'a> {
    part: &'a str
}

fn struct_with_lifetime(){
    println!("\n-----------\n Lifetime in struct");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}


/// METHOD WITH LIFETIME

impl <'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }


    /// Third rule of lifetime ellison
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    fn get_part(&self)-> &'a str{
        return self.part
    }
}

fn method_with_lifetime(){
    println!("\n-----------\n Lifetime in method struct");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Level: {}", i.level());
    println!("Announcment return: {}", i.announce_and_return_part("abc".into()));
    println!("Part: {}", i.get_part());
}