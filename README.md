## crabi_test
An Experimental Rust Library for ABI Exploration

crabi_test is a Rust library designed to facilitate practical exploration and understanding of Application Binary Interface (ABI) concepts, particularly 
as they relate to cross-language interoperability and Rust’s Foreign Function Interface (FFI).

## Purpose
The library provides a set of simple, well-documented examples to illustrate how Rust can interact with other languages like C. By using features like #[repr(C)] for predictable memory layouts and extern "C" functions for FFI, crabi_test demonstrates the challenges and considerations involved in ABI compatibility.

## Key Features
Data Structure Layouts: Examples of #[repr(C)] data structures to understand how Rust data can be made ABI-compliant. Cross-Language Function Calls: extern "C" functions that show how data is passed and modified between Rust and other languages. Practical Tests: Built-in tests to validate behavior and illustrate common pitfalls in ABI design and implementation.

## Use Cases
crabi_test is suitable for:

Developers interested in systems programming and understanding how Rust can interface with C and other languages. Experiments and educational purposes, offering a foundation to explore the implications of ABI stability or the lack thereof in Rust.

##Example Code
```rust
#[repr(C)]
pub struct SimpleData {
    pub value: i32,
    pub flag: bool,
}

#[no_mangle]
pub extern "C" fn modify_simple_data(data: *mut SimpleData) {
    // Function to modify SimpleData structure
}
```
## License
Distributed under the MIT or Apache 2.0 license.

## Author
Ben Santora bensatlantik@gmail.com

## Support My Work
If you find this library helpful, consider supporting my open-source efforts: 

https://www.paypal.com/donate/?business=QMSZ2E6L75BYN&no_recurring=0&item_name=If+my+Rust+libraries+have+added+value+to+your+projects%2C+consider+a+small+donation+to+help+me+continue.+Thank+you%21&currency_code=USD

