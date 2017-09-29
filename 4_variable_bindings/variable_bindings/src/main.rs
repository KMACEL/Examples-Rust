fn main() {
    println!("Variables");

    let only_let_variables = 5;
    let (multiple_variables_1, multiple_variables_2) = (1, 2);

    let variables_type: i32 = 5;
    // i : int
    // 32 : bit value (8,16,32,64)
    // if we do not specify it,


    // This style definition is fixed. If you do something like this, you will get an error like
    // let x = 5;
    // x = 10;
    /*
    --> src/main.rs:13:5
    |
 12 |     let x = 5;
    |         - first assignment to `x`
 13 |     x = 10;
    |     ^^^^^^ re-assignment of immutable variable
    error: aborting due to previous error

    error: Could not compile `variable_bindings`.
    */
    // If you are to assign the variable later, you should put the "mut" statement. Mut - Mutability is coming. This gives the variable meaning. The meaning of change.
    let mut mutabilitiy_variables = 5; // mut mutabilitiy_variables: i32
    mutabilitiy_variables = 10;

     println!("Variables : {},{},{},{},{}",only_let_variables,multiple_variables_1,multiple_variables_2,variables_type,mutabilitiy_variables);


     let out_variable:i32 = 17;
     {
         let in_variable: i32 = 3;
         println!("The value of x is {} and value of y is {}", out_variable, in_variable);
     }
     //println!("The value of x is {} and value of y is {}",out_variable, in_variable); // This section does not work. Because the duration of the "in_variable" defined between {} has ended in {}.


     // where the mutability state of the variables can be used.
     let mut x: i32 = 1;
     x = 7;
     let x = x; // `x` is now immutable and is bound to `7`.

     let y = 4;
     let y = "I can also be bound to text!"; // `y` is now of a different type.
}
