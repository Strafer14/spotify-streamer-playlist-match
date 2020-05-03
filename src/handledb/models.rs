use super::schema::streamer_records;
use serde::Deserialize;

#[derive(Queryable, Insertable, Deserialize, Debug)]
#[table_name="streamer_records"]
pub struct StreamerRecord {
    pub(crate) id: i32,
    pub(crate) viewers: i32,
    pub(crate) followersgained: i32,
    pub(crate) rownum: i32,
    pub(crate) logo: String,
    pub(crate) twitchurl: String,
    pub(crate) url: String,
    pub(crate) displayname: String,
}