use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Error;
use crate::Lark;

pub struct OKRService {
    pub client: Box<Lark>,
}

impl OKRService {
    pub fn new(client: Lark) -> Self {
        OKRService { client: Box::new(client) }
    }

    pub fn get_okr_period_list(&self, req: GetOKRPeriodListReq) {
        let uri = format!("https://open.feishu.cn/open-apis/okr/v1/periods");
        let tenant_token = self.client.auth().get_tenant_access_token().unwrap();

        let client = reqwest::blocking::Client::new();
        let res = client.get(&uri).send();
        let text = res.unwrap().text().unwrap();
        println!("text is {}", text);
        // let res_data: GetTokenResp = serde_json::from_str(text.as_str()).unwrap();
        // if res_data.code != 0 {
        //     return Err(Error::new(res_data.code, res_data.msg));
        // }
        // return Ok(TokenExpire {
        //     expire: res_data.expire,
        //     tenant_access_token: res_data.tenant_access_token,
        // });
    }
}


pub struct GetOKRPeriodListReq {
    // 分页标志page_token, 示例值："xaasdasdax"
    pub page_token: Option<String>,
    // 分页大小，默认10, 示例值：10, 默认值: `10`
    pub page_size: Option<i32>,
}