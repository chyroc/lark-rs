use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use lark_error::Error;

pub mod lark_error;

// use lark_error::Error;

pub struct Lark {
    app_id: String,
    app_secret: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenRespData {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenResp {
    pub code: i32,
    pub data: GetTokenRespData,
    pub msg: String,
}


impl Lark {
    pub fn new(app_id: String, app_secret: String) -> Lark {
        Lark {
            app_id,
            app_secret,
        }
    }

    pub fn get_token(&self) -> Result<GetTokenRespData, Error> {
        println!("start get token");
        let uri = String::from("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token");
        let mut map = HashMap::new();
        map.insert("app_id", &self.app_id);
        map.insert("app_secret", &self.app_secret);
        let client = reqwest::blocking::Client::new();
        let res = client.post(&uri).json(&map).send().unwrap();
        let res2: GetTokenResp = res.json().unwrap();
        if res2.code == 0 {
            Ok(res2.data)
        } else {
            Err(Error {
                code: res2.code,
                message: res2.msg,
            })
        }
    }
}