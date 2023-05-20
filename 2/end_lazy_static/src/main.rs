use lazy_static::lazy_static;

#[derive(Debug)]
struct MyType {
    id: i64
}
impl MyType {
    fn new() -> Self {
        // code a
        // code b

        MyType { id: 1 }
    }
}

lazy_static! {
    static ref MYFIXTURE: MyType = {
        MyType::new()
    };
}

fn main() {
    println!("{}", MYFIXTURE.id);
}
