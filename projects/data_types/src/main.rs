/// 
/// 1. rust是一种静态类型的编程语言。
/// 2. 所以所有的变量类型在编译时要明确确定。
/// 
fn main() {
    let guess: u32 = "22".parse().expect("not a number");


    let int1: i8 = 22;

    let double1 = 2.0; // f64
    let double2: f32 = 3.3; // f32

    let bool1 = false;
    let bo23: bool = true;


    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);


}
