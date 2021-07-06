use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SendMessageEvent {}