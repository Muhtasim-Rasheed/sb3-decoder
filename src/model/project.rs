use std::collections::BTreeMap;

use crate::prelude::*;

/// The representation of a Scratch project.
#[derive(Debug)]
#[non_exhaustive]
pub struct Project {
    pub sprites: Vec<Sprite>,
    pub stage: Stage,

    pub global_variables: BTreeMap<code::VarId, code::Variable>,
    pub global_lists: BTreeMap<code::ListId, code::List>,

    pub broadcasts: BTreeMap<code::BroadcastId, code::Broadcast>,
}
