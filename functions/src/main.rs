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

/*Structs can also have functions associated with them.
structs and associated functions is how Rust handles
objects. It's worth noting at this point, Rust is not
a fully object orientated language, so it doesn't
(at least not yet) support the typical object
orientated paradigms such as inheritance*/
struct MyStruct {
    a: i32,
    b: i32,
}

/*to associate functions with a struct we use the
impl keyword*/
impl MyStruct {
    /*a function you will normally see with structs is new
    This is Rusts constructor function*/
    fn new(x: i32, y: i32) -> MyStruct {
        MyStruct { a: x, b: y }
    }

    /*if a function reqires access to the
    associated struct, then it can do so
    by taking a referance to self*/
    fn first(&self) -> i32 {
        self.a
    }
    fn second(&self) -> i32 {
        self.b
    }

    /*if for any reason you want to update
    the structs contents you will need to
    take a mutable reference to it*/
    fn increment_first_by(&mut self, x: i32) {
        self.a += x;
    }
}

fn main() {
    println!("5 doubled is {}", double(5));
    println!("5 tripled is {}", triple(5));

    say_hello();
    read_out_number(42);

    let a = MyStruct::new(42, 56);
    println!("a.first = {:?}", a.first());
    println!("a.second = {:?}", a.second());
    /*note : we cannot use increment_first_by on a
    since it is immutable so calling any function
    that would mutate a is not allowed by the compiler*/
    //a.increment_first_by(5);

    let mut b = MyStruct::new(0, 0);
    b.increment_first_by(5);
    println!("b.first = {:?}", b.first());
    println!("b.second = {:?}", b.second());
}
