use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug)]
#[diesel(table_name = notification_recipients)]
pub struct NotificationRecipient {
    pub id: i32,
    pub notification_id: i32,
    pub user_id: i32,
    pub delivery_status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = super::schema::notification_templates)]
pub struct NotificationTemplate {
    pub id: i32,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::notification_templates)]
pub struct NewNotificationTemplate {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = notifications)]
pub struct Notification {
    pub id: i32,
    pub template_id: i32,
    pub status: String,
    pub sent_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = super::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
