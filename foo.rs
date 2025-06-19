// src/lib.rs
mod utils {      // 开启一个新作用域，模块名为 utils
 fn helper() {
        println!("I am a helper");
    }
}
fn main() {
    utils::helper();
}