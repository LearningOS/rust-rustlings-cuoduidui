// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut res = 42;
    //  Some、Option、Result  https://blog.vgot.net/archives/rust-some.html
    let option = Some(12);
    // if let Some(x)=option{
    //     res +=x;
    // }
    match option{
        Some(x)=>res+=x,
        None=>println!("val is None")
    }
    println!("{}", res);
}
