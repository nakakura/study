#!/bin/sh
cargo build
cc main.c -L./target/debug/ -ltex_sender -ldl -lm -lpthread
