{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # Zaležnosti dlja Rust
  nativeBuildInputs = with pkgs; [
    rustc # Compiljator Rust
    cargo # Menedžer paketiv Cargo
  ];

  # Zminni seredovyŝa
  shellHook = ''
    echo "Ğotovo (NixOS)"
  '';
}
