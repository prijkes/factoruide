//! Writes the money coin icons and the tooltip font out of an installed client into the app's
//! assets (`app/src/assets`, see NOTICE.md): any product has them (`wowf` is one), and they
//! never change between game versions, so this is run once for good.
//!
//!   cargo run -p fg-casc --example extract-client-assets -- "F:\World of Warcraft"

use fg_casc::{Storage, blp};
use std::path::{Path, PathBuf};

fn main() {
    let install = std::env::args()
        .nth(1)
        .expect("the install path (its `World of Warcraft` folder)");
    let storage = Storage::open(Path::new(&install), "wowf")
        .expect("opening the install's storage (its `World of Warcraft` folder)");
    let out = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../app/src/assets");
    let coins = out.join("coins");
    std::fs::create_dir_all(&coins).expect("making the coins' folder");
    for (name, file) in [
        ("Interface\\MoneyFrame\\UI-GoldIcon.blp", "gold.png"),
        ("Interface\\MoneyFrame\\UI-SilverIcon.blp", "silver.png"),
        ("Interface\\MoneyFrame\\UI-CopperIcon.blp", "copper.png"),
    ] {
        let bytes = storage
            .file_by_name(name)
            .expect("reading the manifest")
            .expect("the game's own file in the manifest");
        let png = blp::png(&bytes).expect("decoding the coin's picture");
        std::fs::write(coins.join(file), png).expect("writing the png");
    }
    let font = storage
        .file_by_name("Fonts\\FRIZQT__.TTF")
        .expect("reading the manifest")
        .expect("the game's own file in the manifest");
    std::fs::write(out.join("friz-quadrata.ttf"), font).expect("writing the font");
}
