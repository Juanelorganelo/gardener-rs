# Gardener
A configurable, extensible and blazing fast multi-VCS branch name linter, with presets for commonly used branching conventions, written in Rust.

## Why?
Mainly cause I love writing Rust code, but I think this could be useful for DevOps/GitOps.

## Validators
Validators are just Rust libraries that that take the AST from a `Parser` and do stuff with it. As such, validators are generic over their parser type.
