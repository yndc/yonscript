cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
file(GLOB SOURCES "src/lib/**/*.cpp")

# Yonscript CLI
project(yoncli VERSION 0.1.0)
add_executable(yoncli src/exec/yoncli/main.cpp ${SOURCES})
target_include_directories(yoncli PUBLIC 
    "src/lib"
    "src/lib/vm"
)

