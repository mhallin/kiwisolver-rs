#[cxx::bridge(namespace = "kiwi")]
#[allow(dead_code)]
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

    unsafe extern "C++" {
        include!("kiwisolver/upstream/kiwi/kiwi/solver.h");
        include!("kiwisolver/src_cpp/solver.h");

        type Solver;

        fn new_solver() -> UniquePtr<Solver>;

        #[rust_name = "add_constraint"]
        fn addConstraint(self: Pin<&mut Solver>, constraint: &Constraint);
        #[rust_name = "remove_constraint"]
        fn removeConstraint(self: Pin<&mut Solver>, constraint: &Constraint);
        #[rust_name = "has_constraint"]
        fn hasConstraint(&self, constraint: &Constraint) -> bool;

        #[rust_name = "update_variables"]
        fn updateVariables(self: Pin<&mut Solver>);

        fn reset(self: Pin<&mut Solver>);
        fn dump(self: Pin<&mut Solver>);
    }
}

const STRENGTH_REQUIRED: f64 = 1000.0 * 1000000.0 + 1000.0 * 1000.0 + 1000.0;
const STRENGTH_STRONG: f64 = 1.0 * 1000000.0;
const STRENGTH_MEDIUM: f64 = 1.0 * 1000.0;
const STRENGTH_WEAK: f64 = 1.0;

#[cfg(test)]
mod test {
    use cxx::let_cxx_string;

    use super::ffi;

    #[test]
    fn create_variable() {
        unsafe {
            let_cxx_string!(name = "var_name");

            let var = ffi::new_variable(&name);
            assert_eq!(var.name(), name.as_ref().get_ref());
        }
    }

    #[test]
    fn create_term() {
        unsafe {
            let_cxx_string!(name = "var_name");

            let var = ffi::new_variable(&name);
            let _term = ffi::new_term(&var, 1.0);
        }
    }

    #[test]
    fn create_expression() {
        unsafe {
            let_cxx_string!(name = "var_name");

            let var = ffi::new_variable(&name);
            let term = ffi::new_term(&var, 1.0);
            let terms = vec![term.as_ref().unwrap() as *const ffi::Term];
            let _expr = ffi::new_expression(&terms, 0.0);
        }
    }

    #[test]
    fn create_constraint() {
        unsafe {
            let_cxx_string!(name = "var_name");

            let var = ffi::new_variable(&name);
            let term = ffi::new_term(&var, 1.0);
            let terms = vec![term.as_ref().unwrap() as *const ffi::Term];
            let expr = ffi::new_expression(&terms, 0.0);
            let _constraint = ffi::new_constraint(
                &expr,
                ffi::RelationalOperator::OP_GE,
                super::STRENGTH_REQUIRED,
            );
        }
    }

    #[test]
    fn create_solver() {
        unsafe {
            let _solver = ffi::new_solver();
        }
    }
}
