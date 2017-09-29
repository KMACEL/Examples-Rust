fn main() {
    println!("Numaric Types");

    // Boolean
    let boolean_1 = true;
    let boolean_2: bool = false;
    println!("Boolean : {}, {}", boolean_1,boolean_2 );



    // Char
    let x_characters = 'x';
    let two_hearts = '💕';
    let character_numaric = 89;
    println!("Char : {}, {}, {}", x_characters,two_hearts,character_numaric );



    // Numarics
    // Default Types
    let integer_type = 42; // `x` has type `i32`.
    let float_type = 1.0; // `y` has type `f64`.
    println!("Default Numaric : {}, {}", integer_type,float_type);

    // Other Numaric Types
    // i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64
    let integer_type_test: i32=-42;
    let unsignet_type_test: u16=24; // Not Negative
    let float_type_test:f32=26.35;
    println!("Numaric Types : {}, {},{}", integer_type_test,unsignet_type_test,float_type_test);



    // Arrays
    let array1 = [1, 2, 3]; // array1: [i32; 3]
    let mut array2= [1, 2, 3]; // array2: [i32; 3]
    println!("Array Types : {}, {}", array1[0],array2[1]);



}
