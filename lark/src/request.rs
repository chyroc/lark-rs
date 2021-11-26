use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    // Method        string      // request method
    // URL           string      // request url
    // RequestID     string      // request id, if you got some error and oncall lark/feishu team, please with this request id
    // StatusCode    int         // http response status code
    // Header        http.Header // http response header
    // ContentLength int64       // http response content length
}
