use std::io; // prelude
// use 导入一个库
fn main() {
    println!("猜数");
    // 内部定义的宏
    println!("猜测一个数");
    // let foo = 1;// 声明一个变量
    // foo = 2; // 默认变量是不可变的，需要加上mut表示一个可变变量
    let mut guess = String::new();
    // String 内置对象，new：生成一个新的实例
    io::stdin().read_line(&mut guess).expect("无法读取行");
    //& 表示引用，引用默认也是不可变的
    // io::result ok; Err
    println!("你猜测的数是：{}", guess);
    // {} 占位符
}
