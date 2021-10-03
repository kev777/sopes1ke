use chrono::prelude::*; //For date and time
use mysql::prelude::*;
use mysql::*;
use serde::{Serialize, Deserialize};
 

#[derive(Serialize, Deserialize)]
pub struct Tweet {
    pub id: u64,
    pub nombre: String,
    pub comentario: String,
    pub fecha: NaiveDate,
    pub hashtags: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub api: String,
}
 
pub fn today() -> NaiveDate {
    let l = Local::today();
 
    NaiveDate::from_ymd(l.year(), l.month(), l.day())
}

pub fn insert_tweet(
    conn: &mut PooledConn, 
    tweet: &Tweet) -> std::result::Result<u64, mysql::error::Error> {
 
    conn.exec_drop(
        "insert into tweet (nombre, comentario, fecha, hashtags, upvotes, downvotes, api)  
        values (:nombre, :comentario, :fecha, :hashtags, :upvotes, :downvotes, :api)",
        params! {
            "nombre" => &tweet.nombre,
            "comentario" => &tweet.comentario,
            "fecha" => today(),
            "hashtags" => &tweet.hashtags,
            "upvotes" => tweet.upvotes,
            "downvotes" => tweet.downvotes,
            "api" => &tweet.api,
        },
    )
    .and_then(|_| Ok(conn.last_insert_id()))
}

pub fn find_tweet_by_id(
    conn: &mut PooledConn,
    tweet_id: u64,
) -> std::result::Result<Option<Tweet>, mysql::error::Error> {
    let row = conn.exec_first(
        "select id, nombre, comentario, fecha, hashtags, upvotes, downvotes, api from tweet where id=:tweet_id",
        params! {
            "tweet_id" => tweet_id
        },
    )?;
 
    Ok(row.map(|(id, nombre, comentario, fecha, hashtags, upvotes, downvotes, api)| Tweet {
        id: id,
        nombre: nombre,
        comentario: comentario,
        fecha: fecha,
        hashtags: hashtags,
        upvotes: upvotes,
        downvotes: downvotes,
        api: api
    }))
}

