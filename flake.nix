{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: let
    perSystem = fn:
      inputs.nixpkgs.lib.genAttrs ["x86_64-linux"] (system:
        fn (import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.fenix.overlays.default];
        }));
  in {
    devShells = perSystem (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rust-analyzer-nightly
          (fenix.stable.withComponents
            ["cargo" "clippy" "rustc" "rust-src" "rustfmt"])

          pkg-config
          wayland
          libxkbcommon
          xorg.libX11
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [wayland vulkan-loader]);
      };
    });
  };
}
