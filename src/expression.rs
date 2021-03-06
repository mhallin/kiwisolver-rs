use std::{
    ops::{Add, Mul, Sub},
    rc::Rc,
};

use cxx::UniquePtr;

use crate::{sys, Term, Terms};

#[derive(Clone)]
pub struct Expression {
    expr: Rc<UniquePtr<sys::Expression>>,
}

impl Expression {
    pub fn new(terms: &[&Term], constant: f64) -> Self {
        unsafe {
            let terms = terms
                .iter()
                .map(|t| t.term().as_ref().unwrap() as *const sys::Term as usize)
                .collect::<Vec<_>>();
            Self {
                expr: Rc::new(sys::new_expression(&terms, constant)),
            }
        }
    }

    pub fn from_terms(terms: Terms, constant: f64) -> Self {
        unsafe {
            let terms = terms
                .terms()
                .iter()
                .map(|t| t.term().as_ref().unwrap() as *const sys::Term as usize)
                .collect::<Vec<_>>();
            Self {
                expr: Rc::new(sys::new_expression(&terms, constant)),
            }
        }
    }

    pub(crate) fn expr(&self) -> &UniquePtr<sys::Expression> {
        &self.expr
    }
}

impl From<f64> for Expression {
    fn from(val: f64) -> Self {
        Expression::new(&[], val)
    }
}

impl Add<Expression> for Expression {
    type Output = Expression;

    fn add(self, rhs: Expression) -> Self::Output {
        unsafe {
            let expr = sys::add_expressions(self.expr(), rhs.expr());
            Expression {
                expr: Rc::new(expr),
            }
        }
    }
}

impl Sub<Expression> for Expression {
    type Output = Expression;

    fn sub(self, rhs: Expression) -> Self::Output {
        unsafe {
            let expr = sys::sub_expressions(self.expr(), rhs.expr());
            Expression {
                expr: Rc::new(expr),
            }
        }
    }
}

impl Add<f64> for Expression {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        unsafe {
            let expr = sys::add_expr_double(self.expr(), rhs);
            Expression {
                expr: Rc::new(expr),
            }
        }
    }
}

impl Add<Expression> for f64 {
    type Output = Expression;

    fn add(self, rhs: Expression) -> Self::Output {
        unsafe {
            let expr = sys::add_expr_double(rhs.expr(), self);
            Expression {
                expr: Rc::new(expr),
            }
        }
    }
}

impl Sub<f64> for Expression {
    type Output = Expression;

    fn sub(self, rhs: f64) -> Self::Output {
        unsafe {
            let expr = sys::add_expr_double(self.expr(), -rhs);
            Expression {
                expr: Rc::new(expr),
            }
        }
    }
}

impl Mul<f64> for Expression {
    type Output = Expression;

    fn mul(self, rhs: f64) -> Self::Output {
        unsafe {
            let expr = sys::mul_expr_double(self.expr(), rhs);
            Expression {
                expr: Rc::new(expr),
            }
        }
    }
}

impl Mul<Expression> for f64 {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Self::Output {
        unsafe {
            let expr = sys::mul_expr_double(rhs.expr(), self);
            Expression {
                expr: Rc::new(expr),
            }
        }
    }
}
