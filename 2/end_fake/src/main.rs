use fake::faker::name::en::{FirstName, LastName};
use fake::faker::lorem::en::Sentence;
use fake::faker::address::en::CountryName;
use fake::faker::internet::en::Username;
use fake::Fake;
use std::ops::Range;

fn main() {
    // let name: String = FirstName().fake();
    let sentence: String = Sentence(Range{ start: 5, end: 10}).fake();
    println!("{}", sentence);
}
