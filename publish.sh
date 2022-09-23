cargo publish --manifest-path lang/codegen/Cargo.toml --keep-going
sleep 30
cargo publish --manifest-path lang/macro/Cargo.toml --keep-going
sleep 30
cargo publish --manifest-path lang/Cargo.toml --keep-going
sleep 30
cargo publish --manifest-path contracts/Cargo.toml --keep-going
sleep 30
cargo publish --manifest-path Cargo.toml --keep-going