use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use lark_error::Error;

pub mod lark_error;


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
        let uri = String::from("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal");
        let mut map = HashMap::new();
        map.insert("app_id", &self.app_id);
        map.insert("app_secret", &self.app_secret);
        let client = reqwest::blocking::Client::new();
        let res = client.post(&uri).header("content-type", "application/json").send();
        let text = match res {
            Ok(r) => match r.text() {
                Ok(t) => t,
                Err(e) => return Err(Error::new(-1, e.to_string())),
            },
            Err(e) => {
                return Err(Error::new(-1, e.to_string()));
            }
        };
        println!("text is {}", &text);
        let res_data: GetTokenResp = serde_json::from_str(text.as_str()).unwrap();
        if res_data.code != 0 {
            return Err(Error::new(res_data.code, res_data.msg));
        }
        return Ok(res_data.data);
    }
}