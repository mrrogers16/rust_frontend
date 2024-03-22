use super::schema::{users, posts};
use diesel::{Queryable, Insertable};
use serde::{ Deserialize, Serialize};

#[derive(Debug, Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub link: String,
    pub author: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub link: Option<String>,
    pub author: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String, 
    pub password: String,
}

impl NewPost {
    pub fn from_post_form(title: String, link: String, uid: i32) -> Self {
        NewPost {
            title: title,
            link: link,
            author: uid,
            created_at: chrono::Local::now().naive_utc(),
        }
    }
}