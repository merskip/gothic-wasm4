use crate::dialogue::{Dialogue, DialogueItem, PlayerChoice};
use crate::game::dialogue::*;
use crate::sentence;

/*
Diego: Nazywam się Diego.
Bezimienny: Jestem...
Diego: Nie interesuje mnie kim jesteś. Jesteś tu nowy - a do mnie należy dbanie o nowych. Na razie to tyle...
Diego: Jeśli chcesz jeszcze trochę pożyć, słuchaj się mnie, ale oczywiście nie będę Ci przeszkadzał w ewentualnej próbie samobójstwa. To jak będzie?

A) Bezimienny:

1. Dobra, co powinienem wiedzieć o tym miejscu?
1a. Jak dostanę się do Starego Obozu?
1b. Gdzie mogę znaleźć oręż?
2. Dlaczego mi pomogłeś?
2a. A gdzie jest ten Bullit?
3. Mam list do przywódcy Magów Ognia
3a. Kim jest Gomez?
3b. Założmy, że chciałbym dołączyć do jego ludzi. Co powinienem zrobić?
4. KONIEC

A.1
Bezimienny: Dobra, co powinienem wiedzieć o tym miejscu?
Diego: Nazywamy je kolonią. Wiesz już, że wydobywamy rudę dla króla.
Diego: Cóż, w każdym razie tak robią ludzie ze Starego Obozu.
Diego: Wewnątrz Bariery powstały trzy obozy. Największy i najstarszy jest tak zwany Stary Obóz.
$ umożliwia A.1a*; wraca do A.

A.1a
Bezimienny: Jak dostanę się do Starego Obozu?
Diego: Podążaj tą ścieżką. Stary Obóz to najbliższe, mniej więcej bezpieczne miejsce jakie spotkasz po drodze.
Diego: Pomiędzy obozami kręci się wiele groźnych zwierząt. Radziłbym ci postarać się o jakąś broń.
$ umożliwia A.1b*; wraca do A.

A.1b
Bezimienny: Gdzie mogę znaleźć oręż?
Diego: Rozejrzyj się trochę w pobliżu Opuszczonej Kopalni. Na pewno znajdziesz tam coś przydatnego.
Diego: Kopalnię nie trudno zznaleźć. Leży parę metrów w dół kanionu.
$ wraca do A

A.2
Bezimienny: Dlaczego mi pomogłeś?
Diego: Bo potrzebowałeś pomocy. Gdyby nie ja, Bullit i jego chłopcy mogliby cię wykończyć.
Diego: A ja jestem zbyt miły, żeby się temu spokojnie przyglądać. W końcu przebyłem całą tę drogę po to, by złożyć propozycję.
Bezimienny: Propozycję?
Diego: Tak. Po tym zajściu z Bullitiem i jego ludźmi powinieneś się domyślić, że przyda ci się ochrona.
Diego: Każdy, kto tu trafia ma wybór. W kolonii są trzy obozy i w końcu będziesz musiał do któregoś dołączyć.
Diego: Jestem tu, by udowodnić wszystkim nowym, że najlepszym miejscem dla nich będzie Stary Obóz.
$ wraca do A

A.2a
Bezmienny: A gdzie jest ten Bullit?
Diego: On i pozostali przenoszą towary z zewnątrz do Obozny. Tam go znajdziesz.
Diego: Ale jeśli chcesz z nim walczyć, radzę ci uważać. To doświadczony wojownik.
$ wraca do A

A.4
Bezimienny: Mam list do przywódcy Magów Ognia
Diego: Czyżby?
Bezimienny: Jakiś mag dał mi go zanim mnie tu wrzucono
Diego: Twoje szczęscie, że nie mogę się więcej pokazywać u magów. Ktoś inny mógłby ci poderżnąć gardło za taki list.
Diego: A to dlatego, że magowie hojnie opłacają swoich kurierów, a większość z tutejszych ludzi nic nie posiada.
Diego: Na twoim miejscu trzymałbym język za zebami aż do chwili, gdy spotkasz któregoś z magów. Chociaż wątpię, żeby ci sie udało.
Bezmienny: Dlaczego?
Diego: Magowie mieszkają w zamku, w Starym Obozie. Tylko ludzie Gomeza mają tam wstęp.
$ umożliwia A.4a i A.4b; wraca do A

A.4a
Bezmienny: Kim jest Gomez?
Diego: Gomez jest najpotężniejszym z Magnatów kierujących handlem rudą. To on rządzi w Starym Obozie i ma najwięcej do powiedzenia w całej kolonii.
$ wraca do A

A.4b
Bezimienny: Założmy, że chciałbym dołączyć do jego ludzi. Co powinienem zrobić?
Diego: Przy bramie do zamku znajdziesz człowieka imieniem Thorus. Powiedz mu, że Diego cię przysłal.
$ wraca do A

A.5
Bezimienny: Dzięki za pomoc
Diego: Spotkamy się w Starym Obozie
$ kończy rozmowe
$ początek rozdziału: Rozdział 1. Witamy w Kolonii!
 */

