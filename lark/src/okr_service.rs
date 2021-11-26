use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Error;
use crate::Lark;
use crate::request::Response;

pub struct OKRService {
    pub client: Box<Lark>,
}

impl OKRService {
    pub fn new(client: Lark) -> Self {
        OKRService { client: Box::new(client) }
    }

    pub fn get_okr_period_list(&self, req: GetOKRPeriodListReq) -> Result<GetOKRPeriodListResp, Error> {
        let uri = format!("https://open.feishu.cn/open-apis/okr/v1/periods");
        let tenant_token = self.client.auth().get_tenant_access_token().unwrap();

        let client = reqwest::blocking::Client::new();
        let res = client.get(&uri).
            header("Authorization", format!("Bearer {}", tenant_token.tenant_access_token)).
            send();
        let text = res.unwrap().text().unwrap();
        let res_data: GetOkrPeriodListRespOrigin = serde_json::from_str(text.as_str()).unwrap();
        if res_data.code != 0 {
            return Err(Error::new(res_data.code, res_data.msg));
        }

        Ok(GetOKRPeriodListResp {
            page_token: res_data.data.page_token,
            has_more: res_data.data.has_more,
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetOKRPeriodListReq {
    // 分页标志page_token, 示例值："xaasdasdax"
    #[datatype = "query"]
    pub page_token: Option<String>,
    // 分页大小，默认10, 示例值：10, 默认值: `10`
    #[datatype = "query"]
    pub page_size: Option<i32>,
}


#[derive(Debug, Serialize, Deserialize)]
struct GetOkrPeriodListRespOrigin {
    code: i32,
    data: GetOKRPeriodListResp,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOKRPeriodListResp {
    pub page_token: Option<String>,
    pub has_more: bool,
}

// type  struct {
// 	PageToken string                      `json:"page_token,omitempty"` // 分页标志
// 	HasMore   bool                        `json:"has_more,omitempty"`   // 是否有更多
// 	Items     []*GetOKRPeriodListRespItem `json:"items,omitempty"`      // 数据项
// }