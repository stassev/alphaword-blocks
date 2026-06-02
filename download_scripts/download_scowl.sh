#!/usr/bin/env bash
# AlphaWord Blocks — instructions for building an English word list from SCOWL.
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
# SCOWL (Spell Checker Oriented Word Lists) is distributed under a permissive
# BSD-like license; attribution to Kevin Atkinson is required. SCOWL releases
# are versioned, so this script documents the steps rather than guessing a URL.
set -euo pipefail

cat <<'EOF'
To build "word.list" from SCOWL:

  1. Download the latest SCOWL release from the official project page:
         http://wordlist.aspell.net/
     (versioned tarball, e.g. scowl-<date>.tar.gz).

  2. Unpack it and build a word list with SCOWL's generator, for example:
         tar xf scowl-*.tar.gz && cd scowl-*
         ./mk-list en 60 > word.list
     (tune the size parameter, e.g. 35/50/60/70, to taste).

  3. Copy word.list into ../data/ and add "word.list" to the corpus load
     list in src/interactive.rs and/or check_spellable.jl.

SCOWL Copyright (c) Kevin Atkinson; see SCOWL's own Copyright file for the
exact terms and required attribution.
EOF
