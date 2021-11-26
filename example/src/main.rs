use std::env;
use std::io::Error;

use lark::Lark;

fn main() {
    let app_id = env::var("LARK_APP_ALL_PERMISSION_APP_ID").expect("id not set");
    let app_secret = env::var("LARK_APP_ALL_PERMISSION_APP_SECRET").expect("id not set");

    println!("app_id is {}", &app_id);
    println!("app_secret is {}", &app_secret);

    let ins = Lark::new(app_id, app_secret);
    let lark_auth = ins.auth();

    // println!("tenant token is {:#?}", lark_auth.get_tenant_access_token().unwrap());
    println!("app token is {:#?}", lark_auth.get_app_access_token().unwrap());
}
