use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Sender, Receiver};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetEvent {
    Update(String),  // nombre del widget
    Reload(String),  // nombre del widget
    Show(String),
    Hide(String),
    Data(String, serde_json::Value),  // widget, data
}

pub struct EventBus {
    sender: Sender<WidgetEvent>,
    receiver: Receiver<WidgetEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }

    pub fn sender(&self) -> Sender<WidgetEvent> {
        self.sender.clone()
    }

    pub fn try_recv(&self) -> Option<WidgetEvent> {
        self.receiver.try_recv().ok()
    }

    pub fn recv(&self) -> Option<WidgetEvent> {
        self.receiver.recv().ok()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}