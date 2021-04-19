use std::{fmt::Debug, ops::Mul, rc::Rc};

use cxx::UniquePtr;

use crate::{sys, Term};

#[derive(Clone)]
pub struct Variable {
    var: Rc<UniquePtr<sys::Variable>>,
}

impl Variable {
    pub fn new(name: &str) -> Self {
        cxx::let_cxx_string!(name = name);

        Self {
            var: Rc::new(unsafe { sys::new_variable(&name) }),
        }
    }

    pub fn value(&self) -> f64 {
        self.var.value()
    }

    pub fn name(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.var.name().as_bytes()) }
    }

    pub(crate) fn var(&self) -> &UniquePtr<sys::Variable> {
        &self.var
    }
}

impl From<Variable> for Term {
    fn from(val: Variable) -> Self {
        Term::new(&val, 1.0)
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable({:?})", self.name())
    }
}

impl<'a> Mul<f64> for &'a Variable {
    type Output = Term;

    fn mul(self, rhs: f64) -> Self::Output {
        Term::new(self, rhs)
    }
}

impl Mul<f64> for Variable {
    type Output = Term;

    fn mul(self, rhs: f64) -> Self::Output {
        Term::new(&self, rhs)
    }
}

impl<'a> Mul<&'a Variable> for f64 {
    type Output = Term;

    fn mul(self, rhs: &'a Variable) -> Self::Output {
        Term::new(rhs, self)
    }
}

impl Mul<Variable> for f64 {
    type Output = Term;

    fn mul(self, rhs: Variable) -> Self::Output {
        Term::new(&rhs, self)
    }
}
