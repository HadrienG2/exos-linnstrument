use rand::prelude::*;

fn main() {
    // === Progressions d'accords courantes dans différents modes ===

    let prog_majeur: &[&[&str]] = &[
        // Faites avec I, IV et V
        &[
            "I I⁶ IV V",
            "I IV V",
            "I IV I V",
            "I V IV I",
            "I V⁶ IV V",
            "I I⁶ IV V",
        ],
        // Faites avec I, IV, V et vi
        &[
            "I IV V vi",
            "I IV vi V",
            "I V vi IV",
            "I V⁶ vi IV",
            "I vi IV V",
            "I vi V IV",
            "vi IV I V",
        ],
        // Faites avec I, ii, et un mélange de IV, V et vi
        &["I IV ii V", "I vi ii V", "I vi IV ii", "ii V I"],
        // Faites avec I, iii, V et iv
        &["I iii vi V", "I vi iii V"],
        // Autres progressions, pas groupées pour l'instant
        &["I IV"],
        &["I I⁶ vi7 V"],
        &["I II IV V"],
        &["I iii IV V"],
        &["I III IV V"],
        &["I IV VII III"],
    ];
    let prog_dorien: &[&[&str]] = &[
        &["i ii III ii"],
        &["i III IV IV"],
        &["i III IV ♭VI"],
        &["i IV i"],
        &["i v IV i"],
        &["i VII III IV"],
        &["i VII ♭VI IV"],
    ];
    let prog_phrygien: &[&[&str]] = &[
        &["i II"],
        &["i II i vii"],
        &["i II III II"],
        &["i III vii II"],
        &["i iv III II"],
        &["i vii⁶ II III7"],
    ];
    let prog_lydien: &[&[&str]] = &[
        &["I I II V"],
        &["I II iii II"],
        &["I II vii iii"],
        &["I iii II⁶ V"],
        &["I V iii II"],
    ];
    let prog_mixolydien: &[&[&str]] = &[
        &["I ii v I"],
        &["I v IV I"],
        &["I VII ii I"],
        &["I VII IV I"],
    ];
    let prog_mineur: &[&[&str]] = &[
        &["i III iv"],
        &["i III VII iv", "i III VII v"],
        &["i iv iim7♭5 V"],
        &["i iv III VI"],
        &["i iv v i", "i iv V i"],
        &["i v VI VII", "i v⁶ VI VII"],
        &["i vi ii V"],
        &["i VI III VII"],
        &["i VII VI V", "i VII VI VII"],
        &["iv v"],
    ];
    let prog_locrien: &[&[&str]] = &[&["i° II iii II"], &["i° II iii iv"]];

    // === Toniques et modes travaillés ===

    // TODO: Ajouter toniques altérées quand j'aurai plus d'expé
    let toniques = &['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let modes = &[
        ("majeur", prog_majeur),
        ("dorien", prog_dorien),
        ("phrygien", prog_phrygien),
        ("lydien", prog_lydien),
        ("mixolydien", prog_mixolydien),
        ("mineur naturel", prog_mineur),
        ("mineur harmonique", prog_mineur),
        ("mineur mélodique", prog_mineur),
        ("locrien", prog_locrien),
    ];

    // === Tirage au hasard du tout ===

    let mut rng = rand::thread_rng();
    let tonique = toniques.choose(&mut rng).unwrap();
    let (mode, progressions) = modes.choose(&mut rng).unwrap();
    println!("Aujourd'hui on va travailler {tonique} {mode}");
    let famille = progressions.choose(&mut rng).unwrap();
    let progression = famille.choose(&mut rng).unwrap();
    println!("Et la progression d'accords {progression}");
}
