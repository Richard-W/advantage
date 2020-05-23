#ifndef _ADV_EXPORT_HPP
#define _ADV_EXPORT_HPP

#if defined(_MSC_VER)
#	if defined(_IN_ADV)
#		define ADV_EXPORT __declspec(dllexport)
#	else
#		define ADV_EXPORT __declspec(dllimport)
#	endif
#else
#	define ADV_EXPORT
#endif

#endif // _ADV_EXPORT_HPP
