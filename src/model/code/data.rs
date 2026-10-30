use crate::{macros::string_id, prelude::*};

/// The representation of a Scratch variable.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: code::Value,
    pub cloud: bool,
}

/// THe representation of a Scratch list.
#[derive(Debug, Clone)]
pub struct List {
    pub name: String,
    pub values: Vec<code::Value>,
}

string_id! {
    /// The ID of a Scratch variable.
    VarId
}

string_id! {
    /// The ID of a Scratch list.
    ListId
}
