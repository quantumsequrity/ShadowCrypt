//! Inter-agent communication system
//!
//! Provides message passing and event systems for agents to communicate.

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::agents::{AgentId, AgentKind};

/// A message sent between agents
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// Unique message ID
    pub id: u64,
    /// Sender agent
    pub from: AgentId,
    /// Recipient agent
    pub to: AgentId,
    /// Message content
    pub content: String,
    /// Message type
    pub kind: MessageKind,
    /// Priority (higher = more important)
    pub priority: u32,
    /// Turn when sent
    pub sent_turn: u32,
    /// Whether the message has been read
    pub read: bool,
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

impl Message {
    /// Creates a new message
    pub fn new(from: AgentId, to: AgentId, content: String) -> Self {
        Self {
            id: rand::random(),
            from,
            to,
            content,
            kind: MessageKind::Chat,
            priority: 0,
            sent_turn: 0,
            read: false,
            metadata: HashMap::new(),
        }
    }

    /// Creates a message with a specific kind
    pub fn with_kind(mut self, kind: MessageKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Adds metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Types of messages
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageKind {
    /// General chat/dialogue
    Chat,
    /// Call for help
    HelpRequest,
    /// Warning about danger
    Warning,
    /// Command/order
    Command,
    /// Trade offer
    TradeOffer,
    /// Trade response
    TradeResponse,
    /// Information sharing
    Information,
    /// Greeting
    Greeting,
    /// Farewell
    Farewell,
    /// Threat
    Threat,
    /// Quest related
    Quest,
    /// System message
    System,
}

/// An event that occurred in the game world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Unique event ID
    pub id: u64,
    /// Event kind
    pub kind: EventKind,
    /// Turn when event occurred
    pub turn: u32,
    /// Position where event occurred (if applicable)
    pub position: Option<(usize, usize)>,
    /// Whether the event has been processed
    pub processed: bool,
}

impl Event {
    /// Creates a new event
    pub fn new(kind: EventKind) -> Self {
        Self {
            id: rand::random(),
            kind,
            turn: 0,
            position: None,
            processed: false,
        }
    }

    /// Sets the position
    pub fn at(mut self, x: usize, y: usize) -> Self {
        self.position = Some((x, y));
        self
    }
}

/// Types of events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventKind {
    // Agent events
    AgentSpawned { agent_id: AgentId, kind: AgentKind },
    AgentDied { agent_id: AgentId },
    AgentMoved { agent_id: AgentId, from: (usize, usize), to: (usize, usize) },
    AgentAttacked { attacker: AgentId, target: AgentId, damage: i32 },
    AgentSpoke { agent_id: AgentId, message: String, target: Option<AgentId> },
    AgentStateChanged { agent_id: AgentId, old_state: String, new_state: String },

    // Combat events
    CombatStarted { attacker: AgentId, defender: AgentId },
    CombatEnded { winner: Option<AgentId> },
    DamageDealt { source: AgentId, target: AgentId, amount: i32 },
    Healed { target: AgentId, amount: i32 },
    StatusApplied { target: AgentId, status: String, duration: u32 },

    // World events
    ItemDropped { item: String, x: usize, y: usize },
    ItemPickedUp { agent_id: AgentId, item: String },
    DoorOpened { x: usize, y: usize },
    TrapTriggered { x: usize, y: usize, damage: i32 },
    ChestOpened { x: usize, y: usize },

    // Environmental events
    WeatherChanged { from: String, to: String },
    TimeChanged { from: String, to: String },
    SeasonChanged { from: String, to: String },

    // Quest events
    QuestStarted { quest_id: u32, quest_name: String },
    QuestCompleted { quest_id: u32, quest_name: String },
    QuestFailed { quest_id: u32, quest_name: String },
    ObjectiveUpdated { quest_id: u32, objective: String, progress: u32, target: u32 },

    // Player events
    PlayerLevelUp { new_level: u32 },
    PlayerDied,
    PlayerVictory,

    // Custom events
    Custom { event_type: String, data: HashMap<String, String> },
}

