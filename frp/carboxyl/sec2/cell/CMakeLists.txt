cmake_minimum_required(VERSION 3.8)
project(cell)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/main.rs
        Cargo.lock
        Cargo.toml)

add_executable(cell ${SOURCE_FILES})