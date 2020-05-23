#include <gtest/gtest.h>
#include <adv.hpp>

TEST(ADouble, arithmetic_ops)
{
	adv::AContext ctx;
	auto a = ctx.new_independent();
	auto b = ctx.new_independent();

	a + b;
	a - b;
	a * b;
	a / b;

	a + 1.0;
	a - 1.0;
	a * 1.0;
	a / 1.0;

	1.0 + a;
	1.0 - a;
	1.0 * a;
	1.0 / a;
}

TEST(ADouble, unary_functions)
{
	adv::AContext ctx;
	auto x = ctx.new_independent();

	adv::sin(x);
	adv::cos(x);
	adv::tan(x);
	adv::abs(x);
	adv::exp(x);
	adv::ln(x);
}

TEST(ADouble, binary_functions)
{
	adv::AContext ctx;
	auto a = ctx.new_independent();
	auto b = ctx.new_independent();

	adv::min(a, b);
	adv::min(a, 1.0);
	adv::min(1.0, a);
	ASSERT_EQ(0.0, adv::min(1.0, 0.0));

	adv::max(a, b);
	adv::max(a, 1.0);
	adv::max(1.0, a);
	adv::max(1.0, 0.0);
	ASSERT_EQ(1.0, adv::max(1.0, 0.0));
}
