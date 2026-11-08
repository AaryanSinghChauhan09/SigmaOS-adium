//! SigmaOS-adium — comprehensive instant messaging component.
//!
//! Provides instant messaging functionality including chat protocols,
//! contact management, message history, and multi-protocol support.

use std::collections::HashMap;

/// Chat protocol
#[derive(Debug, Clone, PartialEq)]
pub enum ChatProtocol {
    /// XMPP/Jabber
    XMPP,
    /// IRC
    IRC,
    /// AIM
    AIM,
    /// ICQ
    ICQ,
    /// Yahoo Messenger
    Yahoo,
    /// MSN Messenger
    MSN,
    /// Google Talk
    GoogleTalk,
    /// Facebook Messenger
    Facebook,
    /// WhatsApp
    WhatsApp,
    /// Telegram
    Telegram,
    /// Signal
    Signal,
    /// Matrix
    Matrix,
    /// Custom protocol
    Custom(String),
}

/// Contact status
#[derive(Debug, Clone, PartialEq)]
pub enum ContactStatus {
    /// Online
    Online,
    /// Away
    Away,
    /// Busy
    Busy,
    /// Invisible
    Invisible,
    /// Offline
    Offline,
    /// Do Not Disturb
    DoNotDisturb,
}

/// Message type
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// Text message
    Text,
    /// Image message
    Image,
    /// File message
    File,
    /// Audio message
    Audio,
    /// Video message
    Video,
    /// Location message
    Location,
    /// Contact message
    Contact,
    /// System message
    System,
}

/// Message status
#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    /// Sending
    Sending,
    /// Sent
    Sent,
    /// Delivered
    Delivered,
    /// Read
    Read,
    /// Failed
    Failed,
}

/// Chat contact
#[derive(Debug, Clone)]
pub struct ChatContact {
    /// Contact ID
    pub id: String,
    /// Contact name
    pub name: String,
    /// Contact alias/nickname
    pub alias: String,
    /// Protocol
    pub protocol: ChatProtocol,
    /// Status
    pub status: ContactStatus,
    /// Status message
    pub status_message: String,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// Last seen timestamp
    pub last_seen: u64,
    /// Groups
    pub groups: Vec<String>,
    /// Notes
    pub notes: String,
    /// Blocked flag
    pub blocked: bool,
}

/// Chat message
#[derive(Debug, Clone)]
pub struct ChatMessage {
    /// Message ID
    pub id: String,
    /// Sender ID
    pub sender_id: String,
    /// Recipient ID
    pub recipient_id: String,
    /// Message type
    pub message_type: MessageType,
    /// Message content
    pub content: String,
    /// Message status
    pub status: MessageStatus,
    /// Timestamp
    pub timestamp: u64,
    /// Reply to message ID
    pub reply_to: Option<String>,
    /// Attachments
    pub attachments: Vec<Attachment>,
    /// Read timestamp
    pub read_at: Option<u64>,
    /// Delivered timestamp
    pub delivered_at: Option<u64>,
}

/// Attachment
#[derive(Debug, Clone)]
pub struct Attachment {
    /// Attachment ID
    pub id: String,
    /// Attachment name
    pub name: String,
    /// Attachment type
    pub attachment_type: String,
    /// File size
    pub size: u64,
    /// Download URL
    pub download_url: String,
    /// Thumbnail URL
    pub thumbnail_url: Option<String>,
}

/// Chat conversation
#[derive(Debug, Clone)]
pub struct ChatConversation {
    /// Conversation ID
    pub id: String,
    /// Conversation type (direct, group, channel)
    pub conversation_type: ConversationType,
    /// Participant IDs
    pub participants: Vec<String>,
    /// Conversation name (for groups)
    pub name: String,
    /// Last message
    pub last_message: Option<String>,
    /// Last message timestamp
    pub last_message_time: u64,
    /// Unread count
    pub unread_count: u32,
    /// Messages
    pub messages: Vec<ChatMessage>,
    /// Archived flag
    pub archived: bool,
}

/// Conversation type
#[derive(Debug, Clone, PartialEq)]
pub enum ConversationType {
    /// Direct message
    Direct,
    /// Group chat
    Group,
    /// Channel
    Channel,
    /// Chat room
    ChatRoom,
}

