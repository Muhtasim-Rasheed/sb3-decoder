use crate::macros::string_id;

/// The representation of a Scratch broadcast.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Broadcast {
    pub name: String,
}

string_id! {
    /// The ID of a Scratch broadcast.
    BroadcastId
}
