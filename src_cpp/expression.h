#pragma once

#include <memory>

#include "kiwisolver/src/lib.rs.h"
#include "../upstream/kiwi/kiwi/expression.h"

namespace kiwi
{
    std::unique_ptr<Expression> new_expression(::rust::Slice<const Term *> termSlice, double constant)
    {
        std::vector<Term> terms;
        terms.reserve(termSlice.size());
        for (size_t i = 0; i < termSlice.size(); ++i)
        {
            terms.push_back(*termSlice[i]);
        }
        return std::make_unique<Expression>(terms, constant);
    }
}