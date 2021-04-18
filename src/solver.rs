use std::fmt::Display;

use cxx::UniquePtr;

use crate::{sys, Constraint};

pub struct Solver {
    solver: UniquePtr<sys::Solver>,
}

#[derive(Copy, Clone, Debug)]
pub enum SolverError {
    BadRequiredStrength,
    DuplicateConstraint,
    UnknownConstraint,
    UnsatisfiableConstraint,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            solver: unsafe { sys::new_solver() },
        }
    }

    pub fn add_constraint(&mut self, constraint: &Constraint) -> Result<(), SolverError> {
        let solver = self.solver.pin_mut();
        unsafe { sys::add_constraint(solver, constraint.constraint()) }.into()
    }

    pub fn remove_constraint(&mut self, constraint: &Constraint) -> Result<(), SolverError> {
        let solver = self.solver.pin_mut();
        unsafe { sys::remove_constraint(solver, constraint.constraint()) }.into()
    }

    pub fn has_constraint(&mut self, constraint: &Constraint) -> bool {
        self.solver.has_constraint(constraint.constraint())
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

impl Into<Result<(), SolverError>> for sys::SolverError {
    fn into(self) -> Result<(), SolverError> {
        match self {
            sys::SolverError::NoError => Ok(()),
            sys::SolverError::DuplicateConstraint => Err(SolverError::DuplicateConstraint),
            sys::SolverError::UnknownConstraint => Err(SolverError::UnknownConstraint),
            sys::SolverError::UnsatisfiableConstraint => Err(SolverError::UnsatisfiableConstraint),
            value => panic!("Unknown error from solver: {}", value.repr),
        }
    }
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            SolverError::BadRequiredStrength => {
                "A required strength cannot be used in this context."
            }
            SolverError::DuplicateConstraint => {
                "The constraint has already been added to the solver."
            }
            SolverError::UnknownConstraint => "The constraint has not been added to the solver.",
            SolverError::UnsatisfiableConstraint => "The constraint can not be satisfied.",
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for SolverError {}
