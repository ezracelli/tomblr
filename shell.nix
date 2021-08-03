{ pkgs ? import<nixpkgs> {} }:

pkgs.mkShell {
    nativeBuildInputs = [
        pkgs.cargo
        pkgs.cargo-watch
        pkgs.diesel-cli
        pkgs.docker
        pkgs.docker-compose
        pkgs.gcc
        pkgs.nodejs-16_x
        pkgs.postgresql_13
        pkgs.python39Packages.pgsanity
        pkgs.yarn
    ];
}
