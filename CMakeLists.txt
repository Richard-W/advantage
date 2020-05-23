cmake_minimum_required(VERSION 3.10)
project(advantage)

option(BUILD_TESTS "Build unit tests" ON)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

# Find dependencies
find_package(Threads REQUIRED)
find_package(Doxygen)

# Move cargo target directory to build dir
set(CARGO_TARGET_DIR ${PROJECT_BINARY_DIR}/target)

set(cargo_flags --features ffi)
if (CMAKE_BUILD_TYPE MATCHES DEBUG)
	set(cargo_library_path ${CARGO_TARGET_DIR}/debug/libadvantage.a)
else ()
	set(cargo_flags ${cargo_flags} --release)
	set(cargo_library_path ${CARGO_TARGET_DIR}/release/libadvantage.a)
endif ()

include(ExternalProject)
ExternalProject_add(advantage_rust
	SOURCE_DIR ${PROJECT_SOURCE_DIR}
	BUILD_IN_SOURCE TRUE
	BUILD_ALWAYS TRUE
	CONFIGURE_COMMAND ""
	BUILD_COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo build ${cargo_flags}
	TEST_COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo test ${cargo_flags}
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

add_library(advantage SHARED
	src/cxx/AContext.cpp
	src/cxx/ADouble.cpp
)
add_dependencies(advantage advantage_rust)
target_link_libraries(advantage PRIVATE
	${cargo_library_path}
	${CMAKE_DL_LIBS}
	Threads::Threads
)
target_include_directories(advantage PUBLIC
	${PROJECT_SOURCE_DIR}/include
)

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
			advantage
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
