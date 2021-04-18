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

        fn add_expressions(lhs: &Expression, rhs: &Expression) -> UniquePtr<Expression>;
        fn sub_expressions(lhs: &Expression, rhs: &Expression) -> UniquePtr<Expression>;
        fn add_expr_double(lhs: &Expression, rhs: f64) -> UniquePtr<Expression>;
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

pub(crate) use ffi::*;
