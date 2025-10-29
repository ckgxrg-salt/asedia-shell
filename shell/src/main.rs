use asedia_shell::app::AsediaShell;

use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};

use iced_layershell::application;

fn main() -> Result<(), iced_layershell::Error> {
    application(
        AsediaShell::new,
        AsediaShell::namespace,
        AsediaShell::update,
        AsediaShell::view,
    )
    .style(AsediaShell::style)
    .subscription(AsediaShell::subscription)
    .settings(Settings {
        layer_settings: LayerShellSettings {
            anchor: Anchor::Left | Anchor::Right | Anchor::Bottom,
            layer: Layer::Bottom,
            start_mode: StartMode::TargetScreen(String::from("eDP-1")),
            ..Default::default()
        },
        ..Default::default()
    })
    .run()
}
