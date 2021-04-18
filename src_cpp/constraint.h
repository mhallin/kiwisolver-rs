#pragma once

#include <memory>

#include "../upstream/kiwi/kiwi/constraint.h"

namespace kiwi
{
    std::unique_ptr<Constraint> new_constraint(const Expression &expr, RelationalOperator op, double strength)
    {
        return std::make_unique<Constraint>(expr, op, strength);
    }
}
