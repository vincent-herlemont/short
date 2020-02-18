//! Cloudformation stack description
//! Stack is the remote model retrieve from AWS cloud (via CLI or API)
use std::cell::RefCell;

pub struct Stack {
    nested: Vec<RefCell<Stack>>,
}