pub static DIALOGUE_DIEGO_FIRST_MEET: Dialogue = Dialogue {
    start_item: &SENTENCE_1,
};

static SENTENCE_1: DialogueItem = sentence!(
    DIEGO_ACTOR: "Nazywam sie Diego."
    next: SENTENCE_2
);
static SENTENCE_2: DialogueItem = sentence!(
    PLAYER_ACTOR: "Jestem..."
    next: SENTENCE_3
);
static SENTENCE_3: DialogueItem = sentence!(
    DIEGO_ACTOR: "Nie interesuje mnie kim jestes. Jestes tu nowy - a do mnie nalezy dbanie o nowych. Na razie to tyle..."
    next: SENTENCE_4
);
static SENTENCE_4: DialogueItem = sentence!(
    DIEGO_ACTOR: "Jesli chcesz jeszcze troche pozyc, sluchaj sie mnie, ale oczywiscie nie bede Ci przeszkadzal w ewentualnej probie samobojstwa. To jak bedzie?"
    next: PLAYER_CHOICE
);

static PLAYER_CHOICE: DialogueItem = DialogueItem::PlayerChoice {
    choices: &[
        unsafe { &about_area::CHOICE_A },
        unsafe { &about_area::CHOICE_B },
        unsafe { &about_area::CHOICE_C },
        unsafe { &why_help::CHOICE_A },
        unsafe { &why_help::CHOICE_B },
        unsafe { &letter_to_fire_mages::CHOICE_A },
        unsafe { &letter_to_fire_mages::CHOICE_B },
        unsafe { &letter_to_fire_mages::CHOICE_C },
        &PlayerChoice {
            choice: "KONIEC",
            enabled: true,
            next_item: None, // finishes dialogue
        }
    ]
};

mod about_area {
    use crate::dialogue::{DialogueItem, PlayerChoice, Script};
    use crate::game::dialogue::{DIEGO_ACTOR, PLAYER_ACTOR};
    use crate::sentence;

    use super::PLAYER_CHOICE;

    pub static mut CHOICE_A: PlayerChoice = PlayerChoice {
        choice: "Dobra, co powinienem wiedziec o tym miejscu?",
        enabled: true,
        next_item: Some(&CHOICE_A_SENTENCE_1),
    };
    static CHOICE_A_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "Dobra, co powinienem wiedziec o tym miejscu?"
        next: CHOICE_A_SENTENCE_2
    );
    static CHOICE_A_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "Nazywamy je kolonia. Wiesz juz, ze wydobywamy rude dla krola."
        next: CHOICE_A_SENTENCE_3
    );
    static CHOICE_A_SENTENCE_3: DialogueItem = sentence!(
        DIEGO_ACTOR: "Coz, w kazdym razie tak robia ludzie ze Starego Obozu."
        next: CHOICE_A_SENTENCE_4
    );
    static CHOICE_A_SENTENCE_4: DialogueItem = sentence!(
        DIEGO_ACTOR: "Wewnatrz Bariery powstaly trzy obozy. Najwiekszy i najstarszy jest tak zwany Stary Oboz."
        next: ENABLE_CHOICE_B
    );

    static ENABLE_CHOICE_B: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_A.enabled = false;
            CHOICE_B.enabled = true;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });

    pub static mut CHOICE_B: PlayerChoice = PlayerChoice {
        choice: "Jak dostane sie do Starego Obozu?",
        enabled: false,
        next_item: Some(&CHOICE_B_SENTENCE_1),
    };
    static CHOICE_B_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "Jak dostane sie do Starego Obozu?"
        next: CHOICE_B_SENTENCE_2
    );
    static CHOICE_B_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "Podazaj ta sciezka. Stary Oboz to najblizsze, mniej więcej bezpieczne miejsce jakie spotkasz po drodze."
        next: CHOICE_B_SENTENCE_3
    );
    static CHOICE_B_SENTENCE_3: DialogueItem = sentence!(
        DIEGO_ACTOR: "Pomiedzy obozami kreci sie wiele groznych zwierzat. Radzilbym ci postarac sie o jakas bron."
        next: ENABLE_CHOICE_C
    );

    static ENABLE_CHOICE_C: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_B.enabled = false;
            CHOICE_C.enabled = true;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });

    pub static mut CHOICE_C: PlayerChoice = PlayerChoice {
        choice: "Gdzie moge znalezc orez?",
        enabled: false,
        next_item: Some(&CHOICE_C_SENTENCE_1),
    };
    static CHOICE_C_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "Gdzie moge znalezc orez?"
        next:  CHOICE_C_SENTENCE_2
    );
    static CHOICE_C_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "Rozejrzyj sie troche w poblizu Opuszczonej Kopalni. Na pewno znajdziesz tam cos przydatnego."
        next: CHOICE_C_SENTENCE_3
    );
    static CHOICE_C_SENTENCE_3: DialogueItem = sentence!(
        DIEGO_ACTOR: "Kopalnie nie trudno znalezc. Lezy pare metrow w dol kanionu."
        next: COMPLETE
    );

    static COMPLETE: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_C.enabled = false;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });
}

