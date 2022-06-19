#!/usr/bin/env fish
set FILENAME (status filename)
set DIR (dirname $FILENAME)
cd "$DIR/../../.."
mkdir -p core_book/generated
cd rust
mkdir -p combat_results
cargo run --bin monsters_chapter > ../core_book/generated/monster_descriptions.tex
cargo run --bin modules_chapter > ../core_book/generated/modules.tex
cargo run --bin classes_chapter > ../core_book/generated/classes.tex
cargo run --bin generate_combat_results > combat_results/standard_encounter.json