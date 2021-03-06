#include <adv/AContext.hpp>
#include "ffi.hpp"

#include <stdexcept>

namespace adv
{

struct AContext::Impl
{
public:
	adv_acontext* ctx;
};

AContext::~AContext()
{
	if (m_impl != nullptr) {
		if (m_impl->ctx != nullptr) {
			::adv_acontext_free(m_impl->ctx);
		}
		delete m_impl;
	}
}

AContext::AContext():
	m_impl(new Impl)
{
	m_impl->ctx = ::adv_acontext_new();
}

AContext::AContext(AContext&& other):
	m_impl(other.m_impl)
{
	other.m_impl = nullptr;
}

AContext& AContext::operator=(AContext&& other)
{
	if (m_impl != nullptr) {
		::adv_acontext_free(m_impl->ctx);
		delete m_impl;
	}
	m_impl = other.m_impl;
	other.m_impl = nullptr;
	return *this;
}

void* AContext::get_impl()
{
	return m_impl->ctx;
}

void* AContext::move_impl() {
	auto ctx = m_impl->ctx;
	delete m_impl;
	m_impl = nullptr;
	return ctx;
}

ADouble AContext::new_independent()
{
	return ADouble(::adv_acontext_new_independent(m_impl->ctx));
}

void AContext::set_dependent(const ADouble& var)
{
	::adv_acontext_set_dependent(m_impl->ctx, reinterpret_cast<const ::adv_adouble*>(var.get_impl()));
}

} // namespace adv