mod why_help {
    use crate::dialogue::{DialogueItem, PlayerChoice, Script};
    use crate::game::dialogue::{DIEGO_ACTOR, PLAYER_ACTOR};
    use crate::sentence;

    use super::PLAYER_CHOICE;

    pub static mut CHOICE_A: PlayerChoice = PlayerChoice {
        choice: "Dlaczego mi pomogles?",
        enabled: true,
        next_item: Some(&CHOICE_A_SENTENCE_1),
    };
    static CHOICE_A_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "Dlaczego mi pomogles?"
        next: CHOICE_A_SENTENCE_2
    );
    static CHOICE_A_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "Bo potrzebowales pomocy. Gdyby nie ja, Bullit i jego chlopcy mogliby cie wykonczyc."
        next: CHOICE_A_SENTENCE_3
    );
    static CHOICE_A_SENTENCE_3: DialogueItem = sentence!(
        DIEGO_ACTOR: "A ja jestem zbyt mily, zeby sie temu spokojnie przygladac. W koncu przebylem cala te droge po to, by zlozyc propozycje."
        next: CHOICE_A_SENTENCE_4
    );
    static CHOICE_A_SENTENCE_4: DialogueItem = sentence!(
        PLAYER_ACTOR: "Propozycje...?"
        next: CHOICE_A_SENTENCE_5
    );
    static CHOICE_A_SENTENCE_5: DialogueItem = sentence!(
        DIEGO_ACTOR: "Tak. Po tym zajsciu z Bullitiem i jego ludzmi powinienes sie domyslic, ze przyda ci sie ochrona."
        next: CHOICE_A_SENTENCE_6
    );
    static CHOICE_A_SENTENCE_6: DialogueItem = sentence!(
        DIEGO_ACTOR: "Kazdy, kto tu trafia ma wybor. W kolonii sa trzy obozy i w koncu bedziesz musial do ktoregos dolaczyc."
        next: CHOICE_A_SENTENCE_7
    );
    static CHOICE_A_SENTENCE_7: DialogueItem = sentence!(
        DIEGO_ACTOR: "Jestem tu, by udowodnic wszystkim nowym, ze najlepszym miejscem dla nich bedzie Stary Oboz."
        next: ENABLE_CHOICE_B
    );

    static ENABLE_CHOICE_B: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_A.enabled = false;
            CHOICE_B.enabled = true;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });

    pub static mut CHOICE_B: PlayerChoice = PlayerChoice {
        choice: "A gdzie jest ten Bullit?",
        enabled: false,
        next_item: Some(&CHOICE_B_SENTENCE_1),
    };
    static CHOICE_B_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "A gdzie jest ten Bullit?"
        next: CHOICE_B_SENTENCE_2
    );
    static CHOICE_B_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "On i pozostali przenosza towary z zewnatrz do Obozny. Tam go znajdziesz."
        next: CHOICE_B_SENTENCE_3
    );
    static CHOICE_B_SENTENCE_3: DialogueItem = sentence!(
        DIEGO_ACTOR: "Ale jesli chcesz z nim walczyc, radze ci uwazac. To doswiadczony wojownik."
        next: COMPLETE
    );

    static COMPLETE: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_B.enabled = false;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });
}

