/*Rust has a powerful method for handling enums, particuarly
enums with other types imbedded in them.
lets say you had the following enum*/
enum MyEnum {
    ANumber(u64),
    AWord(String),
    ALabel,
}

//and we want to print the enum
fn print_enum(x: MyEnum) {
    /*In c++ you might use a switch statement.
    Pattern matching can be thought of as a more powerful
    switch statement. As we will see on the line below*/

    match x {
        MyEnum::ANumber(value) => println!("a number : {}", value),
        MyEnum::AWord(value) => println!("a word : {}", value),
        MyEnum::ALabel => println!("just a label"),
    }
}

fn main() {
    print_enum(MyEnum::ANumber(42));
    print_enum(MyEnum::AWord("Hello".to_string()));
    print_enum(MyEnum::ALabel);

    //you can also use pattern matching to set a value of a variable
    let a = MyEnum::ANumber(26);
    let b: u64 = match a {
        MyEnum::ANumber(value) => value,
        _ => 0, //the _ will match anything. It's essentially the default case in a switch
    };
    println!("b={:?}", b);
}
