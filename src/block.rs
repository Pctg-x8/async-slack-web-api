#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block<'s> {
    Divider {
        #[serde(skip_serializing_if = "Option::is_none")]
        block_id: Option<&'s str>,
    },
    Header {
        /// Note: only available PlainText object
        text: TextObject<'s>,
        #[serde(skip_serializing_if = "Option::is_none")]
        block_id: Option<&'s str>,
    },
}

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextObject<'s> {
    #[serde(rename = "mrkdwn")]
    Markdown {
        text: &'s str,
        #[serde(skip_serializing_if = "Option::is_none")]
        verbatim: Option<bool>,
    },
    PlainText {
        text: &'s str,
        #[serde(skip_serializing_if = "Option::is_none")]
        emoji: Option<bool>,
    },
}
