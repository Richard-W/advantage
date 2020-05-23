#ifndef _ADV_DOUBLE_HPP
#define _ADV_DOUBLE_HPP

namespace adv
{

class ADouble
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

	friend ADouble operator+(double, const ADouble&);
	friend ADouble operator-(double, const ADouble&);
	friend ADouble operator*(double, const ADouble&);
	friend ADouble operator/(double, const ADouble&);

	friend ADouble sin(const ADouble&);
	friend ADouble cos(const ADouble&);
	friend ADouble tan(const ADouble&);
	friend ADouble abs(const ADouble&);
	friend ADouble exp(const ADouble&);
	friend ADouble ln(const ADouble&);

	friend ADouble min(const ADouble&, const ADouble&);
	friend ADouble max(const ADouble&, const ADouble&);
};

ADouble operator+(double, const ADouble&);
ADouble operator-(double, const ADouble&);
ADouble operator*(double, const ADouble&);
ADouble operator/(double, const ADouble&);

ADouble sin(const ADouble&);
ADouble cos(const ADouble&);
ADouble tan(const ADouble&);
ADouble abs(const ADouble&);
ADouble exp(const ADouble&);
ADouble ln(const ADouble&);

double sin(double);
double cos(double);
double tan(double);
double abs(double);
double exp(double);
double ln(double);

ADouble min(const ADouble&, const ADouble&);
ADouble max(const ADouble&, const ADouble&);

double min(double, double);
double max(double, double);

} // namespace adv

#endif // _ADV_DOUBLE_HPP

