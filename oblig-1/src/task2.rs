//Task 2:
//Create 3 variables holding your information information
// 1. Firstname
// 2. Lastname
// 3. Age in whole number
//Then print it out using this format "Hey, my name is {Firstname} {Lastname} and I'm {age} years
//old"

fn introduce(firstname: String, lastname: String, age: i32) {
    println!("Hey, my name is {firstname} {lastname} I'm {age} years old.");
}

pub fn run() {
    let firstname = String::from("Martin");
    let lastname = String::from("Ellegard");
    let age = 26;

    introduce(firstname, lastname, age)
}
