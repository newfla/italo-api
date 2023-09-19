use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LoginRequestBody<'a> {
    login: LoginRequestInternal<'a>,
    source_system: i8,
}

impl Default for LoginRequestBody<'_> {
    fn default() -> Self {
        LoginRequestBody {
            login: LoginRequestInternal {
                domain: "WWW",
                password: "Accenture$1",
                username: "WWW_Anonymous",
            },
            source_system: 1,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginResponse {
    signature: String,
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct LoginRequestInternal<'a> {
    domain: &'a str,
    password: &'a str,
    username: &'a str,
}
