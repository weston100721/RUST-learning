struct User {
    name: String,
    age: i32,
    email: String,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let zhaoliang = User {
        name: String::from("zhaoliang"),
        age: 32,
        email: String::from("weston_contribute@163.com"),
    };

    println!("name {}, age {},email {}",zhaoliang.name,zhaoliang.age,zhaoliang.email);


    let mut yuyou = User {
        name: String::from("yuyou"),
        age: 18,
        email: String::from("yuyou@163.com"),
    };

    yuyou.name = String::from("shengping");
    println!("name {}, age {},email {}",yuyou.name,yuyou.age,yuyou.email);

    // let hello = build_user("hello".to_String(), 32);
    // println!("name {}, age {},email {}",hello.name,hello.age,hello.email);

    let user2 = User {
        name: String::from("anotherusername567"),
        ..zhaoliang
    };


    let rect1 = (34,56);
    println!("The area of the rectangle is {} square pixels.",area(rect1));


    let rect2 = Rectangle {width: 23,height: 55,};
    println!("The area of the rectangle is {} square pixels.",theArea(&rect2));
    println!("rect1 is {:?}",rect2);
    println!("{}",rect2.area());




}

fn theArea(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn area(dimensions: (i32,i32)) -> i32 {
    dimensions.0 * dimensions.1
}


fn build_user(name: String, age: i32) -> User {
    User {
        name,
        age,
        email: String::from("hello@163.com"),
    }
}
