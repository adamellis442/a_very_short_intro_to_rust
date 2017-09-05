fn main() {
    //Rust comes with the usual built in types you would expect
    {
        let a: bool = true; //such as booleans
        let b: usize = 0; //usize type for indices and sizes of containers
        let c: u32 = 42; //unsigned 32 bit integers
        let d: u64 = 42; //unsigned 64 bit integers
        let e: i32 = -458; //signed 32 bit integers
        let f: i64 = -307883; //signed 64 bit integers
        let g: f32 = 3.14159; //32 bit floating point numbers
        let h: f64 = 1.6; //64 bit floating point numbers
        let i: char = '∯'; //unicode character

        println!("a={:?}", a);
        println!("b={:?}", b);
        println!("c={:?}", c);
        println!("d={:?}", d);
        println!("e={:?}", e);
        println!("f={:?}", f);
        println!("g={:?}", g);
        println!("h={:?}", h);
        println!("i={:?}", i);

        //also have containers such as vectors
        let j: Vec<u32> = Vec::new();
        //and a speciallised kind of vector for characters called string
        let k: String = "Hello world".to_string();
        println!("j={:?}", j);
        println!("k={:?}", k);
    }

    //structs and enums
    {
        //You can also create your own types by using structs as you might do in c++
        struct MyType {
            a: i32,
            b: f64,
        }

        let l: MyType = MyType { a: 42, b: 3.14159 };
        println!("l.a={:?}", l.a);
        println!("l.b={:?}", l.b);

        //and enumerations
        //which you can use as c++
        enum Fruit {
            Apple,
            Bannana,
            Pear,
        }
        let m: Fruit = Fruit::Apple;

        //alterativly you can use them a bit more powerfully
        enum RusticEnum {
            ANumber(i32), //can make your enum be an existing type
            AStruct(MyType), //can even be a struct
            AnEnum(Fruit), //can be another enum
            Label, //or just a label as before
        }
        let n: RusticEnum = RusticEnum::ANumber(42);
        let o: RusticEnum = RusticEnum::AnEnum(Fruit::Bannana);
    }
}
