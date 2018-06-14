use models::post::Post;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Title {
    pub title: String,
    pub post_id: Option<i32>,
    pub children: Vec<Title>,
}

pub fn gen_titles(posts: Vec<Post>) -> Vec<Title> {
    if posts.len() == 0 {
        return vec![];
    }
    let mut titles: Vec<Title> = vec![];
    for post in &posts {
        let mut parts: Vec<String> = post.title.split('/').map(|t| t.to_string()).collect();
        let (first, tail) = parts.split_at(1);
        titles = gen_titles_inner(
            first.first().unwrap().to_string(),
            tail.to_vec(),
            titles.clone(),
            post,
        );
    }
    titles
}

fn gen_titles_inner(
    part: String,
    parts: Vec<String>,
    mut titles: Vec<Title>,
    post: &Post,
) -> Vec<Title> {
    let mut inserted = false;
    for mut title in &mut titles {
        if title.title == part {
            if parts.len() == 0 {
                title.children = vec![];
                title.post_id = Some(post.id);
            } else {
                let (first, tail) = parts.split_at(1);
                title.children = gen_titles_inner(
                    first.first().unwrap().to_string(),
                    tail.to_vec(),
                    title.children.clone(),
                    post,
                );
            }
            inserted = true;
        }
    }
    if !inserted {
        let mut title = Title {
            title: part,
            post_id: None,
            children: vec![],
        };
        if parts.len() == 0 {
            title.children = vec![];
            title.post_id = Some(post.id);
        } else {
            let (first, tail) = parts.split_at(1);
            title.children = gen_titles_inner(
                first.first().unwrap().to_string(),
                tail.to_vec(),
                title.children.clone(),
                post,
            );
        }
        titles.push(title);
    }
    titles
}
