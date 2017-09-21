/* One of the main features of Rust is what is called the lifetime checker.
By default Rust moves everything into functions, unless the type being
passed in has the copy trait. In which case Rust will copy if needed.
*/

#[derive(Debug)]
enum MyEnum {
    A,
}

fn moves_argument(x: MyEnum) {
    println!("x has been moved into this function. x = {:?}", x);
}

fn foo(x: i32) {
    println!("x has been moved or copied into this function. x = {:?}", x);
}

fn main() {
    let a = MyEnum::A;
    moves_argument(a);
    /*we won't be able to call this function a second time
    because a was moved into the function body on the first
    call and so no longer exists on the second call because
    it could not be copied*/
    //moves_argument(a);

    let b = 42;
    foo(b); //Rust can copy b into the function here
    foo(b); //so that b is still alive by this point in time

    //another example
    //here we borrow b
    let mut d = &b;
    {
        //in this code block we define a new variable
        let e = 59;
        //now set d to borrow from e
        d = &e;
    }
    //this won't compile because e doesn't live as long as d does
}
