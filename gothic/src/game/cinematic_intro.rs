use crate::game::game_scene::make_game_scene;
use crate::get_shared_image;
use crate::image_asset::ImageAsset;
use crate::image_asset::ImageAsset::KingRhobar2;
use crate::renderable::Canvas;
use crate::renderable::Color::{Primary, Secondary, Tertiary, Transparent};
use crate::ui::cinematic::cinematic::{Cinematic, CinematicScreen};
use crate::ui::cinematic::cinematic_player::CinematicPlayer;
use crate::ui::geometry::{Point, Rect};

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
            "Krolestwo Myrthany zjednoczone pod berlem krola Rhobara II. Podczas dlugich lat swego panowania Rhobar zdolal pokonac wszystkich wrogow krolestwa oprocz jednego...",
            draw_king_rhobar_2_scene,
        ),
        CinematicScreen::new(
            "Wojna z Orkami byla niezwykle kosztowna, a cene za nia miala poniesc niewielka grupa skazancow.",
            draw_orc_scene,
        ),
        CinematicScreen::new(
            "Krol potrzebowal mieczy i tarcz dla swego wojska, totez kazdy przestepca, niezaleznie od wystepku, jaki popelnil, byl zmuszany do prac w kopalniach rudy w Khorinis.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Aby uniemozliwic im ucieczke krol wyslal swych najpotezniejszych magow, aby ci otoczyli cala doline magiczna bariera, a ja bylem jednym z nich.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Niespodziewanie cos zaklocilo delikatna strukture zaklecia i zostalismy uwiezieni przez nasze wlasne dzielo. Wiezniowie blyskawicznie wykorzystali moment konsternacji.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Khorinis wraz ze wszystkimi swoimi kopalniami znajdowala sie w rekach wiezniow. Krol nie mial wyboru, musial negocjowac, potrzebowal rudy.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Miesiac po miesiacu krol dostarczal wiezniom wszystkiego, czego potrzebowali. Miesiac po miesiacu otrzymywal od nich ladunek cennej rudy.",
            draw_nothing,
        ),
        CinematicScreen::new(
            "Az do dzis, na skraj urwiska sprowadzono kolejnego wieznia. Nie wiedzial, co go czeka, a to wlasnie on mial wszystko odmienic.",
            draw_nothing,
        ),
    ]
);

fn draw_nothing(_canvas: &dyn Canvas, _frame: Rect) {}

fn draw_king_rhobar_2_scene(canvas: &dyn Canvas, frame: Rect) {
    canvas.set_image_colors([
        Transparent,
        Secondary,
        Transparent,
        Transparent,
    ]);
    let king_image = get_shared_image(KingRhobar2);
    canvas.draw_image(king_image, frame.centered(king_image.size()).origin);
}

fn draw_orc_scene(canvas: &dyn Canvas, frame: Rect) {
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
    canvas.set_image_colors([
        Primary,
        Secondary,
        Transparent,
        Tertiary,
    ]);
    let orc_image = get_shared_image(ImageAsset::Orc);
    for origin in orc_positions {
        canvas.draw_image(orc_image, frame.origin + origin);
    }

    let crossbones_positions = [
        Point::new(80, 20),
        Point::new(100, 40),
        Point::new(140, 30),
        Point::new(90, 70),
        Point::new(120, 75),
    ];
    canvas.set_image_colors([
        Transparent,
        Secondary,
        Transparent,
        Transparent
    ]);
    let crossbones_image = get_shared_image(ImageAsset::Crossbones);
    for origin in crossbones_positions {
        canvas.draw_image(crossbones_image, frame.origin + origin);
    }
}