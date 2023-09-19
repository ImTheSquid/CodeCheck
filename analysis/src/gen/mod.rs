// The generated code is too annoying to deal with every linting error individually
// This will probably bite me in the ass someday but that's not my today me's problem!
#![allow(warnings)]

// Public modules, implemented for each language tree
pub mod cpp14parservisitor;
pub mod cvisitor;
pub mod java20parservisitor;
pub mod python3parservisitor;

// Private modules, only for support
pub mod clexer;
mod clistener;
pub mod cparser;

pub mod cpp14lexer;
pub mod cpp14parser;
mod cpp14parserlistener;

mod java20lexer;
mod java20parser;
mod java20parserlistener;

mod python3lexer;
mod python3parser;
mod python3parserlistener;
