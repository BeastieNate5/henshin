{
  rustPlatform
}:
rustPlatform.buildRustPackage {
  pname = "henshin";
  version = "0.1.0";
  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