/// Chat account
#[derive(Debug, Clone)]
pub struct ChatAccount {
    /// Account ID
    pub id: String,
    /// Account name
    pub account_name: String,
    /// Protocol
    pub protocol: ChatProtocol,
    /// Username
    pub username: String,
    /// Password (encrypted)
    pub password: String,
    /// Server address
    pub server: String,
    /// Port
    pub port: u16,
    /// Connected flag
    pub connected: bool,
    /// Auto-connect flag
    pub auto_connect: bool,
    /// Status
    pub status: ContactStatus,
    /// Status message
    pub status_message: String,
}

/// Instant messaging service
#[derive(Debug, Clone)]
pub struct InstantMessagingService {
    /// Accounts
    pub accounts: HashMap<String, ChatAccount>,
    /// Contacts
    pub contacts: HashMap<String, ChatContact>,
    /// Conversations
    pub conversations: HashMap<String, ChatConversation>,
    /// Messages
    pub messages: HashMap<String, ChatMessage>,
    /// Statistics
    pub stats: IMStatistics,
}

/// IM statistics
#[derive(Debug, Clone, Default)]
pub struct IMStatistics {
    /// Total accounts
    pub total_accounts: u64,
    /// Total contacts
    pub total_contacts: u64,
    /// Total conversations
    pub total_conversations: u64,
    /// Total messages
    pub total_messages: u64,
    /// Online contacts
    pub online_contacts: u64,
    /// Last message timestamp
    pub last_message_time: u64,
}

impl Default for InstantMessagingService {
    fn default() -> Self {
        Self {
            accounts: HashMap::new(),
            contacts: HashMap::new(),
            conversations: HashMap::new(),
            messages: HashMap::new(),
            stats: IMStatistics::default(),
        }
    }
}

impl InstantMessagingService {
    /// Create new IM service
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with default account
    pub fn initialize(&mut self) {
        // Add default account
        let account = ChatAccount {
            id: "default".into(),
            account_name: "Default Account".into(),
            protocol: ChatProtocol::XMPP,
            username: "user".into(),
            password: "password".into(),
            server: "xmpp.example.com".into(),
            port: 5222,
            connected: false,
            auto_connect: true,
            status: ContactStatus::Offline,
            status_message: String::new(),
        };
        
        self.accounts.insert(account.id.clone(), account);
        self.stats.total_accounts += 1;
    }
    
    /// Add account
    pub fn add_account(&mut self, account: ChatAccount) -> String {
        let id = account.id.clone();
        self.accounts.insert(id.clone(), account);
        self.stats.total_accounts += 1;
        id
    }
    
    /// Remove account
    pub fn remove_account(&mut self, account_id: &str) -> bool {
        if self.accounts.remove(account_id).is_some() {
            self.stats.total_accounts = self.stats.total_accounts.saturating_sub(1);
            return true;
        }
        false
    }
    
    /// Get account
    pub fn get_account(&self, account_id: &str) -> Option<&ChatAccount> {
        self.accounts.get(account_id)
    }
    
    /// Get all accounts
    pub fn get_all_accounts(&self) -> Vec<&ChatAccount> {
        self.accounts.values().collect()
    }
    
    /// Connect account
    pub fn connect_account(&mut self, account_id: &str) -> bool {
        if let Some(account) = self.accounts.get_mut(account_id) {
            account.connected = true;
            account.status = ContactStatus::Online;
            return true;
        }
        false
    }
    
    /// Disconnect account
    pub fn disconnect_account(&mut self, account_id: &str) -> bool {
        if let Some(account) = self.accounts.get_mut(account_id) {
            account.connected = false;
            account.status = ContactStatus::Offline;
            return true;
        }
        false
    }
    
    /// Add contact
    pub fn add_contact(&mut self, contact: ChatContact) -> String {
        let id = contact.id.clone();
        self.contacts.insert(id.clone(), contact);
        self.stats.total_contacts += 1;
        id
    }
    
    /// Remove contact
    pub fn remove_contact(&mut self, contact_id: &str) -> bool {
        if self.contacts.remove(contact_id).is_some() {
            self.stats.total_contacts = self.stats.total_contacts.saturating_sub(1);
            return true;
        }
        false
    }
    
