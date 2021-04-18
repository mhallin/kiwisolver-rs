use std::{
    ops::{Add, Sub},
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
                .into_iter()
                .map(|t| t.term().as_ref().unwrap() as *const sys::Term)
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
                .map(|t| t.term().as_ref().unwrap() as *const sys::Term)
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
