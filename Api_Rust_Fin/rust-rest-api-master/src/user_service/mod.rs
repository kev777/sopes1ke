use std::hash;

use bson::ordered::OrderedDocument;
use bson::{doc, Bson, Document};
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, results::InsertOneResult, Collection};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub nombre: String,
    pub comentario: String,
    pub fecha: String,
    pub hashtags: Vec<String>,
    pub upvotes: u64,
    pub downvotes: u64,
}

pub struct Tweet {
    pub nombre: String,
    pub comentario: String,
    pub fecha: String,
    pub hashtags: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub api: String,
}

#[derive(Clone)]
pub struct UserService {
    collection: Collection,
}

///
/// Build user from inputs
/// # Example :
///
/// ```
/// let user = build_user(
///     "hela",
///     "ben khalfallah",
///     "hela@hotmail.fr",
///     "helabenkhalfallah",
///     "azerty"
/// )
/// println!("user  = {:?}", user);
/// ```
fn build_user(
    nombre: String,
    comentario: String,
    fecha: String,
    hashtags: Vec<String>,
    upvotes: u64,
    downvotes: u64,
) -> User {
    User {
        nombre,
        comentario,
        fecha,
        hashtags,
        upvotes,
        downvotes,
    }
}

fn build_tweet(
    nombre: String,
    comentario: String,
    fecha: String,
    hashtags: String,
    upvotes: u64,
    downvotes: u64,
    api: String,
) -> Tweet {
    Tweet {
        nombre,
        comentario,
        fecha,
        hashtags,
        upvotes,
        downvotes,
        api,
    }
}

///
/// Transform mongo db document to User
/// # Example :
///
/// ```
/// let cursor = self.collection.find(None, None).unwrap();
/// for result in cursor {
///    if let Ok(item) = result {
///      data.push(user_from_document(item))
///    }
/// }
/// ```
fn user_from_document(document: Document) -> User {
    let mut _nombre = "".to_string();
    let mut _comentario = "".to_string();
    let mut _fecha = "".to_string();
    let mut _hashtags = "".to_string();
    let mut _upvotes = "".to_string();
    let mut _downvotes = "".to_string();
    let mut _hashtagsv= ["some".to_string(), "long".to_string(), "list".to_string(), "of".to_string(), "strings".to_string()].to_vec();
    if let Some(&Bson::String(ref nombre)) = document.get("nombre") {
        _nombre = nombre.to_string();
    }
    if let Some(&Bson::String(ref comentario)) = document.get("comentario") {
        _comentario = comentario.to_string();
    }
    if let Some(&Bson::String(ref fecha)) = document.get("fecha") {
        _fecha = fecha.to_string();
    }
    if let Some(&Bson::String(ref hashtags)) = document.get("hashtags") {
        _hashtags = hashtags.to_string();
    }
    if let Some(&Bson::I64(ref upvotes)) = document.get("upvotes") {
        _upvotes = upvotes.to_string();
    }
    if let Some(&Bson::I64(ref downvotes)) = document.get("downvotes") {
        _downvotes = downvotes.to_string();
    }

    build_user(_nombre, _comentario, _fecha, _hashtagsv, 0, 0)
}

/// Transform user to mongo db document
fn user_to_document(user: &User) -> Document {
    let User {
        nombre,
        comentario,
        fecha,
        hashtags,
        upvotes,
        downvotes,
    } = user;
    doc! {
        "nombre": nombre,
        "comentario": comentario,
        "fecha": fecha,
        "hashtags": hashtags,
        "upvotes": upvotes,
        "downvotes" : downvotes,
    }
}

fn tweet_to_document(tweet: &Tweet) -> Document {
    let Tweet {
        nombre,
        comentario,
        fecha,
        hashtags,
        upvotes,
        downvotes,
        api,
    } = tweet;
    doc! {
        "nombre": nombre,
        "comentario": comentario,
        "fecha": fecha,
        "hashtags": hashtags,
        "upvotes": upvotes,
        "downvotes" : downvotes,
        "api":api,
    }
}

impl UserService {
    pub fn new(collection: Collection) -> UserService {
        UserService { collection }
    }

    /// Insert user in mongo db (user)
    pub fn create(&self, user: &User) -> Result<InsertOneResult, Error> {
        //self.collection.insert_one(user_to_document(user), None)
        let mut hash_final =  String::from("");
        let mut contador = 0;
        let a="rust";
        for x in &user.hashtags {
            if contador == 0{
                hash_final = x.to_string();
            }
            else{
                hash_final = format!("{}, {}", hash_final, x);
            }
            contador = contador + 1;
        }
        /*let tweet = build_tweet(
                 t.nombre,
                 t.comentario,
                 t.fecha,
                 hash_final,
                 user.upvotes,
                 user.downvotes,
                 a.to_string()
            );*/
        let tweet = Tweet {
            nombre: user.nombre.clone(),
            comentario: user.comentario.clone(),
            fecha: user.fecha.clone(),
            hashtags: hash_final,
            upvotes: user.upvotes.clone(),
            downvotes: user.downvotes.clone(),
            api: a.to_string()
        };    

        self.collection.insert_one(tweet_to_document(&tweet), None)
    }

    /// Update existing user in mongo db (email)
    pub fn update(&self, user: &User) -> Result<UpdateResult, Error> {
        let User {
            nombre: _nombre,
            comentario: _comentario,
            fecha: _fecha,
            hashtags: _hashtags,
            upvotes: _upvotes,
            downvotes: _downvotes,
        } = user;
        self.collection
            .update_one(doc! { "nombre": _nombre}, user_to_document(user), None)
    }

    /// Delete existing user in mongo db (email)
    pub fn delete(&self, nombre: &String) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "nombre": nombre}, None)
    }

    /// get all users
    pub fn get(&self) -> Result<Vec<User>, Error> {
        let cursor = self.collection.find(None, None).unwrap();
        let mut data: Vec<User> = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                data.push(user_from_document(item))
            }
        }

        Ok(data)
    }

    /// Retrieve user by (email)
    pub fn get_user_email(&self, nombre: &String) -> Result<Option<OrderedDocument>, Error> {
        self.collection.find_one(doc! { "nombre": nombre}, None)
    }
}
