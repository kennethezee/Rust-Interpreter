// Link to my forked repo
// https://github.com/kennethezee/interpreter
// Used the same notes file from class but added a new expression to fulfull HW

//##################################################
// HOMEWORK 15 refactor Subtraction
//##################################################

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
    key: String,
    value: Expression
}

impl Environment {
    fn value_for_key(self: &Environment, key: &String) -> &Expression {
        if &self.key == key {
            &self.value
        } else {
            panic!("no key {key} found in environment");
        }
    }
    fn new() -> Environment {
        Environment{key: String::from(""), value: Expression::Number(0)}
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
        Expression::Variable(v) => evaluate(environment.value_for_key(v), environment),
        Expression::Number(n) => *n,
        _ => panic!("we havent done this yet")
    }
}

fn main() {
    let mut expression = Vec::<Expression>::new();
    // expression.push(Expression::Add);
    expression.push(Expression::Number(3));
    expression.push(Expression::Number(4));
    let addition = Expression::Add(expression);
    let sum = evaluate(&addition, &Environment::new());
    println!("{}", sum);

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

    // HOMEWORK 13: We love subexpressions!
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
        let new_env = crate::Environment::new();
        //act
        let expr = new_env.value;
        //assert
        if let crate::Expression::Number(value) = expr {
            assert_eq!(value, 0);
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_value_for_key() {
        //arrange
        let mut new_env = crate::Environment::new();
        new_env.key = String::from("foo");
        new_env.value = crate::Expression::Number(42);
        //act
        let expr = new_env.value_for_key(&String::from("foo"));
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
        new_env.key = String::from("foo");
        new_env.value = crate::Expression::Number(42);
        let v = vec![Expression::Variable(String::from("foo")), Expression::Number(2)];
        let add = Expression::Add(v);
        //act
        let sum = crate::evaluate(&add, &new_env);
        //assert
        assert_eq!(44, sum);
    }
    #[test]
    fn test_whiteboard_example() {
        //arrange
        let mut new_env = crate::Environment::new();
        new_env.key = String::from("a");
        new_env.value = crate::Expression::Number(10);
        let v = vec![Expression::Variable(String::from("a")), Expression::Number(5)];
        let mult = Expression::Multiply(v);
        let add = Expression::Add(vec![mult, Expression::Number(3)]);
        //act
        let sum = crate::evaluate(&add, &new_env);
        //assert
        assert_eq!(sum, 53);
    }
}