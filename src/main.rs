use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article{
        article: String::from("How to work with JSON in Rust"),
        author: String::from("RF"),
        paragraph: vec![
            Paragraph {
                name: String::from("First sentence."),
            },
            Paragraph {
                name: String::from("Middle sentence."),
            },
            Paragraph {
                name: String::from("Last sentence.")
            }
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("The JSON is {}", json)
}
