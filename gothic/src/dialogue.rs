pub struct Dialogue {
    pub start_item: &'static DialogueItem,
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
    pub next_item: Option<&'static DialogueItem>,
}

pub struct PlayerChoice {
    pub choice: &'static str,
    pub next_item: Option<&'static DialogueItem>,
}

#[macro_export]
macro_rules! sentence {
    ($actor:ident: $message:literal finishes dialogue) => {
        $crate::dialogue::DialogueItem::Sentence($crate::dialogue::Sentence {
            actor: $actor,
            message: $message,
            next_item: None,
        })
    };
    ($actor:ident: $message:literal next: $next:ident) => {
        $crate::dialogue::DialogueItem::Sentence($crate::dialogue::Sentence {
            actor: $actor,
            message: $message,
            next_item: Some(&$next),
        })
    };
}