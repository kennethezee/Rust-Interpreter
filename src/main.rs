// Link to my forked repo
// https://github.com/kennethezee/interpreter
// Used the same notes file from class but added a new expression to fulfull HW

use std::env::consts::EXE_SUFFIX;
use std::f32::consts::E;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Variable(String),
    Number(i32),
}

pub struct Environment {
    map: HashMap<String, Expression>,
    parent: Option<Box<Environment>>,
    // HW
    cost: Option<HashMap<i32, i32>>,
    difference: Option<i32>,
}

impl Environment {

    pub fn new() -> Environment {
        Environment{map: HashMap::new(), parent: None, difference: None, cost: None}
    }

    pub fn value_for_key(self: &Environment, key: String) -> &Expression {
       &self.map.get(&key).unwrap_or(&Expression::Number(0))
    }

    pub fn add_value_for_key(&mut self, key: String, value: &Expression) {
        &self.map.insert(key, value.clone());
    }
}

pub fn evalute_addition(e: &Expression, environment: &Environment) -> i32 {
    if let Expression::Add(v) = e {
        let iter = v.iter();
        iter.fold(0, |t, e| t + evaluate(e, environment))
    } else {
        panic!("not addition!")
    }
}

pub fn evaluate_multiplication(e: &Expression, environment: &Environment) -> i32 {
    if let Expression::Multiply(v) = e {
        let iter = v.iter();
        iter.fold(1, |t, e| t * evaluate(e, environment))
    } else {
        panic!("Not multiplication!")
    }
}

pub fn evaluate_substraction(e: &Expression, environment: &Environment) -> i32 {
    if let Expression::Subtract(v) = e {
        let mut iter = v.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(first, environment), |t: i32, e| t - evaluate(e, environment))
    } else {
        panic!("Not multiplication!")
    }
}

fn evaluate(element: &Expression, environment: &Environment) -> i32 {
    match element {
        Expression::Add(_) => evalute_addition(&element, environment),
        Expression::Multiply(_) => evaluate_multiplication(&element, environment),
        Expression::Subtract(_) => evaluate_substraction(&element, environment),
        Expression::Variable(v) => evaluate(environment.value_for_key(v.to_string()), environment),
        Expression::Number(n) => *n,
        _ => panic!("we havent done this yet")
    }
}

fn evaluate_evaluate(element: &Expression, environment: &Environment)  {
    match element {
        Expression::Add(_) => print_add(environment, element),
        Expression::Multiply(_) => print_multiply(environment, element),
        Expression::Subtract(_) => todo!(),
        Expression::Variable(_) => todo!(),
        Expression::Number(_) => (),
    }
}

// Not perfect by any means but will continue to perfect it on my own time
fn print_add(_environment: &Environment, element: &Expression) {
    if let Expression::Add(_v) = element {
        print!("(+ ");
        for i in _v {
            print!(" {:?}", i);    
        }
        print!(")");
    } else {
        panic!("Cant print that!");
    }
}

fn print_multiply(_environment: &Environment, element: &Expression) {
    if let Expression::Multiply(_v) = element {
        print!("(* ");
        for i in _v {
            print!(" {:?}", i);    
        }
        print!(")");
    } else {
        panic!("Cant print that!");
    }
}

fn print_divide(_environment: &Environment, element: &Expression) {
    if let Expression::Add(_v) = element {
        print!("(/ ");
        for i in _v {
            print!(" {:?}", i);    
        }
        print!(")");
    } else {
        panic!("Cant print that!");
    }
}

fn print_substract(_environment: &Environment, element: &Expression) {
    if let Expression::Add(_v) = element {
        print!("(- ");
        for i in _v {
            print!(" {:?}", i);    
        }
        print!(")");
    } else {
        panic!("Cant print that!");
    }
}

