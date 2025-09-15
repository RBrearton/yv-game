cmake_minimum_required(VERSION 3.20)

# Project configuration.
project(yv-rlib
    VERSION 0.1.0
    DESCRIPTION "yv game built with C and raylib"
    LANGUAGES C
)

# Modern C standard.
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

# Build type configuration.
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
endif()

# Compiler-specific options.
if(MSVC)
    add_compile_options(/W4)
    if(CMAKE_BUILD_TYPE STREQUAL "Debug")
        add_compile_options(/Od /MDd)
    else()
        add_compile_options(/O2 /MD)
    endif()
else()
    add_compile_options(-Wall -Wextra -Wpedantic -std=c17)
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

# Collect all source files automatically.
file(GLOB_RECURSE SOURCES "src/*.c")

# Main executable.
add_executable(${PROJECT_NAME} ${SOURCES})

# Explicitly set source files as C language.
set_source_files_properties(${SOURCES} PROPERTIES LANGUAGE C)

# Configure raylib linking based on detection method.
if(RAYLIB_INCLUDE_DIRS AND RAYLIB_LIBRARIES)
    # Using pkg-config or manual detection.
    target_include_directories(${PROJECT_NAME} PRIVATE ${RAYLIB_INCLUDE_DIRS})
    
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
    target_link_libraries(${PROJECT_NAME} raylib)
endif()

# Copy assets to build directory.
add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    ${CMAKE_SOURCE_DIR}/assets
    ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/assets
)

# Optional: Testing configuration (disabled by default).
# Uncomment and add tests in tests/ directory if needed.
# option(BUILD_TESTS "Build tests" OFF)

# Installation configuration.
install(TARGETS ${PROJECT_NAME}
    RUNTIME DESTINATION bin
)

install(DIRECTORY assets/
    DESTINATION share/${PROJECT_NAME}/assets
)
