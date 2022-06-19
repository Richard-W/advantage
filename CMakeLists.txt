cmake_minimum_required(VERSION 3.10)
project(advantage)

option(BUILD_TESTS "Build unit tests" ON)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

# Find dependencies
find_package(Threads REQUIRED)
find_package(Doxygen)

set(cargo_target_dir ${PROJECT_BINARY_DIR}/target)
# FIXME: The second "--features" flag is a noop. cargo does not accept an empty string as an argument.
set(cargo_flags --features ffi $<IF:$<NOT:$<CONFIG:Debug>>,--release,--features>)
set(cargo_library_dir ${cargo_target_dir}/$<IF:$<CONFIG:Debug>,debug,release>)

include(ExternalProject)
ExternalProject_add(advantage_rust
	SOURCE_DIR ${PROJECT_SOURCE_DIR}
	BUILD_IN_SOURCE TRUE
	BUILD_ALWAYS TRUE
	CONFIGURE_COMMAND ""
	BUILD_COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${cargo_target_dir} cargo build ${cargo_flags}
	TEST_COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${cargo_target_dir} cargo test ${cargo_flags}
	INSTALL_COMMAND ""
)
if (BUILD_TESTS)
	ExternalProject_add(googletest
		GIT_REPOSITORY https://github.com/google/googletest
		GIT_TAG release-1.10.0
		CMAKE_ARGS
			-DCMAKE_INSTALL_PREFIX=${PROJECT_BINARY_DIR}/gtest
			-DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
			-Dgtest_force_shared_crt=TRUE
	)
endif ()

add_library(advantage_cxx SHARED
	include/adv.hpp
	include/adv/AContext.hpp
	include/adv/ADouble.hpp
	include/adv/export.hpp
	src/cxx/AContext.cpp
	src/cxx/ADouble.cpp
)
add_dependencies(advantage_cxx advantage_rust)
target_link_directories(advantage_cxx PRIVATE
	${cargo_library_dir}
)
set_target_properties(advantage_cxx PROPERTIES
	OUTPUT_NAME advantage
)
target_link_libraries(advantage_cxx PRIVATE
	advantage
	${CMAKE_DL_LIBS}
	Threads::Threads
)
target_include_directories(advantage_cxx PUBLIC
	${PROJECT_SOURCE_DIR}/include
)
target_compile_definitions(advantage_cxx PRIVATE
	_IN_ADV
)
if (WIN32)
	target_link_libraries(advantage_cxx PRIVATE
		ws2_32
		userenv
		Bcrypt
	)
endif ()

if (BUILD_TESTS)
	enable_testing()
	include(GoogleTest)
	macro(adv_test name)
		add_executable(${name} tests/cxx/${name}.cpp)
		add_dependencies(${name} googletest)
		target_link_directories(${name} PRIVATE
			${PROJECT_BINARY_DIR}/gtest/lib
		)
		target_include_directories(${name} PRIVATE
			${PROJECT_BINARY_DIR}/gtest/include
		)
		target_link_libraries(${name}
			advantage_cxx
			Threads::Threads
			gtest$<$<CONFIG:Debug>:d>
			gtest_main$<$<CONFIG:Debug>:d>
		)
		gtest_discover_tests(${name})
	endmacro()
	adv_test(AContextTests)
	adv_test(ADoubleTests)
endif ()

if (DOXYGEN_FOUND)
	doxygen_add_docs(doc include)
endif ()
