#!/usr/bin/env bash
# AlphaWord Blocks — download dwyl/english-words "words_alpha.txt".
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
# words_alpha.txt comes from https://github.com/dwyl/english-words and is
# released into the public domain via the Unlicense. It is fetched on demand
# rather than redistributed here.
set -euo pipefail
cd "$(dirname "$0")"

URL="https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt"
echo "Downloading $URL ..."
curl -fSL "$URL" -o words_alpha.txt

echo
echo "Done -> words_alpha.txt. Copy into ../data/ and add it to the corpus"
echo "load list in src/interactive.rs and/or check_spellable.jl."
