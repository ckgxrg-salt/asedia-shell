//! Simple library for reading a base16 toml colourscheme.

use serde::Deserialize;

/// Structure of a base16 colourscheme
#[derive(Deserialize)]
pub struct ColourScheme {
    #[serde(rename = "base00")]
    bg: String,
    #[serde(rename = "base01")]
    light_bg: String,
    #[serde(rename = "base02")]
    select_bg: String,
    #[serde(rename = "base03")]
    invisible: String,
    #[serde(rename = "base04")]
    dark_fg: String,
    #[serde(rename = "base05")]
    fg: String,
    #[serde(rename = "base06")]
    light_fg: String,
    #[serde(rename = "base07")]
    lightest_fg: String,
    #[serde(rename = "base08")]
    red: String,
    #[serde(rename = "base09")]
    orange: String,
    #[serde(rename = "base0A")]
    yellow: String,
    #[serde(rename = "base0B")]
    green: String,
    #[serde(rename = "base0C")]
    cyan: String,
    #[serde(rename = "base0D")]
    blue: String,
    #[serde(rename = "base0E")]
    magenta: String,
    #[serde(rename = "base0F")]
    dark_red: String,
}
