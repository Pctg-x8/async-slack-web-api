use std::collections::HashMap;

pub struct PostAPI<Req, Resp>(&'static str, std::marker::PhantomData<(Req, Resp)>);
impl<Req: serde::Serialize, Resp: serde::de::DeserializeOwned> PostAPI<Req, Resp> {
    pub const fn new(ep: &'static str) -> Self {
        Self(ep, std::marker::PhantomData)
    }

    pub async fn send(self, token: &str, req: Req) -> reqwest::Result<Resp> {
        reqwest::Client::new()
            .post(format!("https://slack.com/api/{}", self.0))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&req)
            .send()
            .await?
            .json()
            .await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct GenericSlackError {
    pub error: String,
    #[serde(flatten)]
    pub extras: HashMap<String, serde_json::Value>,
}
impl std::fmt::Display for GenericSlackError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "SlackError: {} extras={}",
            self.error,
            serde_json::to_string(&self.extras)
                .unwrap_or_else(|_| String::from("<ERROR FORMATTING>"))
        )
    }
}
impl std::error::Error for GenericSlackError {}

pub struct GenericSlackResult<T>(Result<T, GenericSlackError>);
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for GenericSlackResult<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut obj = serde_json::Map::deserialize(deserializer)?;

        let ok = obj
            .remove("ok")
            .ok_or_else(|| serde::de::Error::missing_field("ok"))?;
        let ok: bool = bool::deserialize(ok).map_err(serde::de::Error::custom)?;
        let rest = serde_json::Value::Object(obj);

        if ok {
            Ok(Self(Ok(
                T::deserialize(rest).map_err(serde::de::Error::custom)?
            )))
        } else {
            Ok(Self(Err(
                GenericSlackError::deserialize(rest).map_err(serde::de::Error::custom)?
            )))
        }
    }
}
impl<T> GenericSlackResult<T> {
    #[inline]
    pub fn into_result(self) -> Result<T, GenericSlackError> {
        self.0
    }
}
impl<T> From<GenericSlackResult<T>> for Result<T, GenericSlackError> {
    #[inline]
    fn from(r: GenericSlackResult<T>) -> Self {
        r.into_result()
    }
}

pub mod api {
    pub mod chat {
        pub mod post_message {
            #[derive(serde::Serialize)]
            pub struct Request<'s> {
                pub channel: &'s str,
                pub text: Option<&'s str>,
                pub as_user: Option<bool>,
                pub icon_emoji: Option<&'s str>,
                pub icon_url: Option<&'s str>,
                pub username: Option<&'s str>,
                pub thread_ts: Option<&'s str>,
            }
            impl Default for Request<'_> {
                fn default() -> Self {
                    Self {
                        channel: "",
                        text: None,
                        as_user: None,
                        icon_emoji: None,
                        icon_url: None,
                        username: None,
                        thread_ts: None,
                    }
                }
            }

            #[derive(serde::Deserialize)]
            pub struct Response {
                pub channel: String,
                pub ts: String,
            }

            pub const EP: crate::PostAPI<Request, crate::GenericSlackResult<Response>> =
                crate::PostAPI::new("chat.postMessage");
        }

        pub use self::post_message::EP as PostMessage;
    }
}
