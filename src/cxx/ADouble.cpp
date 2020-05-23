#include <adv/ADouble.hpp>
#include "ffi.hpp"

#include <cmath>
#include <stdexcept>

namespace adv
{

struct ADouble::Impl
{
public:
	adv_double* val;
};

ADouble::~ADouble()
{
	::adv_double_free(m_impl->val);
	delete m_impl;
}

ADouble::ADouble(void* impl):
	m_impl(new Impl)
{
	m_impl->val = reinterpret_cast<adv_double*>(impl);
}

ADouble::ADouble():
	ADouble(::adv_double_default())
{
}

ADouble::ADouble(double val):
	ADouble(::adv_double_from_value(val))
{
}

ADouble::ADouble(const ADouble& other):
	ADouble(::adv_double_copy(other.m_impl->val))
{
}

ADouble::ADouble(ADouble&& other):
	ADouble(other.m_impl->val)
{
	other.m_impl->val = ::adv_double_default();
}

ADouble& ADouble::operator=(ADouble&& other)
{
	::adv_double_free(m_impl->val);
	m_impl->val = other.m_impl->val;
	other.m_impl->val = ::adv_double_default();
	return *this;
}

ADouble& ADouble::operator=(const ADouble& other)
{
	::adv_double_free(m_impl->val);
	m_impl->val = ::adv_double_copy(other.m_impl->val);
	return *this;
}

const void* ADouble::get_impl() const
{
	return m_impl->val;
}

#define BINARY_OP_IMPL(NAME, OP) \
	ADouble ADouble::operator OP(const ADouble& rhs) const \
	{ \
		::adv_double* result_impl; \
		::adv_op_##NAME(m_impl->val, rhs.m_impl->val, &result_impl); \
		return ADouble(result_impl); \
	} \
	\
	ADouble operator OP(double lhs_, const ADouble& rhs) \
	{ \
		auto lhs = ADouble(lhs_); \
		return lhs + rhs; \
	}
BINARY_OP_IMPL(add, +)
BINARY_OP_IMPL(sub, -)
BINARY_OP_IMPL(mul, *)
BINARY_OP_IMPL(div, /)
#undef BINARY_OP_IMPL

#define UNARY_FUNC_IMPL(NAME, STDNAME) \
	ADouble NAME(const ADouble& val) { \
		::adv_double* ptr; \
		::adv_##NAME(val.m_impl->val, &ptr); \
		return ADouble(ptr); \
	} \
	\
	double NAME(double val) { \
		return std::STDNAME(val); \
	}

UNARY_FUNC_IMPL(sin, sin)
UNARY_FUNC_IMPL(cos, cos)
UNARY_FUNC_IMPL(tan, tan)
UNARY_FUNC_IMPL(abs, abs)
UNARY_FUNC_IMPL(exp, exp)
UNARY_FUNC_IMPL(ln, log)
#undef UNARY_FUNC_IMPL

#define BINARY_FUNC_IMPL(NAME) \
	ADouble NAME(const ADouble& lhs, const ADouble& rhs) { \
		::adv_double* ptr; \
		::adv_##NAME(lhs.m_impl->val, rhs.m_impl->val, &ptr); \
		return ADouble(ptr); \
	}
BINARY_FUNC_IMPL(min)
BINARY_FUNC_IMPL(max)
#undef BINARY_FUNC_IMPL

double min(double lhs, double rhs) {
	if (lhs < rhs) {
		return lhs;
	}
	else {
		return rhs;
	}
}

double max(double lhs, double rhs) {
	if (lhs < rhs) {
		return rhs;
	}
	else {
		return lhs;
	}
}

} // namespace adv

