fn main() {
    let mut post = Post::new();
    let txt = "some text";
    post.add_text(txt);
    assert_eq!("", post.content()); // not available now
    post.request_review();
    assert_eq!("", post.content()); // not available now
    post.approve();
    assert_eq!(txt, post.content()); // not available now
    println!("OK!");
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // return draft
    }
}

struct PendingReview;
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {}) // published
    }
}

struct Published;
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // return Published
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        &_post.content // only here get text
    }
}

impl Post {
    fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    fn add_text(&mut self, txt: &str) {
        self.content.push_str(txt);
    }
    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
