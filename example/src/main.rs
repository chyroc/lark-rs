use std::env;
use std::io::Error;

use serde::{Deserialize, Serialize};

use lark;
use lark::Lark;

fn main() {
    let app_id = env::var("LARK_APP_ALL_PERMISSION_APP_ID").expect("id not set");
    let app_secret = env::var("LARK_APP_ALL_PERMISSION_APP_SECRET").expect("id not set");

    println!("app_id is {}", &app_id);
    println!("app_secret is {}", &app_secret);

    let ins = Lark::new(app_id, app_secret);

    let res = ins.get_tenant_access_token();
    match res {
        Ok(res) => {
            println!("res is {:#?}", res);
        }
        Err(err) => {
            println!("err is {}", err.message)
        }
    }
}
