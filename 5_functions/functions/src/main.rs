// Every Rust program has at least one function, the main function:
fn main() {
    println!("Functions");

    non_parameter();

    parameter(5,8);

    let get_value = return_parameter(11);
    println!("Get value : {}",get_value );

    println!("Return Parameter 2 : {}",return_parameter2(25) );


    // Without type inference:
    let f: fn(i32) -> i32 = plus_one;

    // With type inference:
    let f = plus_one;

    let six = f(5);
    println!("Plus One : {}",six );

    //diverges()
}

fn non_parameter(){
    println!("None Parameter Function");
}

fn parameter(param1: i32 , param2: i32){
    println!("Parameter Function");
    println!("Paramater 1 : {} \nParameter 2 : {}",param1,param2);
    println!("Sum {}",param1+param2);
}

fn return_parameter(param:i32)->i32{
    param+1 // <- Not ";"
}

fn return_parameter2(param:i32)->i32{
    return param+1;
}

// Panic Error
fn diverges() -> ! {
    panic!("This function never returns!");
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
