
/// 1. 不可变的变量不能再赋值（要想重用变量名，需使用let重新定义，属于覆盖）。
/// 2. 可变变量的数据类型不能改变。
/// 3. 常量，在整个程序的运行期间，都是可以有效使用。

fn main() {

    // x 是一个不可变的变量，如果再赋值给它，就会报错。
    let x = 5;
    println!("这个值为{}",x);

    // 如果需要定义一个可以改变值的变量，则如下所示：
    let mut y = 4;
    println!("这个值为{}",y);
    y = 3;
    println!("这个值为{}",y);

    /// 在常量定义的范围内，在整个程序的运行期间，常量值都是可以有效使用的
    const MAX_POINT: u32 = 100_0000;
    println!("MAX_POINT的值为{}",MAX_POINT);


    /// 不可变变量可以通过覆盖的方式对值进行改变
    let z = 1;
    let z = z + 1;
    let z = z * 2;
    println!("the value of 'z' is {}",z);

    /// 不建议这样使用
    let spaces = "    ";
    let spaces = 2;

    /// let mut size = 43;
    /// size = "  ";

}
