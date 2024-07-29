use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{
        chrono::{DateTime, Local},
        Uuid,
    },
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Angebot {
    #[serde(default)]
    pub angebot_id: Uuid,
    pub angebot_name: String,
    pub beschreibung: Option<String>,
    pub organisation_id: Uuid,
    pub created: DateTime<Local>,
    pub last_modified: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organisation {
    #[serde(default)]
    pub organisation_id: Uuid,
    pub organisation_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ansprechpartner {
    pub ansprech_partner_id: Uuid,
    pub nach_name: String,
    pub vor_name: String,
}
