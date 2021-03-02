use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum KudosuAction {
    #[serde(rename = "recalculate.reset")]
    RecalculateReset,
    #[serde(rename = "vote.give")]
    VoteGive,
    #[serde(rename = "vote.revoke")]
    VoteRevoke,
    #[serde(rename = "vote.reset")]
    VoteReset,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct KudosuGiver {
    pub url: String,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KudosuHistory {
    pub id: u32,
    /// Either `give`, `reset`, or `revoke`.
    pub action: KudosuAction,
    pub amount: i32,
    /// Object type which the exchange happened on (forum_post, etc).
    pub model: String,
    pub created_at: DateTime<Utc>,
    /// Simple detail of the user who started the exchange.
    pub giver: Option<KudosuGiver>,
    /// Simple detail of the object for display.
    pub post: KudosuPost,
}

impl PartialEq for KudosuHistory {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for KudosuHistory {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct KudosuPost {
    /// Url of the object.
    pub url: Option<String>,
    /// Title of the object. It'll be "[deleted beatmap]" for deleted beatmaps.
    pub title: String,
}
