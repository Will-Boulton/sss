# Super Simple Structs

A language and compiler for defining language agnostic message protocols using packed-structs.

The compiler takes protocol specifications as `.sss` files and emits source code for de/encoding protocol in one of a few supported programming languages.
The compiler is written in Rust and has a plugin model to allow implementing source generation logic for other programming languages.