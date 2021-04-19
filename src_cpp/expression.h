#pragma once

#include <memory>

#include "kiwisolver/src/sys.rs.h"
#include "../upstream/kiwi/kiwi/expression.h"
#include "../upstream/kiwi/kiwi/symbolics.h"

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

    std::unique_ptr<Expression> add_expressions(const Expression &lhs, const Expression &rhs)
    {
        auto expr = lhs + rhs;
        return std::make_unique<Expression>(expr);
    }

    std::unique_ptr<Expression> sub_expressions(const Expression &lhs, const Expression &rhs)
    {
        auto expr = lhs - rhs;
        return std::make_unique<Expression>(expr);
    }

    std::unique_ptr<Expression> add_expr_double(const Expression &lhs, double v)
    {
        auto expr = lhs + v;
        return std::make_unique<Expression>(expr);
    }

    std::unique_ptr<Expression> mul_expr_double(const Expression &lhs, double v)
    {
        auto expr = lhs * v;
        return std::make_unique<Expression>(expr);
    }
}