pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which of an event trigger's events count; each event type takes its own.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionFiltersInput {
    /// Only events whose actor is one of these kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_kinds: Option<Vec<SubscriptionFiltersInputActorKindsItem>>,
    /// GitHub issue events only: just issues assigned to this login (on issue.assigned, the person just assigned).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /// ServiceNow events only: just records assigned to this group, by its name as shown, case-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_group: Option<String>,
    /// Slack messages only (not reactions, saves or pins): just messages from this Slack user id (U0123ABCD). GitHub pull request and issue events: just those this login opened. ServiceNow: just changes this user name (jdoe) made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// ServiceNow update events only: just updates that changed any of these fields, e.g. ['state', 'priority'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed: Option<Vec<String>>,
    /// GitHub check events only: just checks that finished with one of these conclusions, e.g. ['failure', 'timed_out'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<Vec<SubscriptionFiltersInputConclusionItem>>,
    /// Ignore changes agents make (true by default for content changes). Your own changes never wake you either way.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_agents: Option<bool>,
    /// GitHub pull request and issue events only: ignore what bots send (review bots, GitHub Apps, Dependabot). Checks are always a CI app's, so they never take it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_bots: Option<bool>,
    /// Ignore what you do yourself (true by default), including the GitHub comments this conversation posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_self: Option<bool>,
    /// Mail events only: just mail whose sender contains this text (case-insensitive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_contains: Option<String>,
    /// Mail events only: just mail with (true) or without attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    /// GitHub issue events only: just issues carrying this label (on issue.labeled, the label just added).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Comment events only: just the comments that mention you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions_me: Option<bool>,
    /// Only with asset.presence.joined (subscribed on its own): hear only these people opening the asset, as user ids (user_...).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_refs: Option<Vec<String>>,
    /// ServiceNow events only: just records at this priority, as shown ('1 - Critical') or its number alone ('1').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// ServiceNow events only: just records left in this state, as ServiceNow shows it (e.g. 'In Progress'), case-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Mail events only: just mail whose subject contains this text (case-insensitive). The subject itself is never stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_contains: Option<String>,
    /// A database's row events (database.row_*) only, and required for them: the one table to hear. Each event names the row's key, never the row: read it with the database tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<SubscriptionTableInput>,
    /// Comment events only: just this comment thread, as list_comment_threads names it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

impl SubscriptionFiltersInput {
    pub fn builder() -> SubscriptionFiltersInputBuilder {
        <SubscriptionFiltersInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionFiltersInputBuilder {
    actor_kinds: Option<Vec<SubscriptionFiltersInputActorKindsItem>>,
    assignee: Option<String>,
    assignment_group: Option<String>,
    author: Option<String>,
    changed: Option<Vec<String>>,
    conclusion: Option<Vec<SubscriptionFiltersInputConclusionItem>>,
    exclude_agents: Option<bool>,
    exclude_bots: Option<bool>,
    exclude_self: Option<bool>,
    from_contains: Option<String>,
    has_attachments: Option<bool>,
    label: Option<String>,
    mentions_me: Option<bool>,
    principal_refs: Option<Vec<String>>,
    priority: Option<String>,
    state: Option<String>,
    subject_contains: Option<String>,
    table: Option<SubscriptionTableInput>,
    thread_id: Option<String>,
}

impl SubscriptionFiltersInputBuilder {
    pub fn actor_kinds(mut self, value: Vec<SubscriptionFiltersInputActorKindsItem>) -> Self {
        self.actor_kinds = Some(value);
        self
    }

    pub fn assignee(mut self, value: impl Into<String>) -> Self {
        self.assignee = Some(value.into());
        self
    }

    pub fn assignment_group(mut self, value: impl Into<String>) -> Self {
        self.assignment_group = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn changed(mut self, value: Vec<String>) -> Self {
        self.changed = Some(value);
        self
    }

    pub fn conclusion(mut self, value: Vec<SubscriptionFiltersInputConclusionItem>) -> Self {
        self.conclusion = Some(value);
        self
    }

    pub fn exclude_agents(mut self, value: bool) -> Self {
        self.exclude_agents = Some(value);
        self
    }

    pub fn exclude_bots(mut self, value: bool) -> Self {
        self.exclude_bots = Some(value);
        self
    }

    pub fn exclude_self(mut self, value: bool) -> Self {
        self.exclude_self = Some(value);
        self
    }

    pub fn from_contains(mut self, value: impl Into<String>) -> Self {
        self.from_contains = Some(value.into());
        self
    }

    pub fn has_attachments(mut self, value: bool) -> Self {
        self.has_attachments = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn mentions_me(mut self, value: bool) -> Self {
        self.mentions_me = Some(value);
        self
    }

    pub fn principal_refs(mut self, value: Vec<String>) -> Self {
        self.principal_refs = Some(value);
        self
    }

    pub fn priority(mut self, value: impl Into<String>) -> Self {
        self.priority = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn subject_contains(mut self, value: impl Into<String>) -> Self {
        self.subject_contains = Some(value.into());
        self
    }

    pub fn table(mut self, value: SubscriptionTableInput) -> Self {
        self.table = Some(value);
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionFiltersInput`].
    pub fn build(self) -> Result<SubscriptionFiltersInput, BuildError> {
        Ok(SubscriptionFiltersInput {
            actor_kinds: self.actor_kinds,
            assignee: self.assignee,
            assignment_group: self.assignment_group,
            author: self.author,
            changed: self.changed,
            conclusion: self.conclusion,
            exclude_agents: self.exclude_agents,
            exclude_bots: self.exclude_bots,
            exclude_self: self.exclude_self,
            from_contains: self.from_contains,
            has_attachments: self.has_attachments,
            label: self.label,
            mentions_me: self.mentions_me,
            principal_refs: self.principal_refs,
            priority: self.priority,
            state: self.state,
            subject_contains: self.subject_contains,
            table: self.table,
            thread_id: self.thread_id,
        })
    }
}
