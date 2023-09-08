// create 3 variables. a=6 , b=3, c=2. Then create a program
// that does the following calculations:
// a) a + b * c
// b) (a + b) * c
// c) a / b / c
// d) a / (b / c)

pub fn run() {
    let a = 6;
    let b = 3;
    let c = 2;

    let answer_a = a + b * c;
    let answer_b = (a + b) * c;
    let answer_c = a / b / c;
    let answer_d = a / (b / c);

    println!("a) {}", answer_a);
    println!("b) {}", answer_b);
    println!("c) {}", answer_c);
    println!("d) {}", answer_d);
}
