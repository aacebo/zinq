# Zinq

**ANOTHER** new programming language!

The goal of the Zinq project is to create a new programming language with all the best parts about Rust and Go. These two fantastic languages are opposite ends of the spectrum of syntax/features, so Zinq is intended to be a more moderate middleground.

## Why

Over the past 5 years I have predominately used [Rust](https://rust-lang.org/) and [Go](https://go.dev/) to build projects. I love these new languages for several reasons but have always felt there was a space between the two that needs to be filled.

### Rust

I **LOVE** the expressive type system and control rust gives you, and of coarse the code generation via proc macros.

The only issue I see is that sometimes you want to make services fast, so typically for that use case I turn to **Go**.

### Go

I **LOVE** how simple it is to accomplish complex tasks using Go, but I feel that the syntax could be more expressive and have a few more features that exist in Rust.

## Features

### Expressive Type System

A Rust inspired type system but with some nice Go features like the `any` interface.

### Optional Garbage Collector

An optional garbage collector so the language can be used with or without automated memory management.

### Rich Standard Library

A rich standard library with lots of out-of-the-box production ready features.

> Not intended to be quite as feature rich as Go's, this is because I think it
> can deter open source ecosystem development.

### Macros

Code generation feature via macros inspired by Rust, but with less restrictions to enable cross parsing of types.