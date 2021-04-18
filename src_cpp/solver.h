#pragma once

#include <memory>

#include "../upstream/kiwi/kiwi/solver.h"

namespace kiwi
{
    enum class SolverError : uint8_t
    {
        NoError,
        DuplicateConstraint,
        UnknownConstraint,
        UnsatisfiableConstraint,
    };

    std::unique_ptr<Solver> new_solver()
    {
        return std::make_unique<Solver>();
    }

    SolverError add_constraint(Solver &solver, const Constraint &constraint)
    {
        try
        {
            solver.addConstraint(constraint);
        }
        catch (DuplicateConstraint &)
        {
            return SolverError::DuplicateConstraint;
        }
        catch (UnsatisfiableConstraint &)
        {
            return SolverError::UnsatisfiableConstraint;
        }
        return SolverError::NoError;
    }

    SolverError remove_constraint(Solver &solver, const Constraint &constraint)
    {
        try
        {
            solver.removeConstraint(constraint);
        }
        catch (UnknownConstraint &)
        {
            return SolverError::UnknownConstraint;
        }
        return SolverError::NoError;
    }
}
