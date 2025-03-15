with import <nixpkgs> {};

mkShell {
  buildInputs = [
    libmysqlclient
    rustup
  ];
}