fn main() {
    let mut expression = Vec::<Expression>::new();
    // expression.push(Expression::Add);
    expression.push(Expression::Number(3));
    expression.push(Expression::Number(4));
    let addition = Expression::Add(expression);
    let env = crate::Environment::new();
    println!("{:?}", evaluate_evaluate(&addition, &env));
    let sum = evaluate(&addition, &Environment::new());
    println!("{}", sum);

    let v = vec![Expression::Number(2), Expression::Number(3)];
    let a = Expression::Add(v);
    let z = Expression::Multiply(vec![a, Expression::Number(7)]);
    // act
    print!("{:?}", evaluate_evaluate( &z, &env));

    let mut expression2 = Vec::<Expression>::new();
    expression2.push(Expression::Number(21));
    expression2.push(Expression::Number(2));
    expression2.push(addition);
    let multiplication = Expression::Multiply(expression2);
    let product = evaluate(&multiplication, &Environment::new());
    println!("{}", product);

    let mut expression3 = Vec::<Expression>::new();
    // expression3.push(Expression::Subtract(()));
    expression3.push(Expression::Number(20));
    expression3.push(Expression::Number(2));
    let substraction = Expression::Subtract(expression3);
    let difference = evaluate(&substraction, &Environment::new());
    println!("{}", difference);

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{evalute_addition};
    use crate::{Expression};
    use crate::{evaluate_multiplication};
    use crate::{evaluate_substraction};


    #[test]
    fn it_does_nothing_useful() {
        // arrange
        // act
        // assert
        assert_eq!(4,2+2);
    }

    #[test]
    fn test_addition() {
        use crate::evalute_addition;
        use crate::Expression;
        // arrange
        let v = vec![Expression::Number(2), Expression::Number(2)];
        let a = Expression::Add(v);
        // act
        let sum = evalute_addition(&a, &crate::Environment::new());
        // assert
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_multiplication() {
        use crate::evaluate_multiplication;
        use crate::Expression;
        // arrange 
        let  q = vec![Expression::Number(2), Expression::Number(2)];
        let n = vec![Expression::Number(3), Expression::Number(4),
        Expression::Number(5), Expression::Add(q) 
        ];
        let m = Expression::Multiply(n);
        // act
        let product = evaluate_multiplication(&m, &crate::Environment::new());
        // assert
        assert_eq!(product, 240)
    }

    #[test]
    fn test_subtraction() {
        use crate::evaluate_substraction;
        use crate::Expression;
        // arrange
        let v = vec![Expression::Number(2), Expression::Number(2)];
        let a = Expression::Subtract(v);
        // act
        let diff = evaluate_substraction(&a, &crate::Environment::new());
        // assert
        assert_eq!(diff, 0);
    }

    #[test]
    fn test_new_environment() {
        //arrange
        let mut new_env = crate::Environment::new();
        new_env.add_value_for_key(String::from("foo"), &crate::Expression::Number(42));
        //act
        let expr = new_env.value_for_key(String::from("foo"));
        //assert 
        if let crate::Expression::Number(value) = expr {
            assert_eq!(42, *value);
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_value_for_key() {
        //arrange
        let mut new_env = crate::Environment::new();
        new_env.add_value_for_key(String::from("foo"), &crate::Expression::Number(42));
        //act
        let expr = new_env.value_for_key(String::from("foo"));
        //assert
        if let crate::Expression::Number(value) = expr {
            assert_eq!(42, *value);
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_addition_with_variable() {
        //arrange
        let mut new_env = crate::Environment::new();
        new_env.add_value_for_key(String::from("foo"), &crate::Expression::Number(42));
        let v = vec![Expression::Variable(String::from("foo")), Expression::Number(2)];
        let add = Expression::Add(v);
        //act
        let sum = crate::evaluate(&add, &new_env);
        //assert
        assert_eq!(44, sum);
    }
    #[test]
    fn test_printing_expression() {
        use crate::print_add;
        use crate::evaluate_evaluate;
        use crate::Expression;
        // arrange
        let mut env = crate::Environment::new();
        let v = vec![Expression::Number(2), Expression::Number(2)];
        let a = Expression::Add(v);
        let z = Expression::Multiply(vec![a, Expression::Number(2)]);
        // act
        let eq = evaluate_evaluate( &z, &env);
        // assert
        assert_eq!(eq, println!("(+ {:?})", eq));
    }
    #[test]
    fn hashy_mappy(){
        //arrange
        let mut new_env = crate::Environment{map: HashMap::new(), parent: None, cost: None, difference: None };
        new_env.map.insert(String::from("a"), Expression::Number(5));
        // let num = crate::Expression::Number(10);
        let v = vec![Expression::Variable(String::from("a")), Expression::Number(10)];
        let mult = Expression::Multiply(v);
        let add = Expression::Add(vec![mult, Expression::Number(3)]);
        //act
        let sum = crate::evaluate(&add, &new_env);
        //assert
        assert_eq!(sum, 53);
    }
}