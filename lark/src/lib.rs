use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use err::Error;

use crate::auth_service::AuthService;

pub mod err;
pub mod request;
pub mod auth_service;


pub struct Lark {
    app_id: String,
    app_secret: String,

    pub auth: AuthService,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenExpire {
    pub expire: i32,
    pub tenant_access_token: String,
}


impl Lark {
    pub fn new(app_id: String, app_secret: String) -> Lark {
        return Lark {
            app_id,
            app_secret,
            auth: AuthService::new(self),
        };
    }
}


