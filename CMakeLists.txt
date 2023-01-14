cmake_minimum_required(VERSION 3.10)

include(FetchContent)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_DEBUG_POSTFIX .debug)
file(GLOB SOURCES "src/lib/**/*.cpp")

# Utility functions
function (FetchContent_GitHub dep repo tag) 
  FetchContent_Declare(
    ${dep}
    https://github.com/${repo}/archive/${tag}.zip
  )
endfunction()

# Dependencies
FetchContent_GitHub(fmt fmtlib/fmt 9.1.0)

list(APPEND EXTERNAL_LIBS fmt)

# Yonscript CLI
project(yoncli VERSION 0.1.0)
add_executable(yoncli src/exec/yoncli/main.cpp ${SOURCES})
set_target_properties(yoncli PROPERTIES DEBUG_POSTFIX ${CMAKE_DEBUG_POSTFIX})
target_link_libraries(yoncli PUBLIC ${EXTERNAL_LIBS})
target_include_directories(yoncli PUBLIC 
    src
    ${PROJECT_BINARY_DIR}
)

