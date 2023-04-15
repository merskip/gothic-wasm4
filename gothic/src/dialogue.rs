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
    ($ident:ident $actor:ident: $message:literal finishes dialogue) => {
        const $ident: $crate::dialogue::DialogueItem = $crate::dialogue::DialogueItem::Sentence($crate::dialogue::Sentence {
            actor: $actor,
            message: $message,
            next_item: None,
        });
    };
    ($ident:ident $actor:ident: $message:literal next: $next:ident) => {
        const $ident: $crate::dialogue::DialogueItem = $crate::dialogue::DialogueItem::Sentence($crate::dialogue::Sentence {
            actor: $actor,
            message: $message,
            next_item: Some(&$next),
        });
    };
}