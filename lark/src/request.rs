use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawRequestReq {
    // api domain, such as Auth, Chat, Mail
    pub scope: String,
    // api name, as in CreateMailGroup
    pub api: String,
    // http request method, such as GET, POST
    pub method: String,
    // http request url
    pub url: String,
    // pub body: Any,  // http request body
    // Body                  interface{}   // http request body, query, path and other parameter information
    // send body data as a file
    pub is_file: bool,
    // IsFile                bool
    // do you need TenantAccessToken
    pub need_tenant_access_token: bool,
    // do you need AppAccessToken
    pub need_app_access_token: bool,
    // do you need UserAccessToken
    pub need_user_access_token: bool,
    // do you need HelpdeskAccessToken
    pub need_helpdesk_auth: bool,
// method option, such as userAccessToken
// MethodOption          *MethodOption
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    // Method        string      // request method
    // URL           string      // request url
    // RequestID     string      // request id, if you got some error and oncall lark/feishu team, please with this request id
    // StatusCode    int         // http response status code
    // Header        http.Header // http response header
    // ContentLength int64       // http response content length
}

// impl Lark {
//     pub fn raw_request(req: RawRequestReq) -> Result<Response, Error> {}
// }