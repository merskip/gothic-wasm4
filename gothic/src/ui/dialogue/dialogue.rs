pub struct Dialogue {
    pub items: &'static [DialogueItem],
}

pub enum DialogueItem {
    Sentence(Sentence),
    PlayerChoice {
        choices: &'static [PlayerChoice],
    },
}

pub struct Sentence {
    pub actor: Option<&'static str>,
    pub message: &'static str,
}

pub struct PlayerChoice {
    pub choice: &'static str,
    pub next_item: &'static DialogueItem,
}

#[macro_export]
macro_rules! sentence {
    ($actor:ident: $message:literal) => {
        DialogueItem::Sentence(Sentence {
            actor: $actor,
            message: $message,
        })
    };
}