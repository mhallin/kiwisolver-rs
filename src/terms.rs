use std::ops::{Add, Sub};

use crate::{Expression, Term};

#[derive(Clone)]
pub struct Terms {
    terms: Vec<Term>,
}

impl Terms {
    pub fn new(terms: Vec<Term>) -> Self {
        Self { terms }
    }

    pub fn terms(&self) -> &[Term] {
        &self.terms
    }
}

impl From<Terms> for Expression {
    fn from(val: Terms) -> Self {
        let terms = val.terms.iter().collect::<Vec<_>>();
        Expression::new(&terms, 0.0)
    }
}

impl Add<Term> for Term {
    type Output = Terms;

    fn add(self, rhs: Term) -> Self::Output {
        Terms::new(vec![self, rhs])
    }
}

impl Add<Term> for Terms {
    type Output = Terms;

    fn add(mut self, rhs: Term) -> Self::Output {
        self.terms.push(rhs);
        self
    }
}

impl Add<Terms> for Terms {
    type Output = Terms;

    fn add(mut self, rhs: Terms) -> Self::Output {
        self.terms.extend(rhs.terms);
        self
    }
}

impl Add<f64> for Terms {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        Expression::from_terms(self, rhs)
    }
}

impl Add<Terms> for f64 {
    type Output = Expression;

    fn add(self, rhs: Terms) -> Self::Output {
        Expression::from_terms(rhs, self)
    }
}

impl Sub<f64> for Terms {
    type Output = Expression;

    fn sub(self, rhs: f64) -> Self::Output {
        Expression::from_terms(self, -rhs)
    }
}
