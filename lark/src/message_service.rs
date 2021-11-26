use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Error;
use crate::Lark;

pub struct MessageService {
    pub client: Box<Lark>,
}

impl MessageService {
    pub fn new(client: Lark) -> Self {
        MessageService { client: Box::new(client) }
    }
}