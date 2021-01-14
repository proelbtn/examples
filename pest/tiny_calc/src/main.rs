#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Calculator;

fn main() {
    use pest::Parser;

    let pairs = Calculator::parse(Rule::expr, "(12+34)*56").unwrap();

    for pair in pairs {
        println!("{:#?}", pair);
    }
}
