fn main() {
    //variables are decleared as let name: type = value;
    let a: i32 = 42;
    //below would not compile as variables must be initialsised
    //let b: i32;
    //variables are immutable by default (ie const) so below would not compile
    //a = 128;
    //if you want a mutable variable you must decleare it as mutable
    let mut c: i32 = 0;
    c = 5;

    //apparently you can leave variables uninitialised so long as you don't ever use them
    let d: i32;

    println!("a={:?}", a);
    //println!("b={:?}", b);
    println!("c={:?}", c);
}
