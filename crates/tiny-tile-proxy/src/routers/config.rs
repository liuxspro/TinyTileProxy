use crate::libs::config::{get_config, save_tokens, Config, StateConfig, Tokens};
use rocket::serde::json::Json;
use rocket::State;

#[derive(FromForm)]
pub(crate) struct PASS {
    pwd: String,
}

pub fn get_pwd() -> Result<String, figment::Error> {
    let config: Config = get_config().extract()?;
    let pwd = config.default.password;
    Ok(pwd)
}

#[get("/config/auth?<q..>")]
pub fn auth(q: PASS) -> String {
    let local_pwd = get_pwd().unwrap();
    if q.pwd == local_pwd {
        return "Ok".to_string();
    } else {
        return "Error".to_string();
    }
}

#[get("/config/tokens?<query..>")]
pub fn show_tokens(config: &State<StateConfig>, query: PASS) -> Json<Tokens> {
    let local_pwd = get_pwd().unwrap();
    if query.pwd == local_pwd {
        Json(config.tokens.read().unwrap().clone())
    } else {
        Json(Tokens {
            geocloud: "".to_string(),
            jl1: "".to_string(),
            jl1earth: "".to_string(),
        })
    }
}

#[post("/config/set_tokens?<query..>", data = "<tokens>")]
pub fn set_tokens(tokens: Json<Tokens>, config: &State<StateConfig>, query: PASS) -> String {
    let local_pwd = get_pwd().unwrap();
    if query.pwd == local_pwd {
        let token = tokens.into_inner();
        // 修改 State
        *config.tokens.write().unwrap() = token.clone();
        // 保存 Token
        save_tokens(&token).unwrap();
        "ok".to_string()
    } else {
        "Error".to_string()
    }
}
