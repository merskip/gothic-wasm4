pub struct Dialogue {
    pub sentences: &'static [Sentence]
}

pub struct Sentence {
    pub message: &'static str,
    pub actor: Option<&'static str>,
}
