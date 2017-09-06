//You can create your own traits and implement traits for your own structs

use std::fmt;

struct MyStruct {
    x: i32,
    y: i32,
}

//implement debug for MyStruct
impl fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct {{ x: {}, y: {} }}", self.x, self.y)
    }
}

//making a new trait is fairly simple, you can have ones that just tag something
trait MyTrait {}

/*traits with functions means that the function must be implemented for all types that
implement the trait*/
trait MyTraitWithFunction {
    fn do_something(&self) -> i32;
}

impl MyTrait for MyStruct {} //don't need to implement anything for this trait

impl MyTraitWithFunction for MyStruct {
    fn do_something(&self) -> i32 {
        self.x
    }
}

//just to show you the new traits working with the generic system
fn print_thing_with_my_trait<T>(_: T)
where
    T: MyTrait,
{
    println!("object with MyTrait implemented");
}

fn print_thing_with_my_trait_with_function<T>(x: T)
where
    T: MyTraitWithFunction,
{
    println!("object with MyTraitWithFunction implemented");
    println!("x.do_something()={:?}", x.do_something());
}

fn main() {
    let a = MyStruct { x: 0, y: 0 };
    println!("a = {:?}", a);

    print_thing_with_my_trait(a);
    //this will not compile since String doesn't have MyTrait
    //print_thing_with_my_trait("Hello".to_string());

    let b = MyStruct { x: 0, y: 0 };
    print_thing_with_my_trait_with_function(b);
}
