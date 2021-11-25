use std::io::Error;

use lark::Lark;

fn main() {
    let ins = Lark::new(String::from(""), String::from(""));

    let res = ins.get_token().unwrap_err();
    println!("err: {}", res.message);
    // match res {
    //     Result(res) => {
    //         println!("{:?}", res);
    //     }
    //     Err(e) => {
    //         println!("{}", e.message);
    //     }
    // }
    // block_on(ins.get_token());
    println!("Hello, world!");
}
