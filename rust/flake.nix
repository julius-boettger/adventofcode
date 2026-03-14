{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    systems.url = "github:nix-systems/default"; # can run on all systems
    rust-overlay = { url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs"; };
  };

  outputs = { self, nixpkgs, systems, rust-overlay, ... }:
  let
    overlays = [ rust-overlay.overlays.default ];
    eachSystem = fn: nixpkgs.lib.genAttrs (import systems) (system: fn system (import nixpkgs {
      inherit system overlays;
    }));
  in
  {
    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # nightly rust for experimental features
          (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            # fix rust-analyzer in vscode
            extensions = [ "rust-src" ];
          }))

          cargo-edit # provides `cargo upgrade` for dependencies
          samply # profiler
          hyperfine # benchmarking
          graphviz # graph visualization
        ];
      };
    });
  };
}
