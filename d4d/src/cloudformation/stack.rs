//! Cloudformation stack description
//! Stack is the remote model retrieve from AWS cloud (via CLI or API)
use std::cell::RefCell;

pub struct Stack {
    pub nested: Vec<RefCell<Stack>>,
    pub content_stack: ContentStack,
}

pub struct ContentStack {}