    /// Get contact
    pub fn get_contact(&self, contact_id: &str) -> Option<&ChatContact> {
        self.contacts.get(contact_id)
    }
    
    /// Get contact mutably
    pub fn get_contact_mut(&mut self, contact_id: &str) -> Option<&mut ChatContact> {
        self.contacts.get_mut(contact_id)
    }
    
    /// Get all contacts
    pub fn get_all_contacts(&self) -> Vec<&ChatContact> {
        self.contacts.values().collect()
    }
    
    /// Get online contacts
    pub fn get_online_contacts(&self) -> Vec<&ChatContact> {
        self.contacts.values()
            .filter(|c| c.status == ContactStatus::Online)
            .collect()
    }
    
    /// Update contact status
    pub fn update_contact_status(&mut self, contact_id: &str, status: ContactStatus, status_message: String) -> bool {
        if let Some(contact) = self.contacts.get_mut(contact_id) {
            contact.status = status;
            contact.status_message = status_message;
            contact.last_seen = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return true;
        }
        false
    }
    
    /// Block contact
    pub fn block_contact(&mut self, contact_id: &str) -> bool {
        if let Some(contact) = self.contacts.get_mut(contact_id) {
            contact.blocked = true;
            return true;
        }
        false
    }
    
    /// Unblock contact
    pub fn unblock_contact(&mut self, contact_id: &str) -> bool {
        if let Some(contact) = self.contacts.get_mut(contact_id) {
            contact.blocked = false;
            return true;
        }
        false
    }
    
    /// Create conversation
    pub fn create_conversation(&mut self, conversation_type: ConversationType, participants: Vec<String>, name: String) -> String {
        let id = format!("conv-{}", self.stats.total_conversations + 1);
        
        let conversation = ChatConversation {
            id: id.clone(),
            conversation_type,
            participants,
            name,
            last_message: None,
            last_message_time: 0,
            unread_count: 0,
            messages: vec![],
            archived: false,
        };
        
        self.conversations.insert(id.clone(), conversation);
        self.stats.total_conversations += 1;
        id
    }
    
    /// Get conversation
    pub fn get_conversation(&self, conversation_id: &str) -> Option<&ChatConversation> {
        self.conversations.get(conversation_id)
    }
    
    /// Get conversation mutably
    pub fn get_conversation_mut(&mut self, conversation_id: &str) -> Option<&mut ChatConversation> {
        self.conversations.get_mut(conversation_id)
    }
    
    /// Get all conversations
    pub fn get_all_conversations(&self) -> Vec<&ChatConversation> {
        self.conversations.values().collect()
    }
    
    /// Send message
    pub fn send_message(&mut self, sender_id: &str, recipient_id: &str, content: String, message_type: MessageType) -> String {
        let message_id = format!("msg-{}", self.stats.total_messages + 1);
        
        let message = ChatMessage {
            id: message_id.clone(),
            sender_id: sender_id.to_string(),
            recipient_id: recipient_id.to_string(),
            message_type,
            content: content.clone(),
            status: MessageStatus::Sent,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            reply_to: None,
            attachments: vec![],
            read_at: None,
            delivered_at: None,
        };
        
        // Store message
        self.messages.insert(message_id.clone(), message.clone());
        self.stats.total_messages += 1;
        self.stats.last_message_time = message.timestamp;
        
        // Update conversation
        let timestamp = message.timestamp;
        if let Some(conversation) = self.conversations.values_mut().find(|c| {
            c.participants.contains(&sender_id.to_string()) && c.participants.contains(&recipient_id.to_string())
        }) {
            conversation.messages.push(message);
            conversation.last_message = Some(content);
            conversation.last_message_time = timestamp;
        }
        
        message_id
    }
    
    /// Get message
    pub fn get_message(&self, message_id: &str) -> Option<&ChatMessage> {
        self.messages.get(message_id)
    }
    
    /// Get all messages
    pub fn get_all_messages(&self) -> Vec<&ChatMessage> {
        self.messages.values().collect()
    }
    
