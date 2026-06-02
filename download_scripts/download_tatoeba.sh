#!/usr/bin/env bash
# AlphaWord Blocks — download & clean the Tatoeba English sentence export.
# Copyright (C) 2025- Svetlin Tassev
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#
# Tatoeba sentences are licensed CC-BY 2.0 FR (attribution required); they are
# NOT redistributed with this repository. This script fetches them on demand.
set -euo pipefail
cd "$(dirname "$0")"

URL="https://downloads.tatoeba.org/exports/per_language/eng/eng_sentences.tsv.bz2"
echo "Downloading $URL ..."
curl -fSL "$URL" -o eng_sentences.tsv.bz2
echo "Decompressing ..."
bunzip2 -f eng_sentences.tsv.bz2

echo "Cleaning (filtered_sentences.py) -> tatoeba_clean.txt ..."
python3 filtered_sentences.py

echo
echo "Done. To use it as AlphaWord Blocks corpus input:"
echo "  cp tatoeba_clean.txt ../data/ && add \"tatoeba_clean.txt\" to the load list"
echo "  in src/interactive.rs and/or check_spellable.jl."
echo "Remember to attribute Tatoeba (CC-BY 2.0 FR) if you redistribute the result."
