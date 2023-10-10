// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(3);
}
// rust 基本类型 https://blog.csdn.net/qq_21484461/article/details/131445816
fn call_me(num:u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
