use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::client::ExtendParams;

use super::{Order, Relationship};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanlationGroupAttributes {
    name: String,
    alt_names: Vec<BTreeMap<String, String>>,
    website: Option<String>,
    irc_server: Option<String>,
    irc_channel: Option<String>,
    discord: Option<String>,
    contact_email: Option<String>,
    description: Option<String>,
    twitter: Option<String>,
    manga_updates: Option<String>,
    focused_language: Option<String>,
    locked: bool,
    official: bool,
    verified: bool,
    inactive: bool,
    ex_licensed: bool,
    publish_delay: Option<String>,
    version: usize,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanlationGroup {
    pub id: String,
    pub attributes: ScanlationGroupAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all="snake_case")]
pub enum ScanlationGroupInclude {
    Leader,
    Member,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScanlationGroupFilter {
    limit: Option<usize>,
    offset: Option<usize>,

    ids: Vec<String>,
    name: Option<String>,
    focused_language: Option<String>,

    order: BTreeMap<String, Order>,
    includes: Vec<ScanlationGroupInclude>,
}

impl ScanlationGroupFilter {
    pub fn limit(mut self, state: usize) -> Self {
        self.limit = Some(state);
        self
    }

    pub fn offset(mut self, state: usize) -> Self {
        self.offset = Some(state);
        self
    }

    pub fn ids<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.ids = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn name<S: std::fmt::Display>(mut self, state: S) -> Self {
        self.name = Some(state.to_string());
        self
    }

    pub fn focused_language<S: std::fmt::Display>(mut self, state: S) -> Self {
        self.focused_language = Some(state.to_string());
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, includes: impl IntoIterator<Item=(S, Order)>) -> Self {
        self.order = includes.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self
    }

    pub fn include(mut self, includes: impl IntoIterator<Item=ScanlationGroupInclude>) -> Self {
        self.includes.extend(includes);
        self
    }
}

impl ExtendParams for ScanlationGroupFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);

        request.add_param_opt("name", self.name);
        request.add_param_opt("focusedLanguage", self.focused_language);

        if !self.ids.is_empty() {
            request.add_param("ids", self.ids);
        }

        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }

        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateScanlationGroup {
    name: String,

    #[serde(skip_serializing_if="Option::is_none")]
    website: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    irc_server: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    irc_channel: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    discord: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    contact_email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    twitter: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    manga_updates: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    inactive: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    publish_delay: Option<String>,
}

impl CreateScanlationGroup {
    pub fn new(name: impl std::fmt::Display) -> Self {
        Self {
            name: name.to_string(),

            website: None,
            irc_server: None,
            irc_channel: None,
            discord: None,
            contact_email: None,
            description: None,
            twitter: None,
            manga_updates: None,
            publish_delay: None,
            inactive: None,
        }
    }

    pub fn website(mut self, s: impl std::fmt::Display) -> Self {
        self.website = Some(s.to_string());
        self
    }
    pub fn irc_server(mut self, s: impl std::fmt::Display) -> Self {
        self.irc_server = Some(s.to_string());
        self
    }
    pub fn irc_channel(mut self, s: impl std::fmt::Display) -> Self {
        self.irc_channel = Some(s.to_string());
        self
    }
    pub fn discord(mut self, s: impl std::fmt::Display) -> Self {
        self.discord = Some(s.to_string());
        self
    }
    pub fn contact_email(mut self, s: impl std::fmt::Display) -> Self {
        self.contact_email = Some(s.to_string());
        self
    }
    pub fn description(mut self, s: impl std::fmt::Display) -> Self {
        self.description = Some(s.to_string());
        self
    }
    pub fn twitter(mut self, s: impl std::fmt::Display) -> Self {
        self.twitter = Some(s.to_string());
        self
    }
    pub fn manga_updates(mut self, s: impl std::fmt::Display) -> Self {
        self.manga_updates = Some(s.to_string());
        self
    }
    pub fn publish_delay(mut self, s: impl std::fmt::Display) -> Self {
        self.publish_delay = Some(s.to_string());
        self
    }
    pub fn inactive(mut self, s: bool) -> Self {
        self.inactive = Some(s);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdateScanlationGroup {
    version: usize,

    #[serde(skip_serializing_if="Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    leader: Option<String>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    members: Vec<String>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    focused_languages: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    locked: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    website: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    irc_server: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    irc_channel: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    discord: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    contact_email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    twitter: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    manga_updates: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    inactive: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    publish_delay: Option<String>,

}

impl UpdateScanlationGroup {
    pub fn new(version: usize) -> Self {
        Self {
            version,

            name: None,
            leader: None,
            members: Vec::new(),
            focused_languages: Vec::new(),
            locked: None,

            website: None,
            irc_server: None,
            irc_channel: None,
            discord: None,
            contact_email: None,
            description: None,
            twitter: None,
            manga_updates: None,
            publish_delay: None,
            inactive: None,
        }
    }

    pub fn website(mut self, s: impl std::fmt::Display) -> Self {
        self.website = Some(s.to_string());
        self
    }
    pub fn irc_server(mut self, s: impl std::fmt::Display) -> Self {
        self.irc_server = Some(s.to_string());
        self
    }
    pub fn irc_channel(mut self, s: impl std::fmt::Display) -> Self {
        self.irc_channel = Some(s.to_string());
        self
    }
    pub fn discord(mut self, s: impl std::fmt::Display) -> Self {
        self.discord = Some(s.to_string());
        self
    }
    pub fn contact_email(mut self, s: impl std::fmt::Display) -> Self {
        self.contact_email = Some(s.to_string());
        self
    }
    pub fn description(mut self, s: impl std::fmt::Display) -> Self {
        self.description = Some(s.to_string());
        self
    }
    pub fn twitter(mut self, s: impl std::fmt::Display) -> Self {
        self.twitter = Some(s.to_string());
        self
    }
    pub fn manga_updates(mut self, s: impl std::fmt::Display) -> Self {
        self.manga_updates = Some(s.to_string());
        self
    }
    pub fn publish_delay(mut self, s: impl std::fmt::Display) -> Self {
        self.publish_delay = Some(s.to_string());
        self
    }
    pub fn inactive(mut self, s: bool) -> Self {
        self.inactive = Some(s);
        self
    }

    pub fn name(mut self, s: impl std::fmt::Display) -> Self {
        self.name = Some(s.to_string());
        self
    }
    pub fn leader(mut self, s: impl std::fmt::Display) -> Self {
        self.leader = Some(s.to_string());
        self
    }
    pub fn locked(mut self, s: bool) -> Self {
        self.locked = Some(s);
        self
    }

    pub fn members<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.members = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn focused_languages<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.focused_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
}
