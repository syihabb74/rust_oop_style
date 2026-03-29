use rust_oop_style::rust_style::Post;

fn main() {

    // usage example
    let mut post = Post::new();
    post.add_content("Hello rust");
    let post = post.pending_review();
    let post = post.approve();
    assert_eq!("Hello rust", post.show_content());
    println!("{}", post.show_content())
}
