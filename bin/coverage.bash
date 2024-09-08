#!/usr/bin/env bash

which cargo-llvm-cov || cargo install cargo-llvm-cov

cargo llvm-cov test
