#!/usr/bin/env bash
# AlphaWord Blocks — fetch an English Wikipedia article dump and extract plain text.
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
# Wikipedia content is licensed CC-BY-SA 4.0 (and GFDL); it is NOT
# redistributed with this repository. This script fetches it on demand.
set -euo pipefail
cd "$(dirname "$0")"

URL="https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pages-articles.xml.bz2"

if [[ "${1:-}" != "--download" ]]; then
  cat <<EOF
The English Wikipedia article dump is very large (tens of GB compressed).
This script does nothing unless you pass --download, e.g.:

  ./download_wikipedia.sh --download

It will then:
  1. Download:  $URL
  2. Extract plain text with WikiExtractor (pip install wikiextractor):
       wikiextractor enwiki-latest-pages-articles.xml.bz2 -o wiki_text
  3. You tokenize wiki_text/*/wiki_* into lowercase words and copy the
     result into ../data/, then add it to the corpus load list.

Attribute Wikipedia (CC-BY-SA 4.0) if you redistribute the result.
EOF
  exit 0
fi

echo "Downloading $URL ..."
curl -fSL "$URL" -o enwiki-latest-pages-articles.xml.bz2

if python3 -c "import wikiextractor" 2>/dev/null; then
  echo "Extracting plain text into wiki_text/ ..."
  python3 -m wikiextractor.WikiExtractor enwiki-latest-pages-articles.xml.bz2 -o wiki_text
  echo "Done. Tokenize wiki_text/*/wiki_* into ../data/ as desired."
else
  echo "wikiextractor not installed. Run:  pip install wikiextractor"
  echo "then:  wikiextractor enwiki-latest-pages-articles.xml.bz2 -o wiki_text"
fi
