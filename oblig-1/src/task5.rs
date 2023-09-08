// Make an overview of how many cookies people have eaten (5, 9, 2.5, 21, 0)
// Find the average amount of cookies eaten represented as an integer

struct Person {
    firstname: String,
    lastname: String,
    cookies_eaten: f32,
}

fn get_average(people: Vec<Person>) -> i32 {
    let mut total = 0.0;
    let length = people.len();

    people
        .into_iter()
        .for_each(|person| total += person.cookies_eaten);

    (total / length as f32).round() as i32
}

pub fn run() {
    let cookie_input = vec![5.0, 9.0, 2.5, 21.0, 0.0];
    let mut people: Vec<Person> = Vec::new();

    cookie_input
        .into_iter()
        .enumerate()
        .for_each(|(i, cookies_eaten)| {
            people.push(Person {
                firstname: "Person".to_string(),
                lastname: format_args!("{}", i).to_string(),
                cookies_eaten: cookies_eaten,
            })
        });

    println!("Avergae cookies eaten per person: {}", get_average(people));
}
