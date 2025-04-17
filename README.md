# syntx
Modular syntax engine in Rust for lexing, parsing, and analyzing programming languages. (WIP)

This program is designed to tokenize and analyze structured code files using a predictive and stateful lexer/parser engine.  
It supports user-defined grammars and built-in token inference for languages like Java and C.

## Current state

It's in the bootstrapping phase, im trying to use .stx files to generate optimized rust code and recompile syntx itself on the fly

## About

Some of the most important modules that power `syntx` are:
+ Custom trait-based **tokenizer** for streaming byte-level tokenization
+ Stateful **semantic engine** for inferring expression trees and variable types
+ Fully pluggable **language modules** with isolated delimiter/token sets
+ Abstract **hash tree** for syntax-based caching and analysis
+ Support for `.stx` grammar files to describe custom syntax

## Installation and Execution

Clone the repository using Git:
```
git clone https://github.com/lvxrr/syntx cd syntx
```
Then compile using `cargo`:
```
cargo build --release
```
To get help, just run syntx --help

## Supported Languages

Currently, `syntx` includes language modules for:
+ `Java` (inference, imports, token set, delimiters)
+ stx (bootstrap)

## Goals

+  Language-agnostic syntax tree generator  
+  Custom grammar specs with `.stx` format  
+  Fast, predictable byte-level lexing  
+  Type-aware token prediction
+  Embeddable architecture for IDEs, compilers, ML...  

## Current benchmarks
```
===== Syntx Benchmark =====

real    0m0,001s
user    0m0,000s
sys     0m0,001s
syntx tokens: 341

===== Javac Benchmark =====

real    0m0,067s
user    0m0,092s
sys     0m0,013s
javac lines: 350
```
```
========== Benchmark Results for tests/sources/big_java.java ==========
Lines        : 10576940
Tokens       : 46301950
Time         : 2.3439 s
Memory       : 0.00 MB
Lines/sec    : 4512613
Tokens/sec   : 19754562
```
