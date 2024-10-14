{
  description = "tyrell development environment";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ...}:
  flake-utils.lib.eachDefaultSystem (system: 
    let
      overlays = [ (import rust-overlay) ];

      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rustVersion = pkgs.rust-bin.stable.latest.default;
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustVersion
          cargo
          rustc
          rustfmt
          rust-analyzer
          clippy
          openssl
          pkg-config
          bacon
          mdbook
        ];
        shellHook = ''
          echo "rust version: $(rustc --version)"
        '';
      };
    }
  );
}
