use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Ballot, Poll};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>, // If admin is None, default to the sender's address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll { 
        poll_id: String,
        question: String,
        options: Vec<String>
    },
    Vote{
        poll_id: String,
        vote: String,
    }
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
    Poll { poll_id: String },
    Vote { poll_id: String, address: String },
    
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllPollsResponse {
    pub polls: Vec<Poll>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PollResponse {
    pub poll: Option<Poll>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VoteResponse {
    pub vote: Option<Ballot>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
