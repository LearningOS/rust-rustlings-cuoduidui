// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // number = 3; // don't rename this variable
    // Rust有类型检查,执行运算或者赋值时候要遵循类型的规律,但是Rust可以重新定义同名变量,变量的类型可以发生改变
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
