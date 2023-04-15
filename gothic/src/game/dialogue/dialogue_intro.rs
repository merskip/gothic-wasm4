use crate::game::dialogue::*;
use crate::sentence;
use crate::ui::dialogue::dialogue::{Dialogue, DialogueItem, Sentence};

/*
Sędzia: W imieniu jego wysokości, króla Rhobara II, pana Varantu, skazuje tego więźnia na...
Mag ognia: Stać! Skazańcze, mam dla Ciebie propozycję... Ten list musi dotrzeć do arcymistrza kręgu magów ognia.
Bezimienny: Marnujesz czas
Mag ognia: Sam będziesz mógł wybrać sobie nagrodę. Magowie dadzą Ci wszystko czego zażądasz.
Bezimienny: Niech będzie, zaniosę wasz cenny list. Pod jednym warunkiem. Oszczędźcie mi reszty tej paplaniny.
Sędzia: JAK ŚMIE...
Mag ognia: Milcz!
Mag ognia: Dobra, zrzucajcie go.
Bezimienny: Aaaaa!
<plusk>
Bullit: Witamy w kolonii!
<jeb>
Diego: Dość tego! Zostawcie go! A teraz precz!
Diego: No już, wstawaj.
 */

pub const DIALOGUE_INTRO: &Dialogue = &Dialogue {
    // sentence!(JUDGE_ACTOR: "W imieniu jego wysokosci, krola Rhobara II, pana Varantu, skazuje tego wieznia na..."),
    // sentence!(FIRE_MAGE_ACTOR: "Stac! Skazancze, mam dla Ciebie propozycje... Ten list musi dotrzec do arcymistrza kregu magow ognia."),
    // sentence!(PLAYER_ACTOR: "Marnujesz czas."),
    // sentence!(FIRE_MAGE_ACTOR: "Sam bedziesz mogl wybrac sobie nagrode. Magowie dadza Ci wszystko czego zazadasz."),
    // sentence!(PLAYER_ACTOR: "Niech bedzie, zaniose wasz cenny list. Pod jednym warunkiem."),
    // sentence!(PLAYER_ACTOR: "Oszczedzcie mi reszty tej paplaniny."),
    // sentence!(JUDGE_ACTOR: "JAK SMIE..."),
    // sentence!(FIRE_MAGE_ACTOR: "Milcz!"),
    // sentence!(FIRE_MAGE_ACTOR: "Dobra, zrzucajcie go."),
    // sentence!(PLAYER_ACTOR: "Aaaaa!"),
    // // <plusk>
    // sentence!(BULLIT_ACTOR: "Witamy w kolonii!"),
    // // <jeb>
    // sentence!(DIEGO_ACTOR: "Dosc tego! Zostawcie go! A teraz precz!"),
    // sentence!(DIEGO_ACTOR: "No juz, wstawaj."),

    start_item: &DialogueItem::Sentence(Sentence {
        actor: JUDGE_ACTOR,
        message: "W imieniu jego wysokosci, krola Rhobara II, pana Varantu, skazuje tego wieznia na...",
        next_item: Some(&DialogueItem::Sentence(Sentence {
            actor: FIRE_MAGE_ACTOR,
            message: "Stac! Skazancze, mam dla Ciebie propozycje... Ten list musi dotrzec do arcymistrza kregu magow ognia.",
            next_item: None,
        })),
    }),
};
