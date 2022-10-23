# GENERIC - TRAIT - LIFETIME 

## Generic 
We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

- Generic fucntion: Allow function to take generic parameters 
- Generic struct: Allow creating struct with a generic type 
- Generic enum: Same as struct. Example: Option<T>
- Generic method: Generic behavior for struct's method implementation

## Trait 
Trait is somewhat similar to interface in other programming language.
A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.

- Trait bound
```
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

- Trait bounds to conditional Implement methods -> Example conditional_trait_bound()

## Lifetime
Life time ensure that referrences are valid as long as we need them to be. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.

### How the compile handle lifetime!

```
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

```
Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As you can see, the inner 'b block is much smaller than the outer 'a lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.