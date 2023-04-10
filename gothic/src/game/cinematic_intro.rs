use wasm4::framebuffer::{DrawColorIndex, Framebuffer};
use wasm4::framebuffer::PaletteIndex::{Palette2, Palette3, Palette4, Transparent};
use wasm4::geometry::{Point, Rect};
use crate::game::game_scene::make_game_scene;

use crate::sprites::{CROSSBONES_SPRITE, KING__RHOBAR_2_SPRITE, ORC_SPRITE};
use crate::ui::cinematic::cinematic::Cinematic;
use crate::ui::cinematic::cinematic_player::CinematicPlayer;
use crate::ui::cinematic::cinematic_screen::CinematicScreen;

pub fn make_cinematic_intro() -> CinematicPlayer {
    CinematicPlayer::new(
        &CINEMATIC_INTRO,
        |context| {
            context.navigator.push_view(make_game_scene());
        },
    )
}

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
            "Krolestwo Myrthany\n\
            zjednoczone pod\n\
            berlem krola\n\
            Rhobara II.\n\
            Podczas dlugich lat\n\
            swego panowania\n\
            Rhobar zdolal\n\
            pokonac wszystkich\n\
            wrogow krolestwa\n\
            oprocz jednego...",
            draw_king_rhobar_2_scene,
        ),
        CinematicScreen::new(
            "Wojna z Orkami\n\
            byla niezwykle\n\
            kosztowna,\n\
            a cene za nia\n\
            miala poniesc\n\
            niewielka grupa\n\
            skazancow.",
            draw_orc_scene,
        ),
        CinematicScreen::new(
            "Krol potrzebowal\n\
            mieczy i tarcz\n\
            dla swego wojska,\n\
            totez kazdy\n\
            przestepca,\n\
            niezaleznie\n\
            od wystepku,\n\
            jaki popelnil,\n\
            byl zmuszany\n\
            do prac w\n\
            kopalniach rudy\n\
            w Khorinis.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Aby uniemozliwic\n\
            im ucieczke\n\
            krol wyslal swych\n\
            najpotezniejszych\n\
            magow, aby ci\n\
            otoczyli cala\n\
            doline magiczna\n\
            bariera,\n\
            a ja bylem\n\
            jednym z nich.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Niespodziewanie cos\n\
            zaklocilo delikatna\n\
            strukture zaklecia\n\
            i zostalismy\n\
            uwiezieni przez\n\
            nasze wlasne dzielo.\n\
            Wiezniowie\n\
            blyskawicznie\n\
            wykorzystali\n\
            moment konsternacji.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Khorinis wraz ze\n\
            wszystkimi swoimi\n\
            kopalniami\n\
            znajdowala sie\n\
            w rekach wiezniow.\n\
            Krol nie mial\n\
            wyboru,\n\
            musial negocjowac,\n\
            potrzebowal rudy.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Miesiac po miesiacu\n\
            krol dostarczal\n\
            wiezniom wszystkiego,\n\
            czego potrzebowali.\n\
            Miesiac po miesiacu\n\
            otrzymywal od nich\n\
            ladunek cennej rudy.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Az do dzis,\n\
            na skraj urwiska\n\
            sprowadzono kolejnego\n\
            wieznia.\n\
            Nie wiedzial,\n\
            co go czeka,\n\
            a to wlasnie\n\
            on mial wszystko\n\
            odmienic.",
            draw_nothing,
        ),
    ]
);

fn draw_nothing(_framebuffer: &Framebuffer, _frame: Rect) {}

fn draw_king_rhobar_2_scene(framebuffer: &Framebuffer, frame: Rect) {
    framebuffer.set_draw_color(DrawColorIndex::Index1, Transparent);
    framebuffer.set_draw_color(DrawColorIndex::Index2, Palette3);
    framebuffer.sprite(
        KING__RHOBAR_2_SPRITE,
        frame.centered(KING__RHOBAR_2_SPRITE.size()).origin,
    );
}

fn draw_orc_scene(framebuffer: &Framebuffer, frame: Rect) {
    framebuffer.set_draw_colors([
        Palette2,
        Palette3,
        Transparent,
        Palette4,
    ]);
    let orc_positions = [
        Point::new(10, 10),
        Point::new(20, 20),
        Point::new(10, 30),
        Point::new(30, 30),
        Point::new(20, 40),
        Point::new(40, 40),
        Point::new(10, 50),
        Point::new(30, 50),
        Point::new(20, 60),
        Point::new(10, 70),
    ];
    for origin in orc_positions {
        framebuffer.sprite(
            ORC_SPRITE,
            frame.origin + origin,
        );
    }

    framebuffer.set_draw_colors([
        Transparent,
        Palette3,
        Transparent,
        Transparent,
    ]);
    let crossbones_positions = [
        Point::new(80, 10),
        Point::new(100, 40),
        Point::new(140, 30),
        Point::new(90, 70),
        Point::new(120, 100),
    ];
    for origin in crossbones_positions {
        framebuffer.sprite(
            CROSSBONES_SPRITE,
            frame.origin + origin,
        );
    }
}