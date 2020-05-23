#include <gtest/gtest.h>
#include <adv.hpp>

TEST(AContext, init)
{
	adv::AContext ctx;
}

TEST(AContext, get_doubles)
{
	adv::AContext ctx;
	auto v1 = ctx.new_independent();
	ctx.set_dependent(v1);
}

TEST(AContext, record_simple_tape)
{
	adv::AContext ctx;
	auto v1 = ctx.new_independent();
	auto v2 = v1 * v1;
	ctx.set_dependent(v2);
}
