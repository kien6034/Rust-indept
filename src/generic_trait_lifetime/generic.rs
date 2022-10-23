pub fn run(){
    non_generic();
    generic_function();
    generic_struct();
    generic_method();
}


fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
 

fn non_generic(){
    println!("\n-----------\n Non generic");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = _largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = _largest_char(&char_list);
    println!("The largest char is {}", result);
}



// Largest is a generic function
// The generic type T need to implement PartialOrd trait, since we need to do the comparision (item > largest)
fn _largest<T>(list: &[T]) -> &T 
where 
    T: PartialOrd
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_function(){
    println!("\n-----------\n generic_function");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = _largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = _largest(&char_list);
    println!("The largest char is {}", result);
}



/// Generic Struct
struct Point<T> 
where T: Copy

{
    x: T,
    y: T,
} 

struct Point2<T, U> {
    x: T,
    y: U
}

fn generic_struct() {
    println!("\n-----------\n generic struct");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Integer point: {} - {}", integer.x, integer.y);
    println!("Float point: {} - {}", float.x, float.y);

    //let point = Point{x: 1, y: 1.0}; // will fail since Point accept two value with same types
    let point = Point2{x: 1, y: 1.0};
    println!("Point: {} - {}", point.x, point.y);
}


/// Generic in method defination
/// Implement generic returns for the generic struct point that we have just created 
impl <T> Point<T> 
where T:Copy {
    fn get_x(&self)-> T {
        return self.x
    }
}



fn generic_method(){
    println!("\n-----------\n generic struct");
    let point = Point {x: 1.0, y: 4.0};

    println!("{:?}", point.get_x());
}