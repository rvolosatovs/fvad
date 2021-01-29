{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    autoconf
    automake
    libsndfile
    libtool
    rustup
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
}