    /// Get messages by conversation
    pub fn get_messages_by_conversation(&self, conversation_id: &str) -> Vec<&ChatMessage> {
        self.conversations.get(conversation_id)
            .map(|c| c.messages.iter().collect())
            .unwrap_or_default()
    }
    
    /// Mark message as read
    pub fn mark_message_read(&mut self, message_id: &str) -> bool {
        if let Some(message) = self.messages.get_mut(message_id) {
            message.status = MessageStatus::Read;
            message.read_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            return true;
        }
        false
    }
    
    /// Mark message as delivered
    pub fn mark_message_delivered(&mut self, message_id: &str) -> bool {
        if let Some(message) = self.messages.get_mut(message_id) {
            message.status = MessageStatus::Delivered;
            message.delivered_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            return true;
        }
        false
    }
    
    /// Search contacts
    pub fn search_contacts(&self, query: &str) -> Vec<&ChatContact> {
        let query_lower = query.to_lowercase();
        self.contacts.values()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower) ||
                c.alias.to_lowercase().contains(&query_lower) ||
                c.id.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Get IM statistics
    pub fn get_statistics(&self) -> &IMStatistics {
        &self.stats
    }
    
    /// Export IM data
    pub fn export_data(&self) -> IMExport {
        IMExport {
            accounts: self.accounts.values().cloned().collect(),
            contacts: self.contacts.values().cloned().collect(),
            conversations: self.conversations.values().cloned().collect(),
            messages: self.messages.values().cloned().collect(),
        }
    }
    
    /// Import IM data
    pub fn import_data(&mut self, export: IMExport) {
        for account in export.accounts {
            self.accounts.insert(account.id.clone(), account);
        }
        
        for contact in export.contacts {
            self.contacts.insert(contact.id.clone(), contact);
        }
        
        for conversation in export.conversations {
            self.conversations.insert(conversation.id.clone(), conversation);
        }
        
        for message in export.messages {
            self.messages.insert(message.id.clone(), message);
        }
        
        // Recalculate statistics
        self.stats.total_accounts = self.accounts.len() as u64;
        self.stats.total_contacts = self.contacts.len() as u64;
        self.stats.total_conversations = self.conversations.len() as u64;
        self.stats.total_messages = self.messages.len() as u64;
        self.stats.online_contacts = self.contacts.values()
            .filter(|c| c.status == ContactStatus::Online)
            .count() as u64;
        self.stats.last_message_time = self.messages.values()
            .map(|m| m.timestamp)
            .max()
            .unwrap_or(0);
    }
}

/// IM export data
#[derive(Debug, Clone)]
pub struct IMExport {
    /// Accounts
    pub accounts: Vec<ChatAccount>,
    /// Contacts
    pub contacts: Vec<ChatContact>,
    /// Conversations
    pub conversations: Vec<ChatConversation>,
    /// Messages
    pub messages: Vec<ChatMessage>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chat_protocol() {
        let protocols = vec![
            ChatProtocol::XMPP,
            ChatProtocol::IRC,
            ChatProtocol::AIM,
            ChatProtocol::ICQ,
            ChatProtocol::Yahoo,
            ChatProtocol::MSN,
            ChatProtocol::GoogleTalk,
            ChatProtocol::Facebook,
            ChatProtocol::WhatsApp,
            ChatProtocol::Telegram,
            ChatProtocol::Signal,
            ChatProtocol::Matrix,
        ];
        assert_eq!(protocols.len(), 12);
    }
    
    #[test]
    fn test_contact_status() {
        let statuses = vec![
            ContactStatus::Online,
            ContactStatus::Away,
            ContactStatus::Busy,
            ContactStatus::Invisible,
            ContactStatus::Offline,
            ContactStatus::DoNotDisturb,
        ];
        assert_eq!(statuses.len(), 6);
    }
    
    #[test]
    fn test_message_type() {
        let types = vec![
            MessageType::Text,
            MessageType::Image,
            MessageType::File,
            MessageType::Audio,
            MessageType::Video,
            MessageType::Location,
            MessageType::Contact,
            MessageType::System,
        ];
        assert_eq!(types.len(), 8);
    }
    
