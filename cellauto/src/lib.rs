//! An abstract cellular automata engine
#![deny(unsafe_code, missing_docs)]

#[derive(Debug)]
pub struct ConwaysLife {
    bounds: Bounds,
}
