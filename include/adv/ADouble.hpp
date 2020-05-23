#ifndef _ADV_DOUBLE_HPP
#define _ADV_DOUBLE_HPP

#include "export.hpp"

namespace adv
{

class ADV_EXPORT ADouble
{
public:
	/// \brief Destructor
	~ADouble();

	/// \brief Default constructor.
	ADouble();
	/// \brief Construct from a primitive
	ADouble(double);
	/// \brief Deleted copy constructor.
	ADouble(const ADouble&);
	/// \brief Move constructor.
	ADouble(ADouble&&);

	/// \brief Deleted copy assignment.
	ADouble& operator=(const ADouble&);
	/// \brief Move assignment.
	ADouble& operator=(ADouble&&);

	// Overloaded arithmetic operators
	ADouble operator+(const ADouble&) const;
	ADouble operator-(const ADouble&) const;
	ADouble operator*(const ADouble&) const;
	ADouble operator/(const ADouble&) const;

private:
	struct Impl;
	Impl* m_impl;

	ADouble(void* impl);
	const void* get_impl() const;

	friend class AContext;

	friend ADV_EXPORT ADouble operator+(double, const ADouble&);
	friend ADV_EXPORT ADouble operator-(double, const ADouble&);
	friend ADV_EXPORT ADouble operator*(double, const ADouble&);
	friend ADV_EXPORT ADouble operator/(double, const ADouble&);

	friend ADV_EXPORT ADouble sin(const ADouble&);
	friend ADV_EXPORT ADouble cos(const ADouble&);
	friend ADV_EXPORT ADouble tan(const ADouble&);
	friend ADV_EXPORT ADouble abs(const ADouble&);
	friend ADV_EXPORT ADouble exp(const ADouble&);
	friend ADV_EXPORT ADouble ln(const ADouble&);

	friend ADV_EXPORT ADouble min(const ADouble&, const ADouble&);
	friend ADV_EXPORT ADouble max(const ADouble&, const ADouble&);
};

ADV_EXPORT ADouble operator+(double, const ADouble&);
ADV_EXPORT ADouble operator-(double, const ADouble&);
ADV_EXPORT ADouble operator*(double, const ADouble&);
ADV_EXPORT ADouble operator/(double, const ADouble&);

ADV_EXPORT ADouble sin(const ADouble&);
ADV_EXPORT ADouble cos(const ADouble&);
ADV_EXPORT ADouble tan(const ADouble&);
ADV_EXPORT ADouble abs(const ADouble&);
ADV_EXPORT ADouble exp(const ADouble&);
ADV_EXPORT ADouble ln(const ADouble&);

ADV_EXPORT double sin(double);
ADV_EXPORT double cos(double);
ADV_EXPORT double tan(double);
ADV_EXPORT double abs(double);
ADV_EXPORT double exp(double);
ADV_EXPORT double ln(double);

ADV_EXPORT ADouble min(const ADouble&, const ADouble&);
ADV_EXPORT ADouble max(const ADouble&, const ADouble&);

ADV_EXPORT double min(double, double);
ADV_EXPORT double max(double, double);

} // namespace adv

#endif // _ADV_DOUBLE_HPP

