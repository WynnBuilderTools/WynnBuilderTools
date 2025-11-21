{
  description = "Rust flake with nightly";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    lib = pkgs.lib;
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        fenix.packages.${system}.default.toolchain
        pkg-config
        openssl
        rustfmt
      ];
      RUSTFLAGS = "-C target-cpu=native";
      LD_LIBRARY_PATH = lib.makeLibraryPath [pkgs.openssl];
    };
  };
}
