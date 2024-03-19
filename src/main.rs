// Link to my forked repo
// https://github.com/kennethezee/interpreter
// Used the same notes file from class but added a new expression to fulfull HW

//##################################################
// HOMEWORK 13 is at the bottom of the code ...
//##################################################

#[derive(Clone)]
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    // Divide, // New Expression for HW
    Number(i32),
}

pub fn evalute_addition(e: Expression) -> i32 {
    if let Expression::Add(v) = e {
        let iter = v.iter();
        iter.fold(0, |t, e| t + evaluate(e))
    } else {
        panic!("not addition!")
    }
}

pub fn evaluate_multiplication(e: Expression) -> i32 {
    if let Expression::Multiply(v) = e {
        let iter = v.iter();
        iter.fold(1, |t, e| t * evaluate(e))
    } else {
        panic!("Not multiplication!")
    }
}

fn evaluate(element: &Expression) -> i32 {
    match element {
        Expression::Add(prim) => {
            let iter = prim.iter();
            iter.fold(0, |t, e| t + evaluate(e))
        },
        Expression::Multiply(prim) => {
            let iter = prim.iter();
            iter.fold(1, |t, e| t * evaluate(e))
        },
        Expression::Subtract(prim) => {
            let mut iter = prim.iter();
            let first = iter.next().unwrap();
            iter.fold(evaluate(first), |t, e| t - evaluate(e))
        },
        Expression::Number(n) => *n
    }
}

fn main() {
    let mut expression = Vec::<Expression>::new();
    // expression.push(Expression::Add);
    expression.push(Expression::Number(3));
    expression.push(Expression::Number(4));
    let addition = Expression::Add(expression);
    let sum = evaluate(&addition);
    println!("{}", sum);

    let mut expression2 = Vec::<Expression>::new();
    expression2.push(Expression::Number(21));
    expression2.push(Expression::Number(2));
    expression2.push(addition);
    let multiplication = Expression::Multiply(expression2);
    let product = evaluate(&multiplication);
    println!("{}", product);

    let mut expression3 = Vec::<Expression>::new();
    // expression3.push(Expression::Subtract(()));
    expression3.push(Expression::Number(20));
    expression3.push(Expression::Number(2));
    let substraction = Expression::Subtract(expression3);
    let difference = evaluate(&substraction);
    println!("{}", difference);
}

#[cfg(test)]
mod tests {
    use crate::{evalute_addition};
    use crate::{Expression};
    use crate::{evaluate_multiplication};


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
        let sum = evalute_addition(a);
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
        let product = evaluate_multiplication(m);
        // assert
        assert_eq!(product, 240)
    }
}