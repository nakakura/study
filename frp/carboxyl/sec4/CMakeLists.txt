cmake_minimum_required(VERSION 3.8)
project(sec4)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/main.rs
        Cargo.toml)

add_executable(sec4 ${SOURCE_FILES})