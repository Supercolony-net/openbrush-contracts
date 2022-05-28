cargo publish --package openbrush_lang_codegen --keep-going
sleep 5
cargo publish --package openbrush_lang_macro --keep-going
sleep 5
cargo publish --package openbrush_lang --keep-going
sleep 5
cargo publish --package openbrush_contracts_derive --keep-going
sleep 5
cargo publish --package openbrush_contracts --keep-going
sleep 5
cargo publish --package openbrush --keep-going