use crate::{Constraint, RelationalOperator, Solver, SolverError, Variable, STRENGTH_REQUIRED};

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
