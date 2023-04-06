use wasm4::sprite::Sprite;
use crate::ui::cinematic::cinematic::Cinematic;
use crate::ui::cinematic::cinematic_screen::CinematicScreen;

/*
Królestwo Myrthany zjednoczone pod berłem króla Rhobara II.
Podczas długich lat swego panowania Rhobar zdołał pokonać wszystkich wrogów królestwa oprócz jednego..
Wojna z Orkami była niezwykle kosztowna, a cenę za nią miała ponieść niewielka grupa skazańców.
Król potrzebował mieczy i tarcz dla swego wojska, toteż każdy przestępca, niezależnie od występku, jaki popełnił zmuszany był do prac w kopalniach rudy w Khorinis.
Aby uniemożliwić im ucieczkę król wysłał swych najpotężniejszych magów, aby ci otoczyli całą dolinę magiczną barierą, a ja byłem jednym z nich.
Niespodziewanie coś zakłóciło delikatną strukturę zaklęcia i zostaliśmy uwięzieni przez nasze własne dzieło. Więźniowie błyskawicznie wykorzystali moment konsternacji.
Khorinis wraz ze wszystkimi swoimi kopalniami znajdowała się w rękach więźniów. Król nie miał wyboru, musiał negocjować, potrzebował rudy.
Miesiąc po miesiącu król dostarczał więźniom wszystkiego, czego potrzebowali. Miesiąc po miesiącu otrzymywał od nich ładunek cennej rudy.
Aż do dziś, na skraj urwiska sprowadzono kolejnego więźnia. Nie wiedział, co go czeka, a to właśnie on miał wszystko odmienić.
 */

pub static CINEMATIC_INTRO: Cinematic = Cinematic::new(
    &[
        CinematicScreen::new(
            Sprite::empty(),
            "Krolestwo Myrthany\nzjednoczone pod\nberlem krola\nRhobara II.\nPodczas dlugich lat\nswego panowania\nRhobar zdolal\npokonac wszystkich\nwrogow krolestwa\noprocz jednego...",
        ),
        CinematicScreen::new(
            Sprite::empty(),
            "Wojna z Orkami\nbyla niezwykle\nkosztowna,\na cene za nia\nmiala poniesc\nniewielka grupa\nskazancow.",
        ),
        CinematicScreen::new(
            Sprite::empty(),
            "Krol potrzebowal\nmieczy i tarcz\ndla swego wojska,\ntotez kazdy\nprzestepca,\nniezaleznie\nod wystepku,\njaki popelnil,\nbyl zmuszany\ndo prac w\nkopalniach rudy\nw Khorinis.",
        ),
        CinematicScreen::new(
            Sprite::empty(),
            "Aby uniemozliwic\nim ucieczke\nkrol wyslal swych\nnajpotezniejszych\nmagow, aby ci\notoczyli cala\ndoline magiczna\nbariera,\na ja bylem\njednym z nich.",
        ),
        CinematicScreen::new(
            Sprite::empty(),
            "Niespodziewanie cos\nzaklocilo delikatna\nstrukture zaklecia\ni zostalismy\nuwiezieni przez\nnasze wlasne dzielo.\nWiezniowie\nblyskawicznie\nwykorzystali\nmoment konsternacji.",
        ),
        CinematicScreen::new(
            Sprite::empty(),
            "Khorinis wraz ze\nwszystkimi swoimi\nkopalniami\nznajdowala sie\nw rekach wiezniow.\nKrol nie mial\nwyboru,\nmusial negocjowac,\npotrzebowal rudy.",
        ),
        CinematicScreen::new(
            Sprite::empty(),
            "Miesiac po miesiacu\nkrol dostarczal\nwiezniom wszystkiego,\nczego potrzebowali.\nMiesiac po miesiacu\notrzymywal od nich\nladunek cennej rudy.",
        ),
        CinematicScreen::new(
            Sprite::empty(),
            "Az do dzis,\nna skraj urwiska\nsprowadzono kolejnego\nwieznia.\nNie wiedzial,\nco go czeka,\na to wlasnie\non mial wszystko\nodmienic.",
        ),
    ]
);
