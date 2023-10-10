// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a hint.


use std::f32;

fn main() {
    // let pi = 3.14_f32;
    let pi = std::f32::consts::PI;//std::f32::consts 检查分别在或 中定义的近似常量的浮点文字 std::f64::consts，建议使用预定义常量。
    let radius = 5.00_f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
