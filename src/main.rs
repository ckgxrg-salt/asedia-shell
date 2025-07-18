use asedia_shell::app::AsediaShell;

use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};

fn main() -> Result<(), iced_layershell::Error> {
    AsediaShell::run(Settings {
        layer_settings: LayerShellSettings {
            anchor: Anchor::Left | Anchor::Right | Anchor::Bottom,
            layer: Layer::Top,
            exclusive_zone: 400,
            size: Some((0, 400)),
            start_mode: StartMode::TargetScreen(String::from("eDP-1")),
            ..Default::default()
        },
        ..Default::default()
    })
}
