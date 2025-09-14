cmake_minimum_required(VERSION 3.20)

# Project configuration.
project(yvium-rlib
    VERSION 0.1.0
    DESCRIPTION "Yvium game built with C++ and raylib"
    LANGUAGES CXX
)

# Modern C++ standard.
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Build type configuration.
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
endif()

# Compiler-specific options.
if(MSVC)
    add_compile_options(/W4 /permissive-)
    if(CMAKE_BUILD_TYPE STREQUAL "Debug")
        add_compile_options(/Od /MDd)
    else()
        add_compile_options(/O2 /MD)
    endif()
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
    if(CMAKE_BUILD_TYPE STREQUAL "Debug")
        add_compile_options(-g -O0)
    else()
        add_compile_options(-O3 -DNDEBUG)
    endif()
endif()

# Output directories.
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Find raylib - try multiple methods.
set(RAYLIB_FOUND FALSE)

# Method 1: Try pkg-config (common on Linux/macOS with package managers).
find_package(PkgConfig QUIET)
if(PKG_CONFIG_FOUND)
    pkg_check_modules(RAYLIB raylib)
    if(RAYLIB_FOUND)
        message(STATUS "Found raylib via pkg-config: ${RAYLIB_VERSION}")
        # Set consistent variable names for later use.
        set(RAYLIB_INCLUDE_DIRS ${RAYLIB_INCLUDE_DIRS})
        set(RAYLIB_LIBRARIES ${RAYLIB_LIBRARIES})
        set(RAYLIB_LIBRARY_DIRS ${RAYLIB_LIBRARY_DIRS})
    endif()
endif()

# Method 2: Try CMake's find_package (if raylib provides CMake config).
if(NOT RAYLIB_FOUND)
    find_package(raylib QUIET)
    if(raylib_FOUND)
        set(RAYLIB_FOUND TRUE)
        message(STATUS "Found raylib via CMake find_package")
    endif()
endif()

# Method 3: Manual search in common Homebrew/system locations.
if(NOT RAYLIB_FOUND)
    find_path(RAYLIB_INCLUDE_DIR raylib.h
        PATHS
            /opt/homebrew/include
            /usr/local/include
            /usr/include
        PATH_SUFFIXES raylib
    )
    
    find_library(RAYLIB_LIBRARY raylib
        PATHS
            /opt/homebrew/lib
            /usr/local/lib
            /usr/lib
    )
    
    if(RAYLIB_INCLUDE_DIR AND RAYLIB_LIBRARY)
        set(RAYLIB_FOUND TRUE)
        set(RAYLIB_INCLUDE_DIRS ${RAYLIB_INCLUDE_DIR})
        set(RAYLIB_LIBRARIES ${RAYLIB_LIBRARY})
        message(STATUS "Found raylib manually: ${RAYLIB_LIBRARY}")
    endif()
endif()

# Method 4: Fetch from source if not found anywhere.
if(NOT RAYLIB_FOUND)
    message(STATUS "raylib not found locally, fetching from source...")
    include(FetchContent)
    FetchContent_Declare(
        raylib
        GIT_REPOSITORY https://github.com/raysan5/raylib.git
        GIT_TAG 5.5
    )
    FetchContent_MakeAvailable(raylib)
    set(RAYLIB_FOUND TRUE)
endif()

# Include directories.
include_directories(include)

# Collect source files.
file(GLOB_RECURSE SOURCES "src/game/*.cpp")
file(GLOB_RECURSE HEADERS "include/yvium/*.h" "include/yvium/*.hpp")

# Main executable.
add_executable(${PROJECT_NAME} 
    src/main.cpp
    ${SOURCES}
)

# Configure raylib linking based on detection method.
if(RAYLIB_INCLUDE_DIRS AND RAYLIB_LIBRARIES)
    # Using pkg-config or manual detection.
    target_include_directories(${PROJECT_NAME} PRIVATE 
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${RAYLIB_INCLUDE_DIRS}
    )
    
    # Add library directories if specified.
    if(RAYLIB_LIBRARY_DIRS)
        target_link_directories(${PROJECT_NAME} PRIVATE ${RAYLIB_LIBRARY_DIRS})
    endif()
    
    target_link_libraries(${PROJECT_NAME} ${RAYLIB_LIBRARIES})
    
    # Add any additional flags from pkg-config.
    if(RAYLIB_CFLAGS_OTHER)
        target_compile_options(${PROJECT_NAME} PRIVATE ${RAYLIB_CFLAGS_OTHER})
    endif()
    if(RAYLIB_LDFLAGS_OTHER)
        target_link_options(${PROJECT_NAME} PRIVATE ${RAYLIB_LDFLAGS_OTHER})
    endif()
else()
    # Using CMake target or FetchContent.
    target_include_directories(${PROJECT_NAME} PRIVATE 
        ${CMAKE_CURRENT_SOURCE_DIR}/include
    )
    target_link_libraries(${PROJECT_NAME} raylib)
endif()

# Copy assets to build directory.
add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    ${CMAKE_SOURCE_DIR}/assets
    ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/assets
)

# Testing configuration.
option(BUILD_TESTS "Build tests" OFF)
if(BUILD_TESTS)
    enable_testing()
    
    # Find or fetch Catch2.
    find_package(Catch2 QUIET)
    if(NOT Catch2_FOUND)
        FetchContent_Declare(
            Catch2
            GIT_REPOSITORY https://github.com/catchorg/Catch2.git
            GIT_TAG v3.4.0
        )
        FetchContent_MakeAvailable(Catch2)
    endif()
    
    # Test executable.
    file(GLOB_RECURSE TEST_SOURCES "tests/*.cpp")
    add_executable(${PROJECT_NAME}-tests ${TEST_SOURCES} ${SOURCES})
    
    # Configure test linking the same way as main executable.
    if(RAYLIB_INCLUDE_DIRS AND RAYLIB_LIBRARIES)
        target_include_directories(${PROJECT_NAME}-tests PRIVATE 
            ${CMAKE_CURRENT_SOURCE_DIR}/include
            ${RAYLIB_INCLUDE_DIRS}
        )
        
        # Add library directories if specified.
        if(RAYLIB_LIBRARY_DIRS)
            target_link_directories(${PROJECT_NAME}-tests PRIVATE ${RAYLIB_LIBRARY_DIRS})
        endif()
        
        target_link_libraries(${PROJECT_NAME}-tests Catch2::Catch2WithMain ${RAYLIB_LIBRARIES})
        
        if(RAYLIB_CFLAGS_OTHER)
            target_compile_options(${PROJECT_NAME}-tests PRIVATE ${RAYLIB_CFLAGS_OTHER})
        endif()
        if(RAYLIB_LDFLAGS_OTHER)
            target_link_options(${PROJECT_NAME}-tests PRIVATE ${RAYLIB_LDFLAGS_OTHER})
        endif()
    else()
        target_include_directories(${PROJECT_NAME}-tests PRIVATE 
            ${CMAKE_CURRENT_SOURCE_DIR}/include
        )
        target_link_libraries(${PROJECT_NAME}-tests Catch2::Catch2WithMain raylib)
    endif()
    
    include(CTest)
    include(Catch)
    catch_discover_tests(${PROJECT_NAME}-tests)
endif()

# Installation configuration.
install(TARGETS ${PROJECT_NAME}
    RUNTIME DESTINATION bin
)

install(DIRECTORY assets/
    DESTINATION share/${PROJECT_NAME}/assets
)
