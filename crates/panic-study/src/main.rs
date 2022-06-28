use std::{fs::File, io::ErrorKind};

fn main() {
    // let f = File::open("hello.text");

    //1 .
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.text") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("error creating file: {:?}", e),
    //         },
    //         oe => panic!("Erro opening the file: {:?}", oe),
    //     },
    // };

    // 2.  unwrap 代提 match
    // let f =File::open("hello.text").unwrap();

    // 3. expect 可以打印错误信息
    // let f =File::open("hello.text").expect("期望创建成功");

    // let f = match File::open("hello.text") {
    //     Ok(file) => file,
    //     Err(e) => panic!("{}", e)
    // }

    // same with
    let f = File::open("hello.text")?;
}
