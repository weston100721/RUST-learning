fn main() {
    println!("Hello, world!");

    another_function();
    the_value(3233);
    the_plus(32,12);
    // contain_statements();
    println!("the return value of the function is {}. ",five());

}

fn another_function(){
    println!("another function")
}

fn the_value(x: i32){
    println!("the value of variable is {}",x)
}

fn the_plus(x: i32, y: i32){
    println!("the x + y = {}",x + y);
}

fn contain_statements(){
    let a = 5;

    let b = {
        let a = 3;
        a + 1
    };

    println!("The value of y is: {}", b);
}

fn five() -> i32 {
    return 5;
}