    #[test]
    fn test_message_status() {
        let statuses = vec![
            MessageStatus::Sending,
            MessageStatus::Sent,
            MessageStatus::Delivered,
            MessageStatus::Read,
            MessageStatus::Failed,
        ];
        assert_eq!(statuses.len(), 5);
    }
    
    #[test]
    fn test_conversation_type() {
        let types = vec![
            ConversationType::Direct,
            ConversationType::Group,
            ConversationType::Channel,
            ConversationType::ChatRoom,
        ];
        assert_eq!(types.len(), 4);
    }
    
    #[test]
    fn test_service_creation() {
        let service = InstantMessagingService::new();
        assert_eq!(service.get_statistics().total_accounts, 0);
    }
    
    #[test]
    fn test_initialize() {
        let mut service = InstantMessagingService::new();
        service.initialize();
        
        assert!(service.get_statistics().total_accounts > 0);
        assert!(service.get_account("default").is_some());
    }
    
    #[test]
    fn test_add_account() {
        let mut service = InstantMessagingService::new();
        
        let account = ChatAccount {
            id: "test".into(),
            account_name: "Test Account".into(),
            protocol: ChatProtocol::IRC,
            username: "testuser".into(),
            password: "testpass".into(),
            server: "irc.example.com".into(),
            port: 6667,
            connected: false,
            auto_connect: true,
            status: ContactStatus::Offline,
            status_message: String::new(),
        };
        
        service.add_account(account);
        assert!(service.get_account("test").is_some());
        assert_eq!(service.get_statistics().total_accounts, 1);
    }
    
    #[test]
    fn test_connect_disconnect_account() {
        let mut service = InstantMessagingService::new();
        service.initialize();
        
        assert!(service.connect_account("default"));
        assert_eq!(service.get_account("default").unwrap().status, ContactStatus::Online);
        
        assert!(service.disconnect_account("default"));
        assert_eq!(service.get_account("default").unwrap().status, ContactStatus::Offline);
    }
    
    #[test]
    fn test_add_contact() {
        let mut service = InstantMessagingService::new();
        
        let contact = ChatContact {
            id: "contact1".into(),
            name: "John Doe".into(),
            alias: "johndoe".into(),
            protocol: ChatProtocol::XMPP,
            status: ContactStatus::Online,
            status_message: "Available".into(),
            avatar_url: None,
            last_seen: 1234567890,
            groups: vec![],
            notes: String::new(),
            blocked: false,
        };
        
        service.add_contact(contact);
        assert!(service.get_contact("contact1").is_some());
        assert_eq!(service.get_statistics().total_contacts, 1);
    }
    
    #[test]
    fn test_update_contact_status() {
        let mut service = InstantMessagingService::new();
        
        let contact = ChatContact {
            id: "contact1".into(),
            name: "John Doe".into(),
            alias: "johndoe".into(),
            protocol: ChatProtocol::XMPP,
            status: ContactStatus::Online,
            status_message: "Available".into(),
            avatar_url: None,
            last_seen: 1234567890,
            groups: vec![],
            notes: String::new(),
            blocked: false,
        };
        
        service.add_contact(contact);
        
        assert!(service.update_contact_status("contact1", ContactStatus::Busy, "In a meeting".into()));
        
        let contact = service.get_contact("contact1").unwrap();
        assert_eq!(contact.status, ContactStatus::Busy);
        assert_eq!(contact.status_message, "In a meeting");
    }
    
    #[test]
    fn test_block_unblock_contact() {
        let mut service = InstantMessagingService::new();
        
        let contact = ChatContact {
            id: "contact1".into(),
            name: "John Doe".into(),
            alias: "johndoe".into(),
            protocol: ChatProtocol::XMPP,
            status: ContactStatus::Online,
            status_message: "Available".into(),
            avatar_url: None,
            last_seen: 1234567890,
            groups: vec![],
            notes: String::new(),
            blocked: false,
        };
        
        service.add_contact(contact);
        
        assert!(service.block_contact("contact1"));
        assert!(service.get_contact("contact1").unwrap().blocked);
        
        assert!(service.unblock_contact("contact1"));
        assert!(!service.get_contact("contact1").unwrap().blocked);
    }
    
