#![allow(unused_unsafe)]

use std::{fmt::Debug, ops::{Add, Mul, Sub}, rc::Rc};

use cxx::UniquePtr;

#[cxx::bridge(namespace = "kiwi")]
mod ffi {
    unsafe extern "C++" {
        include!("kiwisolver/upstream/kiwi/kiwi/variable.h");
        include!("kiwisolver/src_cpp/variable.h");

        type Variable;

        fn new_variable(name: &CxxString) -> UniquePtr<Variable>;

        fn name(&self) -> &CxxString;
        fn value(&self) -> f64;
    }

    unsafe extern "C++" {
        include!("kiwisolver/upstream/kiwi/kiwi/term.h");
        include!("kiwisolver/src_cpp/term.h");

        type Term;

        fn new_term(variable: &Variable, coefficient: f64) -> UniquePtr<Term>;
    }

    unsafe extern "C++" {
        include!("kiwisolver/upstream/kiwi/kiwi/expression.h");
        include!("kiwisolver/src_cpp/expression.h");

        type Expression;

        fn new_expression(terms: &[*const Term], constant: f64) -> UniquePtr<Expression>;
    }

    #[repr(u32)]
    enum RelationalOperator {
        OP_LE,
        OP_GE,
        OP_EQ,
    }

    unsafe extern "C++" {
        include!("kiwisolver/upstream/kiwi/kiwi/constraint.h");
        include!("kiwisolver/src_cpp/constraint.h");

        type Constraint;
        type RelationalOperator;

        fn new_constraint(
            expression: &Expression,
            op: RelationalOperator,
            strength: f64,
        ) -> UniquePtr<Constraint>;
    }

    #[repr(u8)]
    enum SolverError {
        NoError,
        DuplicateConstraint,
        UnknownConstraint,
        UnsatisfiableConstraint,
    }

    unsafe extern "C++" {
        include!("kiwisolver/src_cpp/solver.h");

        type SolverError;
    }

    unsafe extern "C++" {
        include!("kiwisolver/upstream/kiwi/kiwi/solver.h");
        include!("kiwisolver/src_cpp/solver.h");

        type Solver;

        fn new_solver() -> UniquePtr<Solver>;

        fn add_constraint(solver: Pin<&mut Solver>, constraint: &Constraint) -> SolverError;
        fn remove_constraint(solver: Pin<&mut Solver>, constraint: &Constraint) -> SolverError;

        #[rust_name = "has_constraint"]
        fn hasConstraint(&self, constraint: &Constraint) -> bool;

        #[rust_name = "update_variables"]
        fn updateVariables(self: Pin<&mut Solver>);

        fn reset(self: Pin<&mut Solver>);
        fn dump(self: Pin<&mut Solver>);
    }
}

pub const STRENGTH_REQUIRED: f64 = 1000.0 * 1000000.0 + 1000.0 * 1000.0 + 1000.0;
pub const STRENGTH_STRONG: f64 = 1.0 * 1000000.0;
pub const STRENGTH_MEDIUM: f64 = 1.0 * 1000.0;
pub const STRENGTH_WEAK: f64 = 1.0;

#[derive(Clone)]
pub struct Variable {
    var: Rc<UniquePtr<ffi::Variable>>,
}

#[derive(Clone)]
pub struct Term {
    term: Rc<UniquePtr<ffi::Term>>,
}

#[derive(Clone)]
pub struct Expression {
    expr: Rc<UniquePtr<ffi::Expression>>,
}

#[derive(Clone)]
pub struct Constraint {
    constraint: Rc<UniquePtr<ffi::Constraint>>,
}

pub struct Solver {
    solver: UniquePtr<ffi::Solver>,
}

#[derive(Copy, Clone)]
pub enum RelationalOperator {
    LessThanEqualZero,
    EqualZero,
    GreaterThanEqualZero,
}

#[derive(Copy, Clone)]
pub enum SolverError {
    BadRequiredStrength,
    DuplicateConstraint,
    UnknownConstraint,
    UnsatisfiableConstraint,
}

impl Variable {
    pub fn new(name: &str) -> Self {
        cxx::let_cxx_string!(name = name);

        Self {
            var: Rc::new(unsafe { ffi::new_variable(&name) }),
        }
    }

    pub fn value(&self) -> f64 {
        self.var.value()
    }

    pub fn name(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.var.name().as_bytes()) }
    }
}

impl Term {
    pub fn new(variable: &Variable, coefficient: f64) -> Self {
        Self {
            term: Rc::new(unsafe { ffi::new_term(&variable.var, coefficient) }),
        }
    }
}

impl Expression {
    pub fn new(terms: &[&Term], constant: f64) -> Self {
        unsafe {
            let terms = terms
                .into_iter()
                .map(|t| (&t.term as &UniquePtr<ffi::Term>).as_ref().unwrap() as *const ffi::Term)
                .collect::<Vec<_>>();
            Self {
                expr: Rc::new(ffi::new_expression(&terms, constant)),
            }
        }
    }
}

