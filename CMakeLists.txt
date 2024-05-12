cmake_minimum_required(VERSION 3.29)

set(PROJECT_NAME raytracer)

project(${PROJECT_NAME})
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 -Wextra -Wall")

add_executable(${PROJECT_NAME} main.c)