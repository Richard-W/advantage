#ifndef _ADV_CONTEXT_HPP
#define _ADV_CONTEXT_HPP

#include "ADouble.hpp"

namespace adv
{

class AContext
{
public:
	/// \brief Destructor
	~AContext();

	/// \brief Default constructor.
	AContext();
	/// \brief Deleted copy constructor.
	AContext(const AContext&) = delete;
	/// \brief Move constructor.
	AContext(AContext&&);

	/// \brief Deleted copy assignment.
	AContext& operator=(const AContext&) = delete;
	/// \brief Move assignment.
	AContext& operator=(AContext&&);

	/// Get a new independent variable
	ADouble new_independent();
	/// Set a variable dependent
	void set_dependent(const ADouble& var);

private:
	struct Impl;
	Impl* m_impl;

	friend class Tape;
	void* get_impl();
	void* move_impl();
};

} // namespace adv

#endif // _ADV_CONTEXT_HPP
