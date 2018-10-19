pub mod logic;
pub mod ast;

fn main() {
    let formula = logic::FormulaParser::new().parse("aVb").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("(aVb)^c").unwrap();
    println!("{:?}", formula);
}
