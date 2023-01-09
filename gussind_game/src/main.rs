// 导入标准库 io
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // let声明一个变量；
    //mut表示是个可变变量；
    // = ： 表示绑定一个值到一个变量
    // String::new 表示返回一个string类型的实例；
    // String 标准库中的字符串类型，支持utf8编码；
    // ::new 表示 new是一个string类型的关联函数；
    // new 表示一个新的空的字符串；new是一个通用的函数名称，可以是很多其他类型；
    // 该语句表示，创建一个可变比纳凉当前绑定到一个新的空的字符串类型；
    let mut guess = String::new();
    
    // 接受用户输入
    // 使用io模块的stdin函数；处理用户输入
    // 如果开头没有通过use std::io; 导入IO模块；也可以将该句写为：std::io::stdin
    // std::io::stdin 函数表示接受一个终端的输入
    io::stdin()
        // read_line 获取用户终端输入
        // & 表示这个参数是一个引用；多段代码引用同一部分数据，而不用多次在内存中写入；
        .read_line(&mut guess)
        // 也可以写成 io::stdin().read_line(&mut guess).expect("failed to read line");
        // 处理异常
        .expect("Failed read line");
    //println! 中的占位符
    println!("You Guessed: {guess}");
}