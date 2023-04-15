use crate::dialogue::Dialogue;
use crate::game::dialogue::*;
use crate::sentence;

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

pub const DIALOGUE_INTRO: Dialogue = Dialogue {
    start_item: &SENTENCE_1,
};

sentence!(SENTENCE_1
    JUDGE_ACTOR: "W imieniu jego wysokosci, krola Rhobara II, pana Varantu, skazuje tego wieznia na..."
    next: SENTENCE_2
);
sentence!(SENTENCE_2
    FIRE_MAGE_ACTOR: "Stac! Skazancze, mam dla Ciebie propozycje... Ten list musi dotrzec do arcymistrza kregu magow ognia."
    next: SENTENCE_3
);
sentence!(SENTENCE_3
    PLAYER_ACTOR: "Marnujesz czas."
    next: SENTENCE_4
);
sentence!(SENTENCE_4
    FIRE_MAGE_ACTOR: "Sam bedziesz mogl wybrac sobie nagrode. Magowie dadza Ci wszystko czego zazadasz."
    next: SENTENCE_5
);
sentence!(SENTENCE_5
    PLAYER_ACTOR: "Niech bedzie, zaniose wasz cenny list. Pod jednym warunkiem."
    next: SENTENCE_6
);
sentence!(SENTENCE_6
    PLAYER_ACTOR: "Oszczedzcie mi reszty tej paplaniny."
    next: SENTENCE_7
);
sentence!(SENTENCE_7
    JUDGE_ACTOR: "JAK SMIE..."
    next: SENTENCE_8
);
sentence!(SENTENCE_8
    FIRE_MAGE_ACTOR: "Milcz!"
    next: SENTENCE_9
);
sentence!(SENTENCE_9
    FIRE_MAGE_ACTOR: "Dobra, zrzucajcie go."
    next: SENTENCE_10
);
sentence!(SENTENCE_10
    PLAYER_ACTOR: "Aaaaa!"
    next: SENTENCE_11
);
// <plusk>
sentence!(SENTENCE_11
    BULLIT_ACTOR: "Witamy w kolonii!"
    next: SENTENCE_12
);
// <jeb>
sentence!(SENTENCE_12
    DIEGO_ACTOR: "Dosc tego! Zostawcie go! A teraz precz!"
    next: SENTENCE_13
);
sentence!(SENTENCE_13
    DIEGO_ACTOR: "No juz, wstawaj."
    finishes dialogue
);
