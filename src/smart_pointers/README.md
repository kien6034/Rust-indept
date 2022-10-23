## Referrences 
https://doc.rust-lang.org/book/ch15-01-box.html

## Smart Pointers 
### What is smart pointers?
Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type (Chapter 17)


### Computing size of a Non-Recursive Type 
```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
The variant takes the most space will be used as size of enum Message 

### ConsList 
ConsList examples show that we can use Smart Pointer to handle a type whose size can't be known at compile time.



## Dereferrence 15.2
Dereferrence is the process of getting the value whose the pointer is pointing too

- normal_dereferrence: Example of a normal deferrence actions
- boxt_dereferrence: Dereferrece when using Box T 
- my_smart_counter: Create a custom smart counter which implemetns the DeRef trait 
- deref_coercion: Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type
- DerefMut: 15.2 - How Deref Coercion Interacts with Mutability