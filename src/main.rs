pub mod logic;
pub mod ast;

fn main() {
    let formula = logic::FormulaParser::new().parse("aVb").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("(aVb)^c").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("~a").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("~(aVb)").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("~b^i").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("aV(~b^i)").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("~(aV(~b^i))").unwrap();
    println!("{:?}", formula);
    let formula = logic::FormulaParser::new().parse("i^(~(aV(~b^i)))").unwrap();
    println!("{:?}", formula);
}
