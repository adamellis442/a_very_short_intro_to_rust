//the function syntax in Rust is as follows
//fn name(args...)->returnType

fn double(x: i32) -> i32 {
    2 * x
}

/*You may have noticed that there is no return keyword used.
This is because Rust uses the last expression in the function body
as the return.

Expert warning: This is because in Rust functions are expressions rather than
statements. So they just evaluate to the last line of code in the function
body

If you find the concept of not having a return keyword too confusing
Rust does have one.*/

fn triple(x: i32) -> i32 {
    return 3 * x;
}

/*what if your function doesn't return anything?
You use the same syntax except you drop the -> and the return type.

Expert warning : All functions in Rust actually have to evaluate to something
(remember functions are expressions), the compiler will default all functions
to return and empty tuple "()" when you do not spesify the return type
*/

fn say_hello() {
    println!("Hello World");
}

fn read_out_number(x: i32) {
    println!("your number is {:?}", x);
}

fn main() {
    println!("5 doubled is {}", double(5));
    println!("5 tripled is {}", triple(5));

    say_hello();
    read_out_number(42);
}
