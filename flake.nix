{
  description = "Fundoc - a documentation generator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system;
          overlays = overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        fundoc = pkgs.rustPlatform.buildRustPackage {
          pname = "fundoc";
          version = "0.5.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];

          meta = with pkgs.lib; {
            description = "Fundoc - a documentation generator that helps you keep your documentation up-to-date.";
            homepage = "https://github.com/daynin/fundoc";
            license = licenses.mit;
          };
        };
      in {
        packages.default = fundoc;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.pkg-config
          ];
          RUST_BACKTRACE = "1";
        };
      });
}
