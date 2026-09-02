use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    // TODO: Implement this function to create and return a message channel
    mpsc::channel()

}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    thread::spawn(move || {
        for mut message in messages {
            message.priority = if message.content.contains("ERROR") {
                Priority::Critical
            } else if message.content.contains("WARNING") {
                Priority::High
            } else if message.content.contains("DEBUG") {
                Priority::Medium
            } else {
                Priority::Low
            };
            tx.send(message).unwrap();
        }
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    thread::spawn(move || {
        let mut results = vec![];
        while let Ok(message) = rx.recv() {
            let priority = match message.priority {
                Priority::Low => "LOW",
                Priority::Medium => "MED",
                Priority::High => "HIGH",
                Priority::Critical => "CRIT",
            };
            results.push(format!(
                "[{}|{}] {}",
                priority, message.sender_id, message.content
            ));
        }
        results
    })
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}
