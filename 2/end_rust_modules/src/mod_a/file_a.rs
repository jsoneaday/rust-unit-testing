use self::inner_mod::inner_greet;

mod inner_mod {
    pub fn inner_greet() {
        println!("inner hello");
    }

    fn private_greet() {
        println!("private hello");
    }
}

pub fn greet() {
    inner_greet();
}