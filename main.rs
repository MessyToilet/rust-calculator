mod calcualtor;

fn main() {
    let tokens = calcualtor::Calculator::parse("2 * 2 + 48 / 4");
    println!("{:?}", tokens);
    let expr = calculator::Calculator::expression(tokens.unwrap());
    println!("{:?}", expr);
}