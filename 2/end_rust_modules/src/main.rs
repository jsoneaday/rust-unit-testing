use mod_a::file_a;

mod mod_a {
    pub mod file_a;
}

fn main() {
    file_a::greet();
}
