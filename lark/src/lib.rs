#[allow(dead_code)]
#[allow(dead_code)]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use err::Error;

use crate::auth_service::AuthService;
use crate::okr_service::OKRService;

pub mod err;
pub mod request;
pub mod auth_service;
pub mod message_service;
pub mod okr_service;


#[derive(Clone)]
pub struct Lark {
    app_id: String,
    app_secret: String,
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
        };
    }


    pub fn auth(&self) -> AuthService {
        AuthService::new(self.clone())
    }

    pub fn okr(&self) -> OKRService {
        OKRService::new(self.clone())
    }
}


