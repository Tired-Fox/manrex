use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    client::ExtendParams,
    uuid::{GroupId, UserId},
};

use super::{Order, Relationship};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanlationGroupAttributes {
    pub name: String,
    pub alt_names: Vec<BTreeMap<String, String>>,
    pub website: Option<String>,
    pub irc_server: Option<String>,
    pub irc_channel: Option<String>,
    pub discord: Option<String>,
    pub contact_email: Option<String>,
    pub description: Option<String>,
    pub twitter: Option<String>,
    pub manga_updates: Option<String>,
    pub focused_language: Option<String>,
    pub locked: bool,
    pub official: bool,
    pub verified: bool,
    pub inactive: bool,
    pub ex_licensed: bool,
    pub publish_delay: Option<String>,
    pub version: usize,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanlationGroup {
    pub id: GroupId,
    pub attributes: ScanlationGroupAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ScanlationGroupInclude {
    Leader,
    Member,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ScanlationGroupFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<GroupId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focused_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<ScanlationGroupInclude>>,
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

    pub fn ids<G: Into<GroupId>>(mut self, s: impl IntoIterator<Item = G>) -> Self {
        self.ids = Some(s.into_iter().map(|v| v.into()).collect());
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

    pub fn order<S: std::fmt::Display>(
        mut self,
        includes: impl IntoIterator<Item = (S, Order)>,
    ) -> Self {
        self.order = Some(includes
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect());
        self
    }

    pub fn include(mut self, includes: impl IntoIterator<Item = ScanlationGroupInclude>) -> Self {
        self.includes = Some(includes.into_iter().collect());
        self
    }
}

impl ExtendParams for ScanlationGroupFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("name", self.name);
        request.add_param_opt("focusedLanguage", self.focused_language);
        request.add_param_opt("ids", self.ids);
        request.add_param_opt("order", self.order);
        request.add_param_opt("includes", self.includes);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateScanlationGroup {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manga_updates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_delay: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateScanlationGroup {
    pub version: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader: Option<UserId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<UserId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub focused_languages: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manga_updates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_delay: Option<String>,
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
    pub fn leader(mut self, s: impl Into<UserId>) -> Self {
        self.leader = Some(s.into());
        self
    }
    pub fn locked(mut self, s: bool) -> Self {
        self.locked = Some(s);
        self
    }

    pub fn members<M: Into<UserId>>(mut self, s: impl IntoIterator<Item = M>) -> Self {
        self.members = s.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn focused_languages<S: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = S>,
    ) -> Self {
        self.focused_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
}
