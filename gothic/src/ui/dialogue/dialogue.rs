pub struct Dialogue {
    pub sentences: &'static [Sentence]
}

pub struct Sentence {
    pub actor: Option<&'static str>,
    pub message: &'static str,
}

#[macro_export]
macro_rules! sentence {
    ($actor:ident: $message:literal) => {
        Sentence {
            actor: $actor,
            message: $message,
        }
    };
}