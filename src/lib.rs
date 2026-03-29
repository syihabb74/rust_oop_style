pub mod rust_style {
    

    pub struct Post {
        content : String,
    }

    pub struct DraftPost {
        content : String
    }

    pub struct PendingPost {
        content : String,
    }

    impl Post {

        pub fn new() -> DraftPost {
            DraftPost { content: String::new() }
        }

        pub fn show_content(&self) -> &str {
            &self.content
        }

    }

    impl DraftPost {

        // this is consume this struct itself to produce
        // another data types it self and avoid waste of memory allocation
        pub fn pending_review(self) -> PendingPost {
            PendingPost { content: self.content }
        }

        pub fn add_content(&mut self, content : &str) {
            self.content.push_str(content);
        }

    }

    impl PendingPost {

        // this is consume this struct itself to produce
        // another data types it self and avoid waste of memory allocation
        pub fn approve(self) -> Post {
            Post { content: self.content }
        }

    }

    

}