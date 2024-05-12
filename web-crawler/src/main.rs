use html5ever::tokenizer::{BufferQueue, Tag, TagToken, Token, TokenSink, TokenSinkResult};

#[derive(Debug)]
struct LinkQueue {
    links: Vec<String>
}

impl TokenSink for &mut LinkQueue {
    type Handle = ();

    // <a href="link">some text</a>
    fn process_token(&mut self,
                     token: Token,
                     line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
         _ => println!("ainst soecial"),
        }
        TokenSinkResult::Continue
    }
}
fn main() {

}