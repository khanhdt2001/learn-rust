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
https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type