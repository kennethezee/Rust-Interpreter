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

#[derive(Clone)]
#[derive(Debug)]
pub struct Environment {
    map: HashMap<String, Expression>,
    parent: Option<Box<Environment>>,
    // HW
    cost: Option<HashMap<i32, i32>>,
    difference: Option<i32>,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expression),
    Print(String),
    PrintV(Expression)
}

impl Environment {

    pub fn new() -> Environment {
        Environment{map: HashMap::new(), parent: None, difference: None, cost: None}
    }

    pub fn value_for_key(self: &Environment, key: String) -> &Expression {
        if let Some(env) = &self.parent {
            &self.map.get(&key).unwrap_or(env.value_for_key(key))
        } else {
            &self.map.get(&key).unwrap_or(&Expression::Number(0))
        }
       
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

pub fn resolve_statement(s: &Statement, e:&mut Environment) {
    match s {
        Statement::Print(t) => print!("{t}"),
        Statement::Assignment(t, exp) => e.add_value_for_key(t.to_string(), exp),
        Statement::PrintV(exp) => print!("{}", evaluate(exp, e)),
        _ => panic!("did not support this statement")
    }
}

fn main() {
    //set up environment
    let mut env = Environment::new();
    // start with a print
    let print_statement = Statement::Print("hello world\n".to_string());
    resolve_statement(&print_statement, &mut env);

    let assign_statement = Statement::Assignment("x".to_string(), Expression::Number(5));
    resolve_statement(&assign_statement, &mut env);

    // let addition = Expression::Add(vec![
    //     Expression::Variable("x".to_string()),
    //     Expression::Number(3)
    // ]);
    // let print_statement = Statement::PrintV(addition);
    // resolve_statement(&print_statement, &mut env);

    // HW
    let square = Expression::Multiply(vec![
        Expression::Variable("x".to_string()),
        Expression::Variable("x".to_string()),
    ]);

    let print_statement = Statement::PrintV(square);
    resolve_statement(&print_statement, &mut env);


    
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::read;

    use crate::{evaluate, evalute_addition};
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
    #[test]
    fn mult_env(){
        //arrange
        let mut e0 = crate::Environment::new();
        let mut e1 = crate::Environment::new();
        e0.add_value_for_key(("x".to_string()), &crate::Expression::Number(2));
        e1.parent = Some(Box::new(e0));
        //act
        let v = e1.value_for_key("x".to_string());
        let value = crate::evaluate(&v, &e1);
        //assert
        assert_eq!(value, 2)
    }
    #[test]
    fn test_square_function() {
        //arrange 
        let mut e0 = crate::Environment::new();
        let mut e1 = crate::Environment::new();
        let square = Expression::Multiply(vec![
            Expression::Variable("x".to_string()),
            Expression::Variable("x".to_string()),
        ]);
        e1.add_value_for_key("square".to_string(), &square);
        e0.add_value_for_key("x".to_string(), &crate::Expression::Number(2));
        e0.parent = Some(Box::new(e1));
        //act
        let v = e0.value_for_key("square".to_string());
        let value = crate::evaluate(&v, &e0);
        //assert
        assert_eq!(value, 4)
    }
    #[test]
    fn test_sum_of_squares_function() {
        //arrange
        let mut e0 = crate::Environment::new();
        let mut e1 = crate::Environment::new();
        let mut e2 = crate::Environment::new();
        let square = Expression::Multiply(vec![
            Expression::Variable("x".to_string()),
            Expression::Variable("x".to_string()),
        ]);
        let sum_of_squares =  Expression::Add(
            vec![
                Expression::Variable("square".to_string()),
                Expression::Variable("square".to_string()),
            ]
        );
        e1.add_value_for_key("square".to_string(), &square);
        e1.add_value_for_key("sum_of_squares".to_string(), &sum_of_squares);
        e0.add_value_for_key("x".to_string(), &Expression::Number(2));
        e0.parent = Some(Box::new(e1));
        //act
        let v = e0.value_for_key("sum_of_squares".to_string());
        let value = crate::evaluate(&v, &e0);
        //assert
        assert_eq!(value, 8);
    }
    #[test]
    fn test_sum_of_squares_applicative() {
        //arrange
        let mut e0 = crate::Environment::new();
        let mut e1 = crate::Environment::new();
        e1.add_value_for_key("x".to_string(), &Expression::Number(2));
        let mut e2 = crate::Environment::new();
        e2.add_value_for_key("x".to_string(), &Expression::Number(3));
        let square = Expression::Multiply(vec![
            Expression::Variable("x".to_string()),
            Expression::Variable("x".to_string()),
        ]);
        e0.add_value_for_key(("square".to_string()), &square);
        e1.parent = Some(Box::new(e0.clone()));
        e2.parent = Some(Box::new(e0.clone()));
        let sum_of_squares =  Expression::Add(
            vec![
                Expression::Number(crate::evaluate(e1.value_for_key("square".to_string()), &e1)),
                Expression::Number(crate::evaluate(e2.value_for_key("square".to_string()), &e2)),
            ]
        );
        e1.add_value_for_key("square".to_string(), &square);
        e1.add_value_for_key("sum_of_squares".to_string(), &sum_of_squares);
        e0.add_value_for_key("x".to_string(), &Expression::Number(2));
        e0.parent = Some(Box::new(e1));
        //act
        let v = e0.value_for_key("sum_of_squares".to_string());
        let value = crate::evaluate(&v, &e0);
        //assert
        assert_eq!(value, 13); 
    }
}