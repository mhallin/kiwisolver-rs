#pragma once

#include <memory>

#include "../upstream/kiwi/kiwi/term.h"

namespace kiwi
{
    std::unique_ptr<Term> new_term(const Variable &variable, double coefficient)
    {
        return std::make_unique<Term>(variable, coefficient);
    }
}