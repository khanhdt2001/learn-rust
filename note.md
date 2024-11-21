Integer Overflow
Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. 

- compiling in debug mode -> panic 
- compiling in release mode -> wrap u8(256)-> 0;

In Rust people like to use snake_case 


The Stack and the Heap
Stack:  All data stored on the stack must have a known, fixed size.
Heap:  Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
## Pushing to the stack is faster than allocating on the heap
## Accessing data in the heap is slower than accessing data on the stack


### Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Memory and Allocation
In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability.
- With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the __heap__.

=> 
- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.

If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2

```
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!");
# s1 become invalid
using .copy()

```
Stack-Only Data: Copy 
data type store on stack can use copy but data store on heap cannot use copy 

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
The character type, char.

### Ownership and Functions
Passing a variable to a function will move or copy, just as assignment does.
```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```
### References and Borrowing
```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); # send address contain value

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.
Rust have data race condition 
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references