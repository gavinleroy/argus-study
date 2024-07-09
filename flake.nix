{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
  flake-utils.lib.eachDefaultSystem (system:
  let 
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in {
    devShell = with pkgs; mkShell { 
      buildInputs = [ 
        llvmPackages_latest.llvm
        llvmPackages_latest.lld
        rust-bin.stable.latest.default
        rust-analyzer
        rustfmt
        clippy
      ] ++ lib.optional stdenv.isDarwin libiconv; 

      RUSTC_LINKER = "${pkgs.llvmPackages.clangUseLLVM}/bin/clang";
    };
  });
}
