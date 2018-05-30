use pandoc;
use std::io;

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        body_html -> Text,
        published -> Bool,
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub body_html: String,
    pub published: bool,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub body_html: String,
    pub published: bool,
}

impl NewPost {
    pub fn generate_body_html(&mut self) -> Result<String, pandoc::PandocError> {
        let mut pandoc = pandoc::new();
        pandoc.set_input(pandoc::InputKind::Pipe(self.body.to_string()));
        pandoc.set_input_format(pandoc::InputFormat::MarkdownGithub, vec![]);
        pandoc.set_output(pandoc::OutputKind::Pipe);
        pandoc.set_output_format(pandoc::OutputFormat::Html5, vec![]);
        match pandoc.execute() {
            Ok(pandoc::PandocOutput::ToBuffer(string)) => {
                self.body_html = string.to_string();
                Ok(string.to_string())
            },
            Err(err) => Err(err),
            _ => Err(pandoc::PandocError::from(io::Error::new(io::ErrorKind::Other, "Unknown error")))
        }
    }
}
