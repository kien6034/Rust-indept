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

