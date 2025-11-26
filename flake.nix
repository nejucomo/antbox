{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        buildInputs = with pkgs; [
          SDL2
          SDL2_gfx
          SDL2_image
          SDL2_mixer
          SDL2_ttf
        ];

        antboxPkg = pkgs.rustPlatform.buildRustPackage {
          pname = "antbox";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [ rustToolchain ];

          inherit buildInputs;

          meta = with pkgs.lib; {
            homepage = "https://github.com/nejucomo/antbox";
            license = licenses.mit;
            maintainers = [ ];
          };
        };
      in
      {
        packages.default = antboxPkg;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ antboxPkg ];

          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            pkg-config
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs}"
          '';
        };
      }
    );
}
