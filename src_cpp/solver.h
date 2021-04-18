#pragma once

#include <memory>

#include "../upstream/kiwi/kiwi/solver.h"

namespace kiwi
{
    std::unique_ptr<Solver> new_solver()
    {
        return std::make_unique<Solver>();
    }
}
