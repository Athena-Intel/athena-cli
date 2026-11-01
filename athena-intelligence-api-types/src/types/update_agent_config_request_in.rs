pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAgentConfigRequestIn {
    /// UI accent color (hex)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    /// Auto-compaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_compaction_enabled: Option<bool>,
    /// Toolkit IDs available for the agent to select
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_toolkits: Option<Vec<String>>,
    /// Tool IDs the agent may not use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklisted_tools: Option<Vec<String>>,
    /// Whether the agent can be used as a sub-agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_subagent: Option<bool>,
    /// Allow @-mentioning the agent in an asset's comments pane
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_pane_enabled: Option<bool>,
    /// Inject contextual information into replies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_injection: Option<bool>,
    /// What the agent does and when to use it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Voice-friendly handle (lowercase words and spaces) used to address the agent as a sub-agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Icon identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Knowledge base references available to the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<Vec<String>>,
    /// Tags used to filter knowledge base context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_tags: Option<Vec<String>>,
    /// Maximum number of past messages passed to sub-agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_past_messages_for_subagents: Option<i64>,
    /// Allow the agent to attend meetings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_attendance_enabled: Option<bool>,
    /// The agent's meeting voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_voice_enabled: Option<bool>,
    /// Model ID the agent runs on; must be available in its workspace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Muses long-term memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muses_enabled: Option<bool>,
    /// Inject relevant Muses memory before each reply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muses_recall_enabled: Option<bool>,
    /// Display name of the agent; also becomes the asset title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Parallel tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Tool IDs that stay enabled whatever else is configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_enabled_tools: Option<Vec<String>>,
    /// Plan Mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_mode_enabled: Option<bool>,
    /// True stops giving the agent's sessions a computer (existing computers are kept). Not together with session_computer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_session_computer: Option<bool>,
    /// Give the agent's sessions a computer the platform creates from an environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_computer: Option<PublicSessionComputerIn>,
    /// Show the agent only to workspace admins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_only_for_admin: Option<bool>,
    /// How the agent answers a text from a workspace member it isn't shared with: 'default' (the standard no-access message), 'silent' (no reply) or 'custom' (sms_blocked_sender_reply verbatim). The text is always saved in the agent's SMS thread and the agent never runs; senders without an Athena account in the workspace never get a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_blocked_sender_mode: Option<String>,
    /// Reply text sent to blocked SMS senders when sms_blocked_sender_mode is 'custom', e.g. 'Got it, thanks.' (max 320 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_blocked_sender_reply: Option<String>,
    /// SMS access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_enabled: Option<bool>,
    /// Knowledge source references: native source IDs, or collab_source:<asset_id>; replaces the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    /// Whether the agent must follow strict configuration constraints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_agent_configuration: Option<bool>,
    /// Collab agent IDs available as sub-agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    /// Suggested prompts or actions shown for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// The instructions the agent follows (markdown); replaces the whole prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Sampling temperature override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Toolkit IDs the agent can use; replaces the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkits: Option<Vec<String>>,
    /// Tool IDs the agent can use; replaces the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    /// Voice access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enabled: Option<bool>,
    /// Collab agent IDs available as voice sub-agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_sub_agents: Option<Vec<String>>,
    /// Email addresses allowed to use the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelisted_emails: Option<Vec<String>>,
}

