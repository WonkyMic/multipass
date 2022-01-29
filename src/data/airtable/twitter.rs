use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    pub id: String,
    pub name: String,
    pub username: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRecord {
    pub fields: Fields
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct CreateRecords { 
    pub records: Vec<NewRecord>
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct Records { 
    pub records: Vec<Record>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub fields: Fields 
}
