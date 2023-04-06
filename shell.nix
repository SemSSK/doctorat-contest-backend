{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    name="dev-environment";
    buildInputs = with pkgs; [
        rustup
        cargo
        lldb_15
        llvmPackages.libclang
        pkgconfig
        openssl
    ];
}
