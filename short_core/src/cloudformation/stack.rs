//! Cloudformation stack description
//! Stack is the remote model retrieve from AWS cloud (via CLI or API)
use std::cell::RefCell;

#[allow(dead_code)]
pub struct Stack {
    pub nested: Vec<RefCell<Stack>>,
    pub content_stack: ContentStack,
}

#[allow(dead_code)]
pub struct ContentStack {}
