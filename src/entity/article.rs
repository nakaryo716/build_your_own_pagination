use serde::{Serialize, ser::SerializeStruct};
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, Clone, FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Article", 4)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("body", &self.body)?;
        s.serialize_field("created_at", &self.created_at.to_string())?;
        s.end()
    }
}