/// Handler function type for events
pub type EventHandler = Box<dyn Fn(&Event) + Send + Sync>;

/// The message bus handles inter-agent communication
#[derive(Default)]
pub struct MessageBus {
    /// Pending messages
    messages: VecDeque<Message>,
    /// Inbox for each agent
    inboxes: HashMap<AgentId, VecDeque<Message>>,
    /// Event queue
    events: VecDeque<Event>,
    /// Event handlers
    handlers: Vec<EventHandler>,
    /// Message ID counter
    message_counter: u64,
    /// Event ID counter
    event_counter: u64,
    /// Current turn
    turn: u32,
}

impl MessageBus {
    /// Creates a new message bus
    pub fn new() -> Self {
        Self::default()
    }

    /// Sends a message
    pub fn send(&mut self, mut message: Message) {
        message.id = self.message_counter;
        message.sent_turn = self.turn;
        self.message_counter += 1;

        // Add to recipient's inbox
        self.inboxes
            .entry(message.to)
            .or_insert_with(VecDeque::new)
            .push_back(message.clone());

        self.messages.push_back(message);
    }

    /// Sends a message to all agents
    pub fn broadcast(&mut self, event: Event) {
        self.events.push_back(event);
    }

    /// Gets pending messages for an agent
    pub fn get_messages(&mut self, agent_id: AgentId) -> Vec<Message> {
        self.inboxes
            .get_mut(&agent_id)
            .map(|inbox| inbox.drain(..).collect())
            .unwrap_or_default()
    }

    /// Gets unread messages for an agent
    pub fn get_unread(&self, agent_id: AgentId) -> Vec<&Message> {
        self.inboxes
            .get(&agent_id)
            .map(|inbox| inbox.iter().filter(|m| !m.read).collect())
            .unwrap_or_default()
    }

    /// Marks a message as read
    pub fn mark_read(&mut self, agent_id: AgentId, message_id: u64) {
        if let Some(inbox) = self.inboxes.get_mut(&agent_id) {
            for message in inbox.iter_mut() {
                if message.id == message_id {
                    message.read = true;
                    break;
                }
            }
        }
    }

    /// Processes all pending events
    pub fn process_all(&mut self) {
        while let Some(mut event) = self.events.pop_front() {
            event.processed = true;
            for handler in &self.handlers {
                handler(&event);
            }
        }
    }

    /// Registers an event handler
    pub fn on_event<F>(&mut self, handler: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Gets all recent events
    pub fn recent_events(&self, count: usize) -> Vec<&Event> {
        self.events.iter().take(count).collect()
    }

    /// Gets recent messages from the global message log
    pub fn recent_messages(&self, count: usize) -> Vec<&Message> {
        self.messages.iter().rev().take(count).collect()
    }

    /// Advances the turn counter
    pub fn tick(&mut self) {
        self.turn += 1;
    }

    /// Clears old messages
    pub fn cleanup(&mut self, max_age: u32) {
        let cutoff = self.turn.saturating_sub(max_age);

        // Clean up main message queue
        self.messages.retain(|m| m.sent_turn >= cutoff);

        // Clean up inboxes
        for inbox in self.inboxes.values_mut() {
            inbox.retain(|m| m.sent_turn >= cutoff);
        }
    }
}

/// Conversation between agents
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    /// Participants
    pub participants: Vec<AgentId>,
    /// Message history
    pub history: Vec<ConversationMessage>,
    /// Topic of conversation
    pub topic: Option<String>,
    /// Whether the conversation is active
    pub active: bool,
    /// Turn when conversation started
    pub started_turn: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub speaker: AgentId,
    pub text: String,
    pub turn: u32,
}

impl Conversation {
    /// Starts a new conversation
    pub fn start(initiator: AgentId, target: AgentId) -> Self {
        Self {
            participants: vec![initiator, target],
            history: Vec::new(),
            topic: None,
            active: true,
            started_turn: 0,
        }
    }