impl Constraint {
    pub fn new(expression: &Expression, op: RelationalOperator, strength: f64) -> Self {
        let op = match op {
            RelationalOperator::LessThanEqualZero => ffi::RelationalOperator::OP_LE,
            RelationalOperator::EqualZero => ffi::RelationalOperator::OP_EQ,
            RelationalOperator::GreaterThanEqualZero => ffi::RelationalOperator::OP_GE,
        };

        Self {
            constraint: Rc::new(unsafe { ffi::new_constraint(&expression.expr, op, strength) }),
        }
    }
}

impl Into<Result<(), SolverError>> for ffi::SolverError {
    fn into(self) -> Result<(), SolverError> {
        match self {
            ffi::SolverError::NoError => Ok(()),
            ffi::SolverError::DuplicateConstraint => Err(SolverError::DuplicateConstraint),
            ffi::SolverError::UnknownConstraint => Err(SolverError::UnknownConstraint),
            ffi::SolverError::UnsatisfiableConstraint => Err(SolverError::UnsatisfiableConstraint),
            value => panic!("Unknown error from solver: {}", value.repr),
        }
    }
}

impl Solver {
    pub fn new() -> Self {
        Self {
            solver: unsafe { ffi::new_solver() },
        }
    }

    pub fn add_constraint(&mut self, constraint: &Constraint) -> Result<(), SolverError> {
        let solver = self.solver.pin_mut();
        unsafe { ffi::add_constraint(solver, &constraint.constraint) }.into()
    }

    pub fn remove_constraint(&mut self, constraint: &Constraint) -> Result<(), SolverError> {
        let solver = self.solver.pin_mut();
        unsafe { ffi::remove_constraint(solver, &constraint.constraint) }.into()
    }

    pub fn has_constraint(&mut self, constraint: &Constraint) -> bool {
        self.solver.has_constraint(&constraint.constraint)
    }

    pub fn update_variables(&mut self) {
        let solver = self.solver.pin_mut();
        solver.update_variables();
    }

    pub fn reset(&mut self) {
        let solver = self.solver.pin_mut();
        solver.reset();
    }

    pub fn dump(&mut self) {
        let solver = self.solver.pin_mut();
        solver.dump();
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

#[cfg(test)]
mod test {
    use super::{Constraint, RelationalOperator, Solver, SolverError, Variable, STRENGTH_REQUIRED};

    #[test]
    fn create_variable() {
        let var = Variable::new("var_name");

        assert_eq!(var.name(), "var_name");
    }

    #[test]
    fn create_term() {
        let var = Variable::new("var_name");
        let _term = var * 1.0;
    }

    #[test]
    fn create_expression() {
        let var = Variable::new("var_name");
        let _expr = var * 1.0 + 0.0;
    }

    #[test]
    fn create_constraint() {
        let expr = Variable::new("var_name") * 1.0 + 0.0;
        let _constraint = Constraint::new(
            &expr,
            RelationalOperator::GreaterThanEqualZero,
            STRENGTH_REQUIRED,
        );
    }

    #[test]
    fn create_solver() {
        let var = Variable::new("var_name");
        let expr = &var * 1.0 - 5.0;
        let constraint = Constraint::new(
            &expr,
            RelationalOperator::GreaterThanEqualZero,
            STRENGTH_REQUIRED,
        );

        let mut solver = Solver::new();
        assert!(solver.add_constraint(&constraint).is_ok());
        solver.update_variables();

        assert_eq!(var.value(), 5.0);
    }

    #[test]
    fn solver_duplicate_constraints() {
        let var = Variable::new("var_name");
        let expr = &var * 1.0 - 5.0;
        let constraint = Constraint::new(
            &expr,
            RelationalOperator::GreaterThanEqualZero,
            STRENGTH_REQUIRED,
        );

        let mut solver = Solver::new();
        assert!(solver.add_constraint(&constraint).is_ok());
        assert!(matches!(
            solver.add_constraint(&constraint),
            Err(SolverError::DuplicateConstraint)
        ));
    }

    #[test]
    fn remove_constraint() {
        let var = Variable::new("var_name");
        let expr = &var * 1.0 - 5.0;
        let constraint = Constraint::new(
            &expr,
            RelationalOperator::GreaterThanEqualZero,
            STRENGTH_REQUIRED,
        );

        let mut solver = Solver::new();
        assert!(solver.add_constraint(&constraint).is_ok());

        assert!(solver.remove_constraint(&constraint).is_ok());
        assert!(matches!(
            solver.remove_constraint(&constraint),
            Err(SolverError::UnknownConstraint)
        ));
    }
}
