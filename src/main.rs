use oop::Post;

 fn main() {
      let mut post  = Post::new();

    post.add_text("i am eating");


    let ReviewedPost = post.reject_post();

    let this_post =  ReviewedPost.review_post();

    let reviewd_reviewed = this_post.approved_for_final();
  
    let final_reviewed = reviewd_reviewed.approve();
    assert_eq!("i am eating", final_reviewed.content())
    
}