    /// Adds a message to the conversation
    pub fn add_message(&mut self, speaker: AgentId, text: String, turn: u32) {
        if self.participants.contains(&speaker) {
            self.history.push(ConversationMessage { speaker, text, turn });
        }
    }

    /// Ends the conversation
    pub fn end(&mut self) {
        self.active = false;
    }

    /// Gets the last message
    pub fn last_message(&self) -> Option<&ConversationMessage> {
        self.history.last()
    }

    /// Gets messages from a specific speaker
    pub fn messages_from(&self, speaker: AgentId) -> Vec<&ConversationMessage> {
        self.history.iter().filter(|m| m.speaker == speaker).collect()
    }
}

/// Group communication channel
#[derive(Clone, Debug)]
pub struct Channel {
    /// Channel name
    pub name: String,
    /// Channel members
    pub members: Vec<AgentId>,
    /// Message history
    pub messages: VecDeque<Message>,
    /// Maximum history size
    pub max_history: usize,
}

impl Channel {
    /// Creates a new channel
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            members: Vec::new(),
            messages: VecDeque::new(),
            max_history: 100,
        }
    }

    /// Adds a member
    pub fn join(&mut self, agent_id: AgentId) {
        if !self.members.contains(&agent_id) {
            self.members.push(agent_id);
        }
    }

    /// Removes a member
    pub fn leave(&mut self, agent_id: AgentId) {
        self.members.retain(|&id| id != agent_id);
    }

    /// Sends a message to the channel
    pub fn send(&mut self, from: AgentId, content: String) {
        if self.members.contains(&from) {
            for &to in &self.members {
                if to != from {
                    let message = Message::new(from, to, content.clone());
                    self.messages.push_back(message);
                }
            }

            // Trim history
            while self.messages.len() > self.max_history {
                self.messages.pop_front();
            }
        }
    }

    /// Gets recent messages
    pub fn recent(&self, count: usize) -> Vec<&Message> {
        self.messages.iter().rev().take(count).collect()
    }
}

/// Manages all conversations and channels
#[derive(Default)]
pub struct ConversationManager {
    /// Active conversations
    conversations: HashMap<u64, Conversation>,
    /// Channels
    channels: HashMap<String, Channel>,
    /// Next conversation ID
    next_id: u64,
}

impl ConversationManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts a conversation
    pub fn start_conversation(&mut self, initiator: AgentId, target: AgentId) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.conversations.insert(id, Conversation::start(initiator, target));
        id
    }

    /// Gets a conversation
    pub fn get(&self, id: u64) -> Option<&Conversation> {
        self.conversations.get(&id)
    }

    /// Gets a mutable conversation
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Conversation> {
        self.conversations.get_mut(&id)
    }

    /// Ends a conversation
    pub fn end_conversation(&mut self, id: u64) {
        if let Some(conv) = self.conversations.get_mut(&id) {
            conv.end();
        }
    }

    /// Gets or creates a channel
    pub fn channel(&mut self, name: &str) -> &mut Channel {
        self.channels.entry(name.to_string()).or_insert_with(|| Channel::new(name))
    }

    /// Gets all active conversations for an agent
    pub fn active_for(&self, agent_id: AgentId) -> Vec<(&u64, &Conversation)> {
        self.conversations
            .iter()
            .filter(|(_, c)| c.active && c.participants.contains(&agent_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_sending() {
        let mut bus = MessageBus::new();
        let from = AgentId::new();
        let to = AgentId::new();

        bus.send(Message::new(from, to, "Hello!".to_string()));

        let messages = bus.get_messages(to);
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello!");
    }

    #[test]
    fn test_conversation() {
        let mut manager = ConversationManager::new();
        let agent1 = AgentId::new();
        let agent2 = AgentId::new();

        let id = manager.start_conversation(agent1, agent2);
        if let Some(conv) = manager.get_mut(id) {
            conv.add_message(agent1, "Hello!".to_string(), 1);
            conv.add_message(agent2, "Hi there!".to_string(), 2);
        }

        let conv = manager.get(id).unwrap();
        assert_eq!(conv.history.len(), 2);
    }
}
