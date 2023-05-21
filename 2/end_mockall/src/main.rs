use mockall::*;
use mockall::predicate::*;

#[automock]
trait MyTrait {
    fn greet(&self, first_name: String, last_name: String) -> String;
}

fn exec_greet(greeter: &impl MyTrait) -> String {
    greeter.greet("John".to_string(), "Choi".to_string())
}

fn main() {
    let first_name = "John".to_string();
    let last_name = "Choi".to_string();

    let mut mock = MockMyTrait::new();
    mock.expect_greet()
        .with(predicate::eq(first_name.clone()), predicate::eq(last_name.clone()))
        .times(1)
        .returning(|first, last| format!("Hello {} {}", first, last));

    // let message = exec_greet(&mock);
    // println!("{}", message);
    assert!(format!("Hello {} {}", "sdfds", last_name) == exec_greet(&mock))
}
