#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub body_html: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn to_new_post(&self) -> NewPost {
        NewPost {
            title: self.title.clone(),
            body: self.body.clone(),
        }
    }
}

impl NewPost {
    pub fn new() -> NewPost {
        NewPost {
            title: "".to_string(),
            body: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Title {
    pub title: String,
    pub post_id: Option<i32>,
    pub children: Vec<Title>,
}