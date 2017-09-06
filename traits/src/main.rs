/*As mentioned prevously to do anything more interesting with generic
types we must spesify their traits

for example lets say you wanted to write a function that gives the
minimum of two arguments*/
fn min<T>(x: T, y: T) -> T
where
    T: Ord,
{
    if x < y { x } else { y }
}
//For the min function we need T to be orderable


fn main() {
    println!("min(5,7)={:?}", min(5, 7));
    println!("min(\"Hello\",\"World\")={:?}", min("Hello", "World"));
    struct MyStruct {
        a: i32,
        b: i32,
    }
    //since MyStruct don't have an orderable trait this won't compile
    let a = MyStruct { a: 1, b: 1 };
    let b = MyStruct { a: 2, b: 2 };
    println!("min(a,b)={:?}", min(a, b));
}
