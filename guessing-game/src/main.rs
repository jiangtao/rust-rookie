// https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html
use rand::Rng;
use std::io; // Import the io module 相当于 Node.js的require
// :: 和 . 的区别是什么, 什么时候用. 什么时候用::
// 得克服心理作用, 把以前学的 js 都忘掉, 毕竟 rust 确实读起来好别扭啊 😳
fn main() {
    println!("猜数字啊?"); // console.log
    println!("输入个数字?"); // console.log
    /**
     * 在Rust中，变量是通过let来声明的，let x = 5;
     * mut的含义是可变的，可以修改变量的值, 不带 mut 代表变量是不可变, 相当于 const
     * :: 相当于访问符号, 就是 javascript .
     * & 引用，指针，可以改变引用的值
     * &mut 可变引用，指针，可以改变引用的值
     * 可以用 let mut x = 5; 来声明可变变量
     */
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..201);
    println!("秘密数字是 {}", secret_number);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("你猜的数字是: {}", guess); // console.log('%o', {}), %s, %d etc...
}
