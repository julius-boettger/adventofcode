{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    systems.url = "github:nix-systems/default"; # can run on all systems
  };

  outputs = { self, nixpkgs, systems, ... }:
  let
    eachSystem = fn: nixpkgs.lib.genAttrs (import systems) (system: fn system (import nixpkgs {
      inherit system;
    }));
  in
  {
    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustc cargo # rust
          clippy # better linter
          cargo-edit # provides `cargo upgrade` for dependencies
          samply # profiler
          hyperfine # benchmarking
        ];
        # fix rust-analyzer in vscode
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}
