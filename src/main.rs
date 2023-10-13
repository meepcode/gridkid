use gridkid::{model::{Primitive, Operation, Evaluatable, CellAddress, CellValue}, environment::Environment};

fn main() {
    let mut environment = Environment::init();

    let five = Primitive::Integer(5);
    let four = Primitive::Integer(4);
    let negative_four = Primitive::Integer(-4);
    let seventeen = Primitive::Integer(17);

    let six_point_four = Primitive::Float(6.4);
    let five_point_seven = Primitive::Float(5.7);
    let negative_five_point_seven = Primitive::Float(-5.7);

    let primitive_true = Primitive::Boolean(true);
    let primitive_false = Primitive::Boolean(false);

    let test_str = Primitive::String("test");
    let not_test_str = Primitive::String("not test");

    println!("INTEGER ARITHMETIC");
    println!("5 + -4 = {}", Operation::Add(&five, &negative_four).evaluate(&environment).unwrap().to_string());
    println!("5 - 17 = {}", Operation::Subtract(&five, &seventeen).evaluate(&environment).unwrap().to_string());
    println!("-4 * 17 = {}", Operation::Multiply(&negative_four, &seventeen).evaluate(&environment).unwrap().to_string());
    println!("5 / -4 = {}", Operation::Divide(&five, &negative_four).evaluate(&environment).unwrap().to_string());
    println!("17 % 5 = {}", Operation::Modulus(&seventeen, &five).evaluate(&environment).unwrap().to_string());
    println!("-4 ** 5 = {}", Operation::Power(&negative_four, &five).evaluate(&environment).unwrap().to_string());
    println!();

    println!("FLOAT ARITHMETIC");
    println!("6.4 + 5.7 = {}", Operation::Add(&six_point_four, &five_point_seven).evaluate(&environment).unwrap().to_string());
    println!("6.4 - -5.7 = {}", Operation::Subtract(&six_point_four, &negative_five_point_seven).evaluate(&environment).unwrap().to_string());
    println!("-5.7 * 5.7 = {}", Operation::Multiply(&negative_five_point_seven, &five_point_seven).evaluate(&environment).unwrap().to_string());
    println!("6.4 / -4 = {}", Operation::Divide(&six_point_four, &negative_four).evaluate(&environment).unwrap().to_string());
    println!("-5.7 % 5 = {}", Operation::Modulus(&negative_five_point_seven, &five).evaluate(&environment).unwrap().to_string());
    println!("5.7 ** -5.7 = {}", Operation::Power(&five_point_seven, &negative_five_point_seven).evaluate(&environment).unwrap().to_string());
    println!();

    println!("LOGICAL");
    println!("true && true = {}", Operation::LogicalAnd(&primitive_true, &primitive_true).evaluate(&environment).unwrap().to_string());
    println!("false || false = {}", Operation::LogicalOr(&primitive_false, &primitive_false).evaluate(&environment).unwrap().to_string());
    println!("!true = {}", Operation::LogicalNot(&primitive_true).evaluate(&environment).unwrap().to_string());
    println!();

    println!("BITWISE");
    println!("5 & 4 = {}", Operation::BitwiseAnd(&five, &four).evaluate(&environment).unwrap().to_string());
    println!("5 | 4 = {}", Operation::BitwiseOr(&five, &four).evaluate(&environment).unwrap().to_string());
    println!("5 ^ 4 = {}", Operation::BitwiseXor(&five, &four).evaluate(&environment).unwrap().to_string());
    println!("~5 = {}", Operation::BitwiseNot(&five).evaluate(&environment).unwrap().to_string());
    println!();

    println!("EQUALITY");
    println!("5 == 5 = {}", Operation::Equals(&five, &five).evaluate(&environment).unwrap().to_string());
    println!("\"test\" ==  \"not test\" = {}", Operation::Equals(&test_str, &not_test_str).evaluate(&environment).unwrap().to_string());
    println!("true == false = {}", Operation::Equals(&primitive_true, &primitive_false).evaluate(&environment).unwrap().to_string());
    println!("5.7 != -5.7 = {}", Operation::NotEquals(&five_point_seven, &negative_five_point_seven).evaluate(&environment).unwrap().to_string());
    println!();

    println!("COMBINATION");
    println!("{} = {}", Operation::Equals(&Operation::Multiply(&Operation::Add(&Operation::Subtract(&five, &five), &five), &Operation::BitwiseOr(&five, &four)), &Primitive::Integer(25)).to_string(), Operation::Equals(&Operation::Multiply(&Operation::Add(&Operation::Subtract(&five, &five), &five), &Operation::BitwiseOr(&five, &four)), &Primitive::Integer(25)).evaluate(&environment).unwrap().to_string());

    println!("Adding to grid");
    environment.set_cell(&CellAddress(0, 0), Box::new(five));
    environment.set_cell(&CellAddress(0, 1), Box::new(not_test_str));
    environment.set_cell(&CellAddress(1, 1), Box::new(CellValue(0, 0)));
    environment.set_cell(&CellAddress(12, 13), Box::new(Operation::Equals(&Primitive::Integer(6), &Primitive::Integer(6))));
    environment.set_cell(&CellAddress(13, 12), Box::new(Operation::Equals(&Primitive::Integer(6), &Primitive::String("hello, world"))));

    println!("{}: {}", CellAddress(0, 0).to_string(), environment.get_cell(&CellAddress(0, 0)).unwrap().to_string());
    println!("{}: {}", CellAddress(0, 1).to_string(), environment.get_cell(&CellAddress(0, 0)).unwrap().to_string());
    // println!("{}: {}", CellAddress(0, 2).to_string(), environment.get_cell(&CellAddress(0, 2)).unwrap().to_string());
    println!("(12, 13): {}", environment.get_cell(&CellAddress(12, 13)).unwrap().evaluate(&environment).unwrap().to_string());
    println!("(13, 12): {}", environment.get_cell(&CellAddress(13, 12)).unwrap().evaluate(&environment).unwrap().to_string());
}
