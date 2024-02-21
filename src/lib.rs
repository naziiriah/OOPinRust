pub struct Post {
    content: String,
}


impl Post {
    pub fn new() -> DraftPost{
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str{  
        &self.content
    }
}

pub struct DraftPost {
     content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, txt:&str) {
        self.content.push_str(txt);
    }

    pub fn review_post(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }

    pub fn reject_post(self)-> DraftPost{
        DraftPost{
            content: self.content
        }
    }
}

pub struct SecondReview {
    content: String,
}
pub struct PendingReviewPost {
     content: String,
}

impl PendingReviewPost {
    pub fn approved_for_final(self) -> SecondReview{
        SecondReview {
            content: self.content,
        }
    }
}

impl SecondReview {
    pub fn approve(self) -> Post{
        Post {
            content: self.content,
        }
    }   
}