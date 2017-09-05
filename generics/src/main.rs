//Generics in Rust are very much like c++

fn foo<T>(x: T) -> T {
    x
}
/*Unlike C++ you can only do operations with
generic types that you have prevously spesified
with traits. Without traits there isn't much
you can do with a type T without knowing about
what traits it has. More about traits later.
For now it's enough to know that the function
below will not compile*/
/*fn add_up<T>(x: T, y: T) -> T {
    x + y
}*/

//you can also use generics in structs
struct Pair<T> {
    first: T,
    second: T,
}
//and in enums
enum MyEnum<T> {
    ANumber(i32),
    SomethingElse(T),
}

fn main() {
    println!("foo(5)={:?}", foo(5));
    let p = Pair {
        first: 1,
        second: 5,
    };
    println!("p.first={:?}", p.first);
    println!("p.second={:?}", p.second);

    let q = MyEnum::SomethingElse("Hello".to_string());
    match q {
        MyEnum::ANumber(n) => println!("a number : {:?}", n),
        MyEnum::SomethingElse(t) => println!("something else {:?}", t),
    }
}