mod letter_to_fire_mages {
    use crate::dialogue::{DialogueItem, PlayerChoice, Script};
    use crate::game::dialogue::{DIEGO_ACTOR, PLAYER_ACTOR};
    use crate::sentence;

    use super::PLAYER_CHOICE;

    pub static mut CHOICE_A: PlayerChoice = PlayerChoice {
        choice: "Mam list do przywodcy Magow Ognia.",
        enabled: true,
        next_item: Some(&CHOICE_A_SENTENCE_1),
    };
    static CHOICE_A_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "Mam list do przywodcy Magow Ognia."
        next: CHOICE_A_SENTENCE_2
    );
    static CHOICE_A_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "Czyzby...?"
        next: CHOICE_A_SENTENCE_3
    );
    static CHOICE_A_SENTENCE_3: DialogueItem = sentence!(
        PLAYER_ACTOR: "Jakis mag dal mi go zanim mnie tu wrzucono."
        next: CHOICE_A_SENTENCE_4
    );
    static CHOICE_A_SENTENCE_4: DialogueItem = sentence!(
        DIEGO_ACTOR: "Twoje szczescie, ze nie moge sie wiecej pokazywac u magow. Ktos inny moglby ci poderznac gardlo za taki list."
        next: CHOICE_A_SENTENCE_5
    );
    static CHOICE_A_SENTENCE_5: DialogueItem = sentence!(
        DIEGO_ACTOR: "A to dlatego, ze magowie hojnie oplacaja swoich kurierow, a wiekszosc z tutejszych ludzi nic nie posiada."
        next: CHOICE_A_SENTENCE_6
    );
    static CHOICE_A_SENTENCE_6: DialogueItem = sentence!(
        DIEGO_ACTOR: "Na twoim miejscu trzymalbym jezyk za zebami az do chwili, gdy spotkasz ktoregos z magow. Chociaz watpie, zeby ci sie udalo."
        next: CHOICE_A_SENTENCE_7
    );
    static CHOICE_A_SENTENCE_7: DialogueItem = sentence!(
        PLAYER_ACTOR: "Dlaczego?"
        next: CHOICE_A_SENTENCE_8
    );
    static CHOICE_A_SENTENCE_8: DialogueItem = sentence!(
        DIEGO_ACTOR: "Magowie mieszkaja w zamku, w Starym Obozie. Tylko ludzie Gomeza maja tam wstep."
        next: ENABLE_CHOICE_B_C
    );

    static ENABLE_CHOICE_B_C: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_A.enabled = false;
            CHOICE_B.enabled = true;
            CHOICE_C.enabled = true;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });

    pub static mut CHOICE_B: PlayerChoice = PlayerChoice {
        choice: "Kim jest Gomez?",
        enabled: false,
        next_item: Some(&CHOICE_B_SENTENCE_1),
    };
    static CHOICE_B_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "Kim jest Gomez?"
        next: CHOICE_B_SENTENCE_2
    );
    static CHOICE_B_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "Gomez jest najpotezniejszym z Magnatow kierujacych handlem ruda. To on rzadzi w Starym Obozie i ma najwiecej do powiedzenia w calej kolonii."
        next: DISABLE_CHOICE_B
    );
    static DISABLE_CHOICE_B: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_B.enabled = false;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });

    pub static mut CHOICE_C: PlayerChoice = PlayerChoice {
        choice: "Zalozmy, ze chcialbym dolaczyc do jego ludzi. Co powinienem zrobic?",
        enabled: false,
        next_item: Some(&CHOICE_C_SENTENCE_1),
    };
    static CHOICE_C_SENTENCE_1: DialogueItem = sentence!(
        PLAYER_ACTOR: "Zalozmy, ze chcialbym dolaczyc do jego ludzi. Co powinienem zrobic?"
        next: CHOICE_C_SENTENCE_2
    );
    static CHOICE_C_SENTENCE_2: DialogueItem = sentence!(
        DIEGO_ACTOR: "Przy bramie do zamku znajdziesz czlowieka imieniem Thorus. Powiedz mu, ze Diego cie przyslal."
        next: DISABLE_CHOICE_C
    );
    static DISABLE_CHOICE_C: DialogueItem = DialogueItem::Script(Script {
        update: |_context| unsafe {
            CHOICE_C.enabled = false;
            true
        },
        render: |_context| {},
        next_item: Some(&PLAYER_CHOICE),
    });
}