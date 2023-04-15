use wasm4::audio::{ADSRDuration, Audio, Channel, Duration, DutyCycle, Flags, Frequency, Pan, Volume};
use wasm4::println;

use crate::dialogue::{Dialogue, DialogueItem, Script};
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

pub static DIALOGUE_INTRO: Dialogue = Dialogue {
    start_item: &SENTENCE_1,
};

static SENTENCE_1: DialogueItem = sentence!(
    JUDGE_ACTOR: "W imieniu jego wysokosci, krola Rhobara II, pana Varantu, skazuje tego wieznia na..."
    next: SENTENCE_2
);
static SENTENCE_2: DialogueItem = sentence!(
    FIRE_MAGE_ACTOR: "Stac! Skazancze, mam dla Ciebie propozycje... Ten list musi dotrzec do arcymistrza kregu magow ognia."
    next: SENTENCE_3
);
static SENTENCE_3: DialogueItem = sentence!(
    PLAYER_ACTOR: "Marnujesz czas."
    next: SENTENCE_4
);
static SENTENCE_4: DialogueItem = sentence!(
    FIRE_MAGE_ACTOR: "Sam bedziesz mogl wybrac sobie nagrode. Magowie dadza Ci wszystko czego zazadasz."
    next: SENTENCE_5
);
static SENTENCE_5: DialogueItem = sentence!(
    PLAYER_ACTOR: "Niech bedzie, zaniose wasz cenny list. Pod jednym warunkiem."
    next: SENTENCE_6
);
static SENTENCE_6: DialogueItem = sentence!(
    PLAYER_ACTOR: "Oszczedzcie mi reszty tej paplaniny."
    next: SENTENCE_7
);
static SENTENCE_7: DialogueItem = sentence!(
    JUDGE_ACTOR: "JAK SMIE..."
    next: SENTENCE_8
);
static SENTENCE_8: DialogueItem = sentence!(
    FIRE_MAGE_ACTOR: "Milcz!"
    next: SENTENCE_9
);
static SENTENCE_9: DialogueItem = sentence!(
    FIRE_MAGE_ACTOR: "Dobra, zrzucajcie go."
    next: SENTENCE_10
);
static SENTENCE_10: DialogueItem = sentence!(
    PLAYER_ACTOR: "Aaaaa!"
    next: SENTENCE_11
);
// <plusk>
static SENTENCE_11: DialogueItem = sentence!(
    BULLIT_ACTOR: "Witamy w kolonii!"
    next: SCRIPT_HIT_SOUND
);
static SCRIPT_HIT_SOUND: DialogueItem = DialogueItem::Script(Script {
    update: |_context| {
        Audio::shared().tone(
            Frequency::linear(380, 220),
            ADSRDuration::new(
                Duration::from_frames(0),
                Duration::from_frames(0),
                Duration::from_frames(14),
                Duration::from_frames(10),
            ),
            Volume::constant(100),
            Flags::new(Channel::Noise, DutyCycle::OneHalf, Pan::default()),
        );
        true
    },
    render: |_context| {},
    next_item: Some(&SENTENCE_12),
});
static SENTENCE_12: DialogueItem = sentence!(
    DIEGO_ACTOR: "Dosc tego! Zostawcie go! A teraz precz!"
    next: SENTENCE_13
);
static SENTENCE_13: DialogueItem = sentence!(
    DIEGO_ACTOR: "No juz, wstawaj."
    finishes dialogue
);
