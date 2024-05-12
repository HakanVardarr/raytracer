cmake_minimum_required(VERSION 3.29)

set(PROJECT_NAME raytracer)

project(${PROJECT_NAME})
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 -Wextra -Wall")

set(INCLUDE_DIRS ${INLUDE_DIRS} ${PROJECT_SOURCE_DIR}/include)
include_directories(${INCLUDE_DIRS})

set(SOURCE_FILES src/vector.c src/color.c)

add_executable(${PROJECT_NAME} main.c ${SOURCE_FILES})