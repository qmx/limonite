use pulldown_cmark::{html, Parser};
use std::io;

#[derive(Debug)]
enum DocumentType {
    Markdown
}

#[derive(Debug)]
struct Document {
    path: String,
    content: String,
    kind: DocumentType
}

impl Document {
    fn render(self) -> Result<String, io::Error> {
        let mut output = String::new();
        let p = Parser::new(&self.content);
        html::push_html(&mut output, p);
        Ok(output)
    }
}

#[test]
fn renders_markdown() {
    let doc = Document {
        path: "index".to_owned(),
        content: "# Hello".to_owned(),
        kind: DocumentType::Markdown
    };
    assert_eq!("<h1>Hello</h1>\n".to_owned(), doc.render().unwrap());
}
