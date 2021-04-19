use std::{
    ops::{Add, Sub},
    rc::Rc,
};

use cxx::UniquePtr;

use crate::{sys, Expression, Terms, Variable};

#[derive(Clone)]
pub struct Term {
    term: Rc<UniquePtr<sys::Term>>,
}

impl Term {
    pub fn new(variable: &Variable, coefficient: f64) -> Self {
        Self {
            term: Rc::new(unsafe { sys::new_term(&variable.var(), coefficient) }),
        }
    }

    pub(crate) fn term(&self) -> &UniquePtr<sys::Term> {
        &self.term
    }
}

impl From<Term> for Terms {
    fn from(val: Term) -> Self {
        Terms::new(vec![val])
    }
}

impl<'a> Add<f64> for &'a Term {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        Expression::new(&[self], rhs)
    }
}

impl Add<f64> for Term {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        Expression::new(&[&self], rhs)
    }
}

impl<'a> Add<&'a Term> for f64 {
    type Output = Expression;

    fn add(self, rhs: &'a Term) -> Self::Output {
        Expression::new(&[rhs], self)
    }
}

impl Add<Term> for f64 {
    type Output = Expression;

    fn add(self, rhs: Term) -> Self::Output {
        Expression::new(&[&rhs], self)
    }
}

impl<'a> Sub<f64> for &'a Term {
    type Output = Expression;

    fn sub(self, rhs: f64) -> Self::Output {
        Expression::new(&[self], -rhs)
    }
}

impl Sub<f64> for Term {
    type Output = Expression;

    fn sub(self, rhs: f64) -> Self::Output {
        Expression::new(&[&self], -rhs)
    }
}

impl<'a> Sub<&'a Term> for f64 {
    type Output = Expression;

    fn sub(self, rhs: &'a Term) -> Self::Output {
        Expression::new(&[rhs], -self)
    }
}

impl Sub<Term> for f64 {
    type Output = Expression;

    fn sub(self, rhs: Term) -> Self::Output {
        Expression::new(&[&rhs], -self)
    }
}
