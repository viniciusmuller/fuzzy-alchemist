{
  description = "FuzzyAlchemist - A fuzzer for the elixir ecosystem";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          packages = [
            cargo
            rustc
            clippy
            rust-analyzer
            rustfmt

            elixir_1_15
            nil
          ];
        };
      });
}
