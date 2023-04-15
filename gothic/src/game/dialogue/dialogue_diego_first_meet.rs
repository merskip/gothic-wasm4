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

sentence!(SENTENCE_1
    DIEGO_ACTOR: "Nazywam sie Diego."
    next: SENTENCE_2
);
sentence!(SENTENCE_2
    PLAYER_ACTOR: "Jestem..."
    next: SENTENCE_3
);
sentence!(SENTENCE_3
    DIEGO_ACTOR: "Nie interesuje mnie kim jestes. Jestes tu nowy - a do mnie nalezy dbanie o nowych. Na razie to tyle..."
    next: SENTENCE_4
);
sentence!(SENTENCE_4
    DIEGO_ACTOR: "Jesli chcesz jeszcze troche pozyc, sluchaj sie mnie, ale oczywiscie nie bede Ci przeszkadzal w ewentualnej probie samobojstwa. To jak bedzie?"
    next: CHOICE_1
);

static CHOICE_1: DialogueItem = DialogueItem::PlayerChoice {
    choices: &[
        PlayerChoice {
            choice: "Dobra, co powinienem wiedziec o tym miejscu?",
            next_item: Some(&CHOICE_1_SENTENCE_1),
        },
        PlayerChoice {
            choice: "Dlaczego mi pomogles?",
            next_item: None,  // TODO
        },
        PlayerChoice {
            choice: "Mam list do przywodcy Magow Ognia",
            next_item: None,  // TODO
        },
        PlayerChoice {
            choice: "KONIEC",
            next_item: None, // finishes dialogue
        }
    ]
};

sentence!(CHOICE_1_SENTENCE_1
    PLAYER_ACTOR: "Dobra, co powinienem wiedziec o tym miejscu?"
    next: CHOICE_1_SENTENCE_2
);

sentence!(CHOICE_1_SENTENCE_2
    DIEGO_ACTOR: "Nazywamy je kolonia. Wiesz juz, ze wydobywamy rude dla krola."
    next: CHOICE_1_SENTENCE_3
);

sentence!(CHOICE_1_SENTENCE_3
    DIEGO_ACTOR: "Coz, w kazdym razie tak robia ludzie ze Starego Obozu."
    next: CHOICE_1_SENTENCE_4
);

sentence!(CHOICE_1_SENTENCE_4
    DIEGO_ACTOR: "Wewnatrz Bariery powstaly trzy obozy. Najwiekszy i najstarszy jest tak zwany Stary Oboz."
    next: CHOICE_1
);
