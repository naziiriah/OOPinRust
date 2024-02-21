use oop::Post;

 fn main() {
      let mut post  = Post::new();

    post.add_text("i am eating");

    let ReviewedPost = post.review_post();

    let this_post =  ReviewedPost.approve();

    println!("{}",this_post.content());
    
}