impl UpdateAgentConfigRequestIn {
    pub fn builder() -> UpdateAgentConfigRequestInBuilder {
        <UpdateAgentConfigRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentConfigRequestInBuilder {
    accent_color: Option<String>,
    auto_compaction_enabled: Option<bool>,
    available_toolkits: Option<Vec<String>>,
    blacklisted_tools: Option<Vec<String>>,
    can_be_subagent: Option<bool>,
    comments_pane_enabled: Option<bool>,
    context_injection: Option<bool>,
    description: Option<String>,
    handle: Option<String>,
    icon: Option<String>,
    knowledge_base: Option<Vec<String>>,
    knowledge_base_tags: Option<Vec<String>>,
    max_past_messages_for_subagents: Option<i64>,
    meeting_attendance_enabled: Option<bool>,
    meeting_voice_enabled: Option<bool>,
    model: Option<String>,
    muses_enabled: Option<bool>,
    muses_recall_enabled: Option<bool>,
    name: Option<String>,
    parallel_tool_calls: Option<bool>,
    permanent_enabled_tools: Option<Vec<String>>,
    plan_mode_enabled: Option<bool>,
    remove_session_computer: Option<bool>,
    session_computer: Option<PublicSessionComputerIn>,
    show_only_for_admin: Option<bool>,
    sms_blocked_sender_mode: Option<String>,
    sms_blocked_sender_reply: Option<String>,
    sms_enabled: Option<bool>,
    sources: Option<Vec<String>>,
    strict_agent_configuration: Option<bool>,
    sub_agents: Option<Vec<String>>,
    suggestions: Option<Vec<HashMap<String, serde_json::Value>>>,
    system_prompt: Option<String>,
    temperature: Option<f64>,
    toolkits: Option<Vec<String>>,
    tools: Option<Vec<String>>,
    voice_enabled: Option<bool>,
    voice_sub_agents: Option<Vec<String>>,
    whitelisted_emails: Option<Vec<String>>,
}

impl UpdateAgentConfigRequestInBuilder {
    pub fn accent_color(mut self, value: impl Into<String>) -> Self {
        self.accent_color = Some(value.into());
        self
    }

    pub fn auto_compaction_enabled(mut self, value: bool) -> Self {
        self.auto_compaction_enabled = Some(value);
        self
    }

    pub fn available_toolkits(mut self, value: Vec<String>) -> Self {
        self.available_toolkits = Some(value);
        self
    }

    pub fn blacklisted_tools(mut self, value: Vec<String>) -> Self {
        self.blacklisted_tools = Some(value);
        self
    }

    pub fn can_be_subagent(mut self, value: bool) -> Self {
        self.can_be_subagent = Some(value);
        self
    }

    pub fn comments_pane_enabled(mut self, value: bool) -> Self {
        self.comments_pane_enabled = Some(value);
        self
    }

    pub fn context_injection(mut self, value: bool) -> Self {
        self.context_injection = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn knowledge_base(mut self, value: Vec<String>) -> Self {
        self.knowledge_base = Some(value);
        self
    }

    pub fn knowledge_base_tags(mut self, value: Vec<String>) -> Self {
        self.knowledge_base_tags = Some(value);
        self
    }

    pub fn max_past_messages_for_subagents(mut self, value: i64) -> Self {
        self.max_past_messages_for_subagents = Some(value);
        self
    }

    pub fn meeting_attendance_enabled(mut self, value: bool) -> Self {
        self.meeting_attendance_enabled = Some(value);
        self
    }

    pub fn meeting_voice_enabled(mut self, value: bool) -> Self {
        self.meeting_voice_enabled = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn muses_enabled(mut self, value: bool) -> Self {
        self.muses_enabled = Some(value);
        self
    }

    pub fn muses_recall_enabled(mut self, value: bool) -> Self {
        self.muses_recall_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn permanent_enabled_tools(mut self, value: Vec<String>) -> Self {
        self.permanent_enabled_tools = Some(value);
        self
    }

    pub fn plan_mode_enabled(mut self, value: bool) -> Self {
        self.plan_mode_enabled = Some(value);
        self
    }

    pub fn remove_session_computer(mut self, value: bool) -> Self {
        self.remove_session_computer = Some(value);
        self
    }

    pub fn session_computer(mut self, value: PublicSessionComputerIn) -> Self {
        self.session_computer = Some(value);
        self
    }

    pub fn show_only_for_admin(mut self, value: bool) -> Self {
        self.show_only_for_admin = Some(value);
        self
    }

    pub fn sms_blocked_sender_mode(mut self, value: impl Into<String>) -> Self {
        self.sms_blocked_sender_mode = Some(value.into());
        self
    }

    pub fn sms_blocked_sender_reply(mut self, value: impl Into<String>) -> Self {
        self.sms_blocked_sender_reply = Some(value.into());
        self
    }

    pub fn sms_enabled(mut self, value: bool) -> Self {
        self.sms_enabled = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn strict_agent_configuration(mut self, value: bool) -> Self {
        self.strict_agent_configuration = Some(value);
        self
    }

    pub fn sub_agents(mut self, value: Vec<String>) -> Self {
        self.sub_agents = Some(value);
        self
    }

    pub fn suggestions(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.suggestions = Some(value);
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn toolkits(mut self, value: Vec<String>) -> Self {
        self.toolkits = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn voice_enabled(mut self, value: bool) -> Self {
        self.voice_enabled = Some(value);
        self
    }

    pub fn voice_sub_agents(mut self, value: Vec<String>) -> Self {
        self.voice_sub_agents = Some(value);
        self
    }

    pub fn whitelisted_emails(mut self, value: Vec<String>) -> Self {
        self.whitelisted_emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAgentConfigRequestIn`].
    pub fn build(self) -> Result<UpdateAgentConfigRequestIn, BuildError> {
        Ok(UpdateAgentConfigRequestIn {
            accent_color: self.accent_color,
            auto_compaction_enabled: self.auto_compaction_enabled,
            available_toolkits: self.available_toolkits,
            blacklisted_tools: self.blacklisted_tools,
            can_be_subagent: self.can_be_subagent,
            comments_pane_enabled: self.comments_pane_enabled,
            context_injection: self.context_injection,
            description: self.description,
            handle: self.handle,
            icon: self.icon,
            knowledge_base: self.knowledge_base,
            knowledge_base_tags: self.knowledge_base_tags,
            max_past_messages_for_subagents: self.max_past_messages_for_subagents,
            meeting_attendance_enabled: self.meeting_attendance_enabled,
            meeting_voice_enabled: self.meeting_voice_enabled,
            model: self.model,
            muses_enabled: self.muses_enabled,
            muses_recall_enabled: self.muses_recall_enabled,
            name: self.name,
            parallel_tool_calls: self.parallel_tool_calls,
            permanent_enabled_tools: self.permanent_enabled_tools,
            plan_mode_enabled: self.plan_mode_enabled,
            remove_session_computer: self.remove_session_computer,
            session_computer: self.session_computer,
            show_only_for_admin: self.show_only_for_admin,
            sms_blocked_sender_mode: self.sms_blocked_sender_mode,
            sms_blocked_sender_reply: self.sms_blocked_sender_reply,
            sms_enabled: self.sms_enabled,
            sources: self.sources,
            strict_agent_configuration: self.strict_agent_configuration,
            sub_agents: self.sub_agents,
            suggestions: self.suggestions,
            system_prompt: self.system_prompt,
            temperature: self.temperature,
            toolkits: self.toolkits,
            tools: self.tools,
            voice_enabled: self.voice_enabled,
            voice_sub_agents: self.voice_sub_agents,
            whitelisted_emails: self.whitelisted_emails,
        })
    }
}

