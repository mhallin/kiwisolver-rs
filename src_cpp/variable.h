#pragma once

#include <memory>

#include "../upstream/kiwi/kiwi/variable.h"

namespace kiwi
{
    std::unique_ptr<Variable> new_variable(const std::string &name)
    {
        return std::make_unique<Variable>(name);
    }
}