    #[test]
    fn test_create_conversation() {
        let mut service = InstantMessagingService::new();
        
        let conversation_id = service.create_conversation(
            ConversationType::Direct,
            vec!["user1".into(), "user2".into()],
            String::new()
        );
        
        assert!(service.get_conversation(&conversation_id).is_some());
        assert_eq!(service.get_statistics().total_conversations, 1);
    }
    
    #[test]
    fn test_send_message() {
        let mut service = InstantMessagingService::new();
        
        // Create conversation first
        service.create_conversation(
            ConversationType::Direct,
            vec!["user1".into(), "user2".into()],
            String::new()
        );
        
        let message_id = service.send_message("user1", "user2", "Hello!".into(), MessageType::Text);
        
        assert!(service.get_message(&message_id).is_some());
        assert_eq!(service.get_statistics().total_messages, 1);
        
        let message = service.get_message(&message_id).unwrap();
        assert_eq!(message.content, "Hello!");
        assert_eq!(message.message_type, MessageType::Text);
    }
    
    #[test]
    fn test_get_messages_by_conversation() {
        let mut service = InstantMessagingService::new();
        
        let conversation_id = service.create_conversation(
            ConversationType::Direct,
            vec!["user1".into(), "user2".into()],
            String::new()
        );
        
        service.send_message("user1", "user2", "Hello!".into(), MessageType::Text);
        service.send_message("user2", "user1", "Hi!".into(), MessageType::Text);
        
        let messages = service.get_messages_by_conversation(&conversation_id);
        assert_eq!(messages.len(), 2);
    }
    
    #[test]
    fn test_mark_message_read() {
        let mut service = InstantMessagingService::new();
        
        let message_id = service.send_message("user1", "user2", "Hello!".into(), MessageType::Text);
        
        assert!(service.mark_message_read(&message_id));
        
        let message = service.get_message(&message_id).unwrap();
        assert_eq!(message.status, MessageStatus::Read);
        assert!(message.read_at.is_some());
    }
    
    #[test]
    fn test_mark_message_delivered() {
        let mut service = InstantMessagingService::new();
        
        let message_id = service.send_message("user1", "user2", "Hello!".into(), MessageType::Text);
        
        assert!(service.mark_message_delivered(&message_id));
        
        let message = service.get_message(&message_id).unwrap();
        assert_eq!(message.status, MessageStatus::Delivered);
        assert!(message.delivered_at.is_some());
    }
    
    #[test]
    fn test_search_contacts() {
        let mut service = InstantMessagingService::new();
        
        service.add_contact(ChatContact {
            id: "contact1".into(),
            name: "John Doe".into(),
            alias: "johndoe".into(),
            protocol: ChatProtocol::XMPP,
            status: ContactStatus::Online,
            status_message: "Available".into(),
            avatar_url: None,
            last_seen: 1234567890,
            groups: vec![],
            notes: String::new(),
            blocked: false,
        });
        
        service.add_contact(ChatContact {
            id: "contact2".into(),
            name: "Jane Smith".into(),
            alias: "janesmith".into(),
            protocol: ChatProtocol::IRC,
            status: ContactStatus::Online,
            status_message: "Available".into(),
            avatar_url: None,
            last_seen: 1234567890,
            groups: vec![],
            notes: String::new(),
            blocked: false,
        });
        
        let results = service.search_contacts("john");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "John Doe");
    }
    
    #[test]
    fn test_export_import() {
        let mut service = InstantMessagingService::new();
        service.initialize();
        
        service.add_contact(ChatContact {
            id: "contact1".into(),
            name: "John Doe".into(),
            alias: "johndoe".into(),
            protocol: ChatProtocol::XMPP,
            status: ContactStatus::Online,
            status_message: "Available".into(),
            avatar_url: None,
            last_seen: 1234567890,
            groups: vec![],
            notes: String::new(),
            blocked: false,
        });
        
        let export = service.export_data();
        
        let mut service2 = InstantMessagingService::new();
        service2.import_data(export);
        
        assert_eq!(service2.get_statistics().total_accounts, service.get_statistics().total_accounts);
        assert_eq!(service2.get_statistics().total_contacts, service.get_statistics().total_contacts);
    }
}
