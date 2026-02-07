use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::greetings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Greeting {
    pub id: i32,
    pub greeting: String,
    pub language: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::greetings)]
pub struct NewGreeting {
    pub greeting: String,
    pub language: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::access_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessToken {
    pub id: i32,
    pub token: String,
}
