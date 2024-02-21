pub struct  Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() ->  Post {
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text:&str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self){
        if let Some(state) = self.state.take()  {
            self.state = Some(state.request_review())            
        }
    }

    pub fn final_review(&mut self){
        if let Some(state) = self.state.take()  {
            self.state = Some(state.final_review())            
        }
    }

    pub fn approve(&mut self){
        if let Some(state) = self.state.take()  {
            self.state = Some(state.approve())            
        }
    }  

    pub fn reject(&mut self){
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject())            
        }
    }

}

trait State {
    fn request_review(self:Box<Self>) -> Box<dyn State>;
    fn final_review(self:Box<Self>) -> Box<dyn State>;
    fn approve(self:Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post:&'a Post)-> &'a str{
        ""
    }
    fn reject(self:Box<Self>) -> Box<dyn State>;
}


struct  Draft {}

struct PendingReview {}

struct FinalReview {}
struct Published{}


// two apprvls required before a post is published
// add text to a post when it is in draft

impl State for Draft {
    fn request_review(self:Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview{})
    }

    fn final_review(self:Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self:Box<Self>) -> Box<dyn State>{
        self
    }

    fn reject(self:Box<Self>) -> Box<dyn State> {
        self
    }

}

impl State for PendingReview {
    fn request_review(self:Box<Self>) -> Box<dyn State>{
        self
    }

    fn final_review(self:Box<Self>) -> Box<dyn State> {
        Box::new(FinalReview {})
    }

    fn approve(self:Box<Self>) -> Box<dyn State>{
        self
    }

    fn reject(self:Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for FinalReview {
    fn request_review(self:Box<Self>) -> Box<dyn State>{
        self
    }

    fn final_review(self:Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self:Box<Self>) -> Box<dyn State>{
        Box::new(Published {})
    }

    fn reject(self:Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }   
}

impl State for Published{
    fn request_review(self:Box<Self>) -> Box<dyn State>{
        self
    }

    fn final_review(self:Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self:Box<Self>) -> Box<dyn State>{
        self
    }

    fn content<'a>(&self, post:&'a Post)-> &'a str{
        &post.content
    }

    fn reject(self:Box<Self>) -> Box<dyn State> {
        self
    }
}
