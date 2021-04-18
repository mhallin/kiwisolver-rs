#![allow(unused_unsafe)]

mod constraint;
mod expression;
mod solver;
mod sys;
mod term;
mod terms;
mod variable;

#[cfg(test)]
mod test;

pub const STRENGTH_REQUIRED: f64 = 1000.0 * 1000000.0 + 1000.0 * 1000.0 + 1000.0;
pub const STRENGTH_STRONG: f64 = 1.0 * 1000000.0;
pub const STRENGTH_MEDIUM: f64 = 1.0 * 1000.0;
pub const STRENGTH_WEAK: f64 = 1.0;

pub use constraint::{Constraint, RelationalOperator};
pub use expression::Expression;
pub use solver::{Solver, SolverError};
pub use term::Term;
pub use terms::Terms;
pub use variable::Variable;
