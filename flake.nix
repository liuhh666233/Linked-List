{
  description = "A rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        DevPython = pkgs.python3.withPackages (ps:
          with ps; [
            # Development-only packages
            jupyterlab
            ipywidgets
            notebook
            pandas
            numpy
            matplotlib
            plotly
            xlrd
            openpyxl
          ]);
      in {
        devShells.default = with pkgs;
          mkShell.override { stdenv = pkgs.llvmPackages_18.stdenv; } rec {
            stdenv = pkgs.llvmPackages_18.stdenv;
            buildInputs = [
              openssl
              pkg-config
              fd
              evcxr
              DevPython
              rustup
              gcc
              binutils
              (rust-bin.beta.latest.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
              })
            ];

            # Setting up the environment variables you need during
            # development.
            shellHook = let
              icon = "f121";
              name = "RustDemo";
            in ''
              export PS1="$(echo -e '\u${icon}') {\[$(tput sgr0)\]\[\033[38;5;228m\]\w\[$(tput sgr0)\]\[\033[38;5;15m\]} (${name}) \\$ \[$(tput sgr0)\]"
              export RUST_BACKTRACE=full;
              export NIX_ENFORCE_NO_NATIVE=0;
              export LD_LIBRARY_PATH=${pkgs.llvmPackages_18.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
              export RUSTFLAGS="-C linker=gcc"
            '';
          };
      });
}
