// an experiment to compare the book implementation with
// with a simple implementation using an enum
// the key difference is the state's behaviour 
// and transitions are split between multiple functions

pub struct Post {
    state: State,
    content: String,
}

#[derive(PartialEq)]
enum State {
    Draft,
    PendingReview,
    Published
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: State::Draft,
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match self.state {
            State::Draft => "",
            State::PendingReview => "",
            State::Published => &self.content    
        }
    }

    // - state transformations f

    pub fn request_review(&mut self) {
        if self.state == State::Draft {
            self.state = State::PendingReview
        }
    }

    pub fn approve(&mut self) {
        if self.state == State::PendingReview {
            self.state = State::Published
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
