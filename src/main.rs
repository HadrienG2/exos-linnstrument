use rand::prelude::*;

fn main() {
    let prog_majeur: &[&[&str]] = &[
        // Faites avec I, IV et V
        &["I IV V", "I IV I V", "I V IV I", "I V6 IV V", "I I6 IV V"],
        // Faites avec I, IV, V et vi
        &[
            "I I6 vi7 V",
            "I IV V vi",
            "I IV vi V",
            "I V vi IV",
            "I V6 vi IV",
            "I vi IV V",
            "I vi V IV",
            "vi IV I V",
        ],
        // Faites avec I, ii/II, et un mélange de IV, V et vi
        &["I II IV V", "I IV ii V", "I vi ii V", "I vi IV ii"],
        // Autres progressions, pas groupées pour l'instant
        &["I iii vi V"],
        &["I III IV V"],
        &["I IV VII III"],
        &["I vi iii V"],
    ];
    let prog_dorien: &[&[&str]] = &[
        &["i ii III ii"],
        &["i III IV IV", "i III IV bVI"],
        &["i v IV i"],
        &["i VII III IV", "i VII bVI IV"],
    ];
    let prog_phrygien: &[&[&str]] = &[
        &["i II i vii"],
        &["i II III II"],
        &["i III vii II"],
        &["i iv III II"],
    ];
    let prog_lydien: &[&[&str]] = &[
        &["I I II V"],
        &["I II iii II"],
        &["I II vii iii"],
        &["I iii II6 V"],
        &["I V iii II"],
    ];
    let prog_mixolydien: &[&[&str]] = &[
        &["I ii v I"],
        &["I v IV I"],
        &["I VII ii I"],
        &["I VII IV I"],
    ];
    let prog_mineur: &[&[&str]] = &[
        &["i III VII iv"],
        &["i iv iim7b5 V"],
        &["i iv V i"],
        &["i v VI VII"],
        &["i v6 VI VII"],
        &["i VII VI V"],
    ];
    let prog_locrien: &[&[&str]] = &[&["io II iii II"], &["io II iii iv"]];

    // TODO: Découpler tonique et mode quand j'aurai plus d'expé
    let toniques_modes = &[
        ("C ionien", prog_majeur),
        ("C majeur", prog_majeur),
        ("D dorien", prog_dorien),
        ("E phrygien", prog_phrygien),
        ("F lydien", prog_lydien),
        ("G mixolydien", prog_mixolydien),
        ("A éolien", prog_mineur),
        ("A mineur naturel", prog_mineur),
        ("A mineur harmonique", prog_mineur),
        ("A mineur mélodique", prog_mineur),
        ("B locrien", prog_locrien),
    ];

    let mut rng = rand::thread_rng();
    let (tonique_mode, progressions) = toniques_modes.choose(&mut rng).unwrap();
    println!("Aujourd'hui on va travailler le mode {tonique_mode}");
    let famille = progressions.choose(&mut rng).unwrap();
    let progression = famille.choose(&mut rng).unwrap();
    println!("Et la progression d'accords {progression}");
}
