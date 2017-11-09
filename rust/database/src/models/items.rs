use schema::items;
use chrono::naive::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Item {
    pub owner: String,
    pub title: String,
    pub borrower: Option<String>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub owner: &'a str,
    pub title: &'a str,
    pub borrower: Option<String>,
    pub due_date: Option<NaiveDateTime>,
}

