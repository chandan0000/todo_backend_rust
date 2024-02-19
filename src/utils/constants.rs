use dotenvy::dotenv;
use dotenvy_macro::dotenv;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref TOKEN: String = set_token();
}
pub fn set_token() -> String{
    dotenv().ok();
    dotenv!("TOKEN").to_string()
    // env::var("TOKEN").unwrap()
}