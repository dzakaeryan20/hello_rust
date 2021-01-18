#![allow(unused)]
fn main() {
    let xe = 5;
    // let y = Box::new(x);
    println!("{}",xe);
    
    // variable mutability
    let x = 5;
    println!("{}", x);

    // x = 6; // error because value x is immutable variable and can't change
    // In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change. Your code is thus easier to reason through.

     // But mutability can be very useful. 
     // Variables are immutable only by default; as you did in Chapter 2, 
     // you can make them mutable by adding mut in front of the variable name. 
     // In addition to allowing this value to change, 
     // mut conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.
     
     // example

     let mut a = 10;
     println!("{}",a); // value a is 10 
     a = 11;
     println!("{}", a); // value a is 11

     // Being unable to change the value of a variable might have reminded you of another programming concept that most other languages have: constants. 
     // Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
     // First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
     // Second, you need declared by data types
     // third variable name must upper case
    
     const MAX: i32 = 23;
     println!("{}", MAX);

}
