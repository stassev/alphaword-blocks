#!/usr/bin/env bash
# AlphaWord Blocks — download the English Wiktionary page-title list (ns0).
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
# Wiktionary content is licensed CC-BY-SA 4.0 (and GFDL); it is NOT
# redistributed with this repository. This script fetches it on demand.
set -euo pipefail
cd "$(dirname "$0")"

URL="https://dumps.wikimedia.org/enwiktionary/latest/enwiktionary-latest-all-titles-in-ns0.gz"
echo "Downloading $URL ..."
curl -fSL "$URL" -o enwiktionary-latest-all-titles-in-ns0.gz
echo "Decompressing ..."
gunzip -f enwiktionary-latest-all-titles-in-ns0.gz

echo
echo "Done -> enwiktionary-latest-all-titles-in-ns0 (one page title per line)."
echo "Lowercase, strip non-letters, drop multi-word titles, then copy the"
echo "result into ../data/ and add it to the corpus load list."
echo "Attribute Wiktionary (CC-BY-SA 4.0) if you redistribute the result."
