{
  description = "Advent of Code 2023";

  # Nixpkgs / NixOS version to use.
  # inputs.nixpkgs.url = "nixpkgs/nixos-21.05";

  inputs.import-cargo.url = github:edolstra/import-cargo;

  outputs = { self, nixpkgs, import-cargo }:
    let

      # Generate a user-friendly version number.
      version = "${builtins.substring 0 8 self.lastModifiedDate}-${self.shortRev or "dirty"}";

      # System types to support.
      supportedSystems = [ "x86_64-linux" ];

      # Helper function to generate an attrset '{ x86_64-linux = f "x86_64-linux"; ... }'.
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);

      # Nixpkgs instantiated for supported system types.
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; overlays = [ self.overlay ]; });

    in {

      # A Nixpkgs overlay.
      overlay = final: prev: {

        rihdl = with final; final.callPackage ({ inShell ? false }: stdenv.mkDerivation rec {
          name = "rihdl-${version}";

          # In 'nix develop', we don't need a copy of the source tree
          # in the Nix store.
          src = if inShell then null else ./.;
          
          RUST_SRC_PATH = "${final.rust.packages.stable.rustPlatform.rustLibSrc}";
          nativeBuildInputs =
            [ rustc
              cargo
            ] ++ (if inShell then [
              # In 'nix develop', provide some developer tools.
              rustfmt
              rust-analyzer
              clippy
            ] else [
              (import-cargo.builders.importCargo {
                lockFile = ./Cargo.lock;
                inherit pkgs;
              }).cargoHome
            ]);
          shellHook = ''
          '';

          target = "--release";

          buildPhase = "cargo build ${target} --frozen --offline";

          doCheck = true;

          checkPhase = "cargo test ${target} --frozen --offline";

          installPhase =
            ''
              mkdir -p $out
              cargo install --frozen --offline --path . --root $out
              rm $out/.crates.toml
            '';
        }) {};

      };

      # Provide some binary packages for selected system types.
      packages = forAllSystems (system:
        {
          inherit (nixpkgsFor.${system}) rihdl;
        });

      # The default package for 'nix build'. This makes sense if the
      # flake provides only one package or there is a clear "main"
      # package.
      defaultPackage = forAllSystems (system: self.packages.${system}.rihdl);

      # Provide a 'nix develop' environment for interactive hacking.
      devShell = forAllSystems (system: self.packages.${system}.rihdl.override { inShell = true; });

    };
}
