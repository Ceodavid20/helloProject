fn main() {
    println!("Hello, world!");

    //primitive data types int(signed(i8, i16, i32, i64) and unsigned(u8, u16, u32, u64)), float(f32, f64), char, bool(true, false)

    let a: i32 = 10; //signed integer
    let b: u32 = 20; //unsigned integer
    let c: f32 = 3.14; //float
    let d: char = 'A'; //character
    let e: bool = true; //boolean  
    
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    //array

    let number: [i64; 5] = [1,2,3,4,5];

    println!("number: {:?}", number);

    //Compound data types: tuple, array, String, slice

    //tuple

    let x: (i32, &str, f64, bool) = (12, "beta", 3.14, true);
    println!("x: {:?}", x);

    func();
    new(25);
    publish("Dave", 21, x)

}

fn func(){
    println!("This is a function");
}
fn new(age: i32){

    println!("Age is: {}", age);

}
fn publish(name: &str, age: i64, school: (i32, &str, f64, bool)){
    println!("Name: {}, Age: {}, School: {:?}", name, age, school);
}