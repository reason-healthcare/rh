{
  description = "Rust Health (rh) - High-performance FHIR toolkit";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "rh";
          version = "0.2.2";

          src = ./.;

          cargoHash = "sha256-iumklh82kHsTiCZU3N9e2DLkh3759TZBzVxk9QZypA0=";

          cargoBuildFlags = [
            "-p"
            "rh-cli"
          ];
          cargoTestFlags = [
            "-p"
            "rh-cli"
          ];

          meta = with pkgs.lib; {
            description = "High-performance FHIR toolkit and CLI written in Rust";
            longDescription = ''
              Rust Health (rh) is a modern, high-performance toolkit for working with
              HL7® FHIR®, purpose-built in Rust. Ships as a cross-platform CLI, Rust
              library crates, and WebAssembly-backed npm packages.

              Features include:
              - FHIR resource validation
              - FHIRPath expression evaluation
              - FHIR Shorthand (FSH) compilation
              - CQL to ELM translation
              - FHIR package management
              - VCL (Value Set Composition Language) support
            '';
            homepage = "https://github.com/reason-healthcare/rh";
            changelog = "https://github.com/reason-healthcare/rh/releases/tag/v${version}";
            license = with licenses; [
              mit
              asl20
            ];
            mainProgram = "rh";
            platforms = platforms.all;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
            pkg-config
            openssl
          ];

          shellHook = ''
            echo "Rust Health dev environment loaded"
            echo "Try: cargo build -p rh-cli"
          '';
        };
      }
    );
}
