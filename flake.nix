{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell rec {
        name = "asedia-shell";

        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          clippy
          rustfmt
          deadnix
          nixfmt-rfc-style
        ];

        buildInputs = with pkgs; [
          expat
          fontconfig
          freetype
          freetype.dev
          libGL
          pkg-config
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          wayland
          libxkbcommon
        ];

        LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
      };
    };
}
