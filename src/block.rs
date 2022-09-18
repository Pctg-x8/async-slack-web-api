use std::borrow::Cow;

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block<'s> {
    Divider {
        #[serde(skip_serializing_if = "Option::is_none")]
        block_id: Option<Cow<'s, str>>,
    },
    Header {
        /// Note: only available PlainText object
        text: TextObject<'s>,
        #[serde(skip_serializing_if = "Option::is_none")]
        block_id: Option<Cow<'s, str>>,
    },
    Section {
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<TextObject<'s>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        block_id: Option<Cow<'s, str>>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        fields: Vec<TextObject<'s>>,
    },
}

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextObject<'s> {
    #[serde(rename = "mrkdwn")]
    Markdown {
        text: Cow<'s, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        verbatim: Option<bool>,
    },
    PlainText {
        text: Cow<'s, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        emoji: Option<bool>,
    },
}
