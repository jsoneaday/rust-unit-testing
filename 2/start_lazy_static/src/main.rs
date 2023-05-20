const IMMUTABLE_VAL: &str = "hello";
static MUTABLE_VAL: i32 = 1;

fn main() {
    println!("{}", IMMUTABLE_VAL);

    println!("{}", MUTABLE_VAL);
}
