fn main() {
    println!("Hello, world!");

    let number1 = 4;

    if number1 >= 4 {
        println!(" >= 4");
    } else {
        println!("< 4");
    }


    /// ```
    /// let number = 3;
    /// if number {
    ///    println!("number was three");
    /// }
    /// ```
    /// 这种方式不行的，他不想javascript一样，会进行类型转化。
    /// 

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // loop {
    //     println!("again");
    // }


    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10{
            break counter * 2; 
        }
    };
    println!("the result is {}", result);


    let mut offset = 3;
    while offset > 0 {
        println!(" > 0");
        offset -= 1;
    }
    println!(" <= 0 ");

    let a = [2,3,5,6,79,96];
    for element in a.iter() {
        print!("{},",element);
    }
    println!();


    for number in (1..4).rev() {
        println!("{}!", number);
    }

}
