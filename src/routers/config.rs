use crate::libs::config::{save_tokens, StateConfig, Tokens};
use rocket::serde::json::Json;
use rocket::State;

#[get("/config/tokens")]
pub fn show_tokens(config: &State<StateConfig>) -> Json<Tokens> {
    Json(config.tokens.read().unwrap().clone())
}

#[post("/config/set_tokens", data = "<tokens>")]
pub fn set_tokens(tokens: Json<Tokens>, config: &State<StateConfig>) -> String {
    let token = tokens.into_inner();
    // 修改 State
    *config.tokens.write().unwrap() = token.clone();
    // 保存 Token
    save_tokens(&token).unwrap();
    "ok".to_string()
}
