use html5ever::tokenizer::{BufferQueue, Tag, TagKind, TagToken, Token, TokenSink, TokenSinkResult};

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
            TagToken(
            ref tag@ Tag{
                kind: TagKind::StartTag,
                ..
            },
            ) => {
                if tag.name.as_ref() == "a" {
                    for attribute in tag.attrs.iter() {
                        if attribute.name.local.as_ref() == "href" {
                            let url_str:&[u8] = attribute.value.borrow();
                            self.links.push(String::from_utf8_lossy(url_str).into_owned());

                        }
                    }

                }
            }
         _ => println!("ainst soecial"),
        }
        TokenSinkResult::Continue
    }
}
fn main() {

}