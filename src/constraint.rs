use std::rc::Rc;

use cxx::UniquePtr;

use crate::{Expression, sys};

#[derive(Copy, Clone)]
pub enum RelationalOperator {
    LessThanEqualZero,
    EqualZero,
    GreaterThanEqualZero,
}

#[derive(Clone)]
pub struct Constraint {
    constraint: Rc<UniquePtr<sys::Constraint>>,
}

impl Constraint {
    pub fn new(expression: &Expression, op: RelationalOperator, strength: f64) -> Self {
        let op = match op {
            RelationalOperator::LessThanEqualZero => sys::RelationalOperator::OP_LE,
            RelationalOperator::EqualZero => sys::RelationalOperator::OP_EQ,
            RelationalOperator::GreaterThanEqualZero => sys::RelationalOperator::OP_GE,
        };

        Self {
            constraint: Rc::new(unsafe { sys::new_constraint(expression.expr(), op, strength) }),
        }
    }

    pub(crate) fn constraint(&self) -> &UniquePtr<sys::Constraint> {
        &self.constraint
    }
}
