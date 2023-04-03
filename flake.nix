{
  description = "A Rust development environment";

  #inputs = {
  #  nixpkgs.url = "github:nixos/nixpkgs/nixos";
  #};

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux = with nixpkgs; {

      # Define the Rust toolchain and dependencies
      rust = rustChannel {
        channel = "stable";
        packages = [
          "cargo"
          "rust-src"
        ];
      };

      # Define the development environment
      devShell = mkShell {
        buildInputs = [
          pkg-config
          openssl
          rust.rust
          lld
          llvmPackages.libclang
        ];
      };
    };
  };
}

