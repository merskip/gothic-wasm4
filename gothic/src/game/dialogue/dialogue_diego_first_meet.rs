use crate::game::dialogue::*;
use crate::sentence;
use crate::ui::dialogue::dialogue::{Dialogue, DialogueItem, Sentence};

/*
Diego: Nazywam się Diego.
Bezimienny: Jestem...
Diego: Nie interesuje mnie kim jesteś. Jesteś tu nowy - a do mnie należy dbanie o nowych. Na razie to tyle...
Diego: Jeśli chcesz jeszcze trochę pożyć, słuchaj się mnie, ale oczywiście nie będę Ci przeszkadzał w ewentualnej próbie samobójstwa. To jak będzie?

A) Bezimienny:

1. Dobra, co powinienem wiedzieć o tym miejscu?
1a. Jak dostanę się do Starego Obozu?
1b. Gdzie mogę znaleźć oręż?
Dlaczego mi pomogłeś?
2a. A gdzie jest ten Bullit?
Mam list do przywódcy Magów Ognia
4a. Kim jest Gomez?
4b. Założmy, że chciałbym dołączyć do jego ludzi. Co powinienem zrobić?
5. KONIEC

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


pub const DIALOGUE_DIEGO_FIRST_MEET: &Dialogue = &Dialogue {
    items: &[
        sentence!(DIEGO_ACTOR: "Nazywam sie Diego."),
        sentence!(PLAYER_ACTOR: "Jestem..."),
        sentence!(DIEGO_ACTOR: "Nie interesuje mnie kim jestes. Jestes tu nowy - a do mnie nalezy dbanie o nowych. Na razie to tyle..."),
        sentence!(DIEGO_ACTOR: "Jesli chcesz jeszcze troche pozyc, sluchaj sie mnie, ale oczywiscie nie bede Ci przeszkadzal w ewentualnej probie samobojstwa. To jak bedzie?"),
    ],
};
