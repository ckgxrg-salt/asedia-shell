use dwsh_main::app::DaywatchShell;

use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};

use iced_layershell::application;

fn main() -> Result<(), iced_layershell::Error> {
    application(
        DaywatchShell::new,
        DaywatchShell::namespace,
        DaywatchShell::update,
        DaywatchShell::view,
    )
    .style(DaywatchShell::style)
    .subscription(DaywatchShell::subscription)
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
