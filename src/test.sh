#!/bin/bash

# Define paths
INPUT_FILE="./tests/sources/JavaBench.java"
CLANG_OUT="/tmp/clang_tokens.log"
SYNTX_OUT="/tmp/syntx_tokens.log"
JAVAC_OUT="/tmp/javac_tokens.log"

# Function to run clang and save output
run_clang() {
	echo "===== Clang Benchmark ====="
	time clang -Xclang -dump-tokens -fsyntax-only -nostdinc -nostdlibinc "$INPUT_FILE" -nostdinc 2>"$CLANG_OUT"
	local count
	count=$(wc -l <"$CLANG_OUT")
	echo "clang tokens: $count"
	echo ""
}

# Function to run syntx and save output
run_syntx() {
	echo "===== Syntx Benchmark ====="
	time ../target/release/syntx --tokens "$INPUT_FILE" 2>"$SYNTX_OUT"
	local count
	count=$(wc -l <"$SYNTX_OUT")
	echo "syntx tokens: $count"
	echo ""
}

# Function to run javac and save output
run_javac() {
	cd tests/sources || exit
	echo "===== Javac Benchmark ====="
	time java -cp . \
		--add-exports jdk.compiler/com.sun.tools.javac.parser=ALL-UNNAMED \
		--add-exports jdk.compiler/com.sun.tools.javac.util=ALL-UNNAMED \
		--add-exports jdk.compiler/com.sun.tools.javac.file=ALL-UNNAMED \
		TokenDumper JavaBench.java >"$JAVAC_OUT"
	local count
	count=$(grep -cE '.' "$JAVAC_OUT")
	echo "javac lines: $count"
	echo ""
}

# Run benchmarks
run_syntx
run_javac
