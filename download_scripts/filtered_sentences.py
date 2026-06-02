# AlphaWord Blocks — Tatoeba sentence cleaner / corpus normalizer
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

import re
import csv
import string

# =========================
# Name / placeholder blocklists applied during corpus cleaning
# =========================

# These content blocklists ship EMPTY by design. This project bundles no
# opinionated word lists; the cleaner stays content-neutral out of the box.
# Populate the sets below with your own lowercase terms to drop sentences
# that mention them.

# Whole tokens stripped from each sentence before analysis (e.g. placeholder
# names). Add your own.
remove_names: set[str] = set()

# Sentences containing any of these lowercase tokens are dropped. Add your own.
blocklist: set[str] = set()

def clean_text(text: str) -> str:
    for name in remove_names:
        text = re.sub(rf'\b{name}\b', '', text, flags=re.IGNORECASE)
    text = re.sub(r'[^a-zA-Z\n]', ' ', text).lower()
    text = re.sub(r'\s+', ' ', text).strip()
    return text

def contains_blocked_words(text: str) -> bool:
    words = set(re.findall(r'\b[a-z]+\b', text))
    return any(w in blocklist for w in words)

# ===============================
# Structural/format filters added
# ===============================

# Capitalized-demonym list, also empty by design. Add your own.
DEMONYMS_CAP: set[str] = set()

JOURNAL_HINTS = {
    "Biometrics", "Diabetes", "Lancet", "Nutrition and Cancer",
    "Clin Orthop Relat Res", "Agroforestry Systems"
}
PUBLISHER_HINTS = {
    "Sons", "Press", "Publishers", "Furlanetto", "Wikimedia", "Commons"
}

YEAR_RE = re.compile(r"\b(18|19|20)\d{2}\b")
VOLUME_ISSUE_RE = re.compile(r"\b\d+\s*\(\d+\)\b")
PAGES_RE = re.compile(r"\b\d+\s*-\s*\d+\b")
COLON_PAGES_YEAR_RE = re.compile(r"\b\d+:\d+\s*-\s*\d+,\s*(18|19|20)\d{2}\b")
SEMI_VOL_PAGES_RE = re.compile(r"\b(18|19|20)\d{2};\s*\d+\s*\(\d+\):\s*\d+")

ACRONYM_RE = re.compile(r"\b[A-Z]{2,}\b")
WORD_WITH_DOT_RE = re.compile(r"\b[A-Za-z]{1,4}\.\b")
PP_RE = re.compile(r"\bpp\.\b", re.IGNORECASE)
FIG_RE = re.compile(r"\bfigs?\.\b", re.IGNORECASE)

ARGUED_RE = re.compile(r"\bArgued\s+[A-Z][a-z]+\s+\d", re.IGNORECASE)
DOMAIN_RE = re.compile(r"\b[A-Za-z0-9\-]+\.[A-Za-z]{2,}\b")

def tokenize_words(line: str):
    return re.findall(r"[A-Za-z']+", line)

def has_demonym_cap(tokens):
    return any(t in DEMONYMS_CAP for t in tokens)

def has_journal_hint(line: str) -> bool:
    return any(hint in line for hint in JOURNAL_HINTS)

def has_publisher_hint(line: str) -> bool:
    return any(hint in line for hint in PUBLISHER_HINTS)

def looks_like_reference(line: str) -> bool:
    if has_journal_hint(line):
        return True
    if COLON_PAGES_YEAR_RE.search(line):
        return True
    if SEMI_VOL_PAGES_RE.search(line):
        return True
    if VOLUME_ISSUE_RE.search(line) and PAGES_RE.search(line) and YEAR_RE.search(line):
        return True
    if YEAR_RE.search(line) and has_publisher_hint(line):
        return True
    if "New York:" in line or "Venice," in line:
        return True
    return False

def has_abbrev_pattern(line: str) -> bool:
    return PP_RE.search(line) or FIG_RE.search(line) or WORD_WITH_DOT_RE.search(line)

def has_acronym(line: str) -> bool:
    return bool(ACRONYM_RE.search(line))

def looks_like_title_case(line: str) -> bool:
    tokens = tokenize_words(line)
    if len(tokens) <= 1:
        return False
    cap_count = sum(1 for t in tokens if t[0].isupper())
    return cap_count >= max(2, len(tokens) // 2)

def has_internal_proper_noun(line: str) -> bool:
    tokens = tokenize_words(line)
    if not tokens or len(tokens) == 1:
        return False
    for i, tok in enumerate(tokens):
        if not tok[0].isupper():
            continue
        if i == 0:
            if tok.isupper() and len(tok) > 1:
                return True
            continue
        if tok == "I":
            continue
        return True
    return False

def looks_like_citation_or_legal(line: str) -> bool:
    if ARGUED_RE.search(line):
        return True
    if YEAR_RE.search(line) and "," in line and any(m in line for m in ("v.", "vs.", "Court", "Justices")):
        return True
    return False

def has_domain_like(line: str) -> bool:
    return bool(DOMAIN_RE.search(line))

def should_drop_structural(line: str) -> bool:
    stripped = line.strip()
    if not stripped:
        return True
    tokens = tokenize_words(stripped)
    if has_acronym(stripped):
        return True
    if has_abbrev_pattern(stripped):
        return True
    if has_demonym_cap(tokens):
        return True
    if has_domain_like(stripped):
        return True
    if looks_like_reference(stripped):
        return True
    if looks_like_citation_or_legal(stripped):
        return True
    if looks_like_title_case(stripped):
        return True
    if has_internal_proper_noun(stripped):
        return True
    return False

def sentence_is_ok(raw_sentence: str) -> bool:
    if should_drop_structural(raw_sentence):
        return False
    cleaned = clean_text(raw_sentence)
    if not cleaned:
        return False
    if contains_blocked_words(cleaned):
        return False
    return True

# ==========================
# NORMALIZE, FILTER BY LENGTH
# ==========================

Nlength = 25  # max characters after removing punctuation and spaces

# translation table to remove punctuation and spaces
_remove_punct_space = str.maketrans('', '', string.punctuation + ' ' + '0123456789')

def normalize_for_length(s: str) -> str:
    """Remove all punctuation and spaces, keep letters/digits, lowercased."""
    s = s.strip()
    s = s.translate(_remove_punct_space)
    return s

def process_file_tsv(in_path: str):
    candidates = []
    with open(in_path, newline='', encoding='utf-8') as f:
        reader = csv.reader(f, delimiter='\t')
        for row in reader:
            if len(row) < 3:
                continue
            raw_text = row[2]
            if not sentence_is_ok(raw_text):
                continue
            norm = normalize_for_length(raw_text)
            if not norm:
                continue
            if len(norm) > Nlength:
                continue
            candidates.append((len(norm), norm, raw_text.strip()))

    # sort by normalized length, then lexicographically for stability
    candidates.sort(key=lambda x: (x[0], x[1]))

    for length, norm, original in candidates:
        print(f"{length:2d} | {norm} | {original}")

def filter_plain_text(in_path: str, out_path: str):
    candidates = []
    with open(in_path, encoding="utf-8") as fin:
        for line in fin:
            if not sentence_is_ok(line):
                continue
            norm = normalize_for_length(line)
            if not norm:
                continue
            if len(norm) > Nlength:
                continue
            candidates.append((len(norm), norm, line.strip()))

    candidates.sort(key=lambda x: (x[0], x[1]))

    with open(out_path, "w", encoding="utf-8") as fout:
        for _, norm, original in candidates:
            fout.write(norm + "\n")  # or write original if you prefer

def write_tsv_corpus(in_path: str, out_path: str):
    """Clean a Tatoeba TSV export into normalized corpus tokens (one per line)."""
    candidates = []
    with open(in_path, newline="", encoding="utf-8") as f:
        reader = csv.reader(f, delimiter="\t")
        for row in reader:
            if len(row) < 3:
                continue
            raw_text = row[2]
            if not sentence_is_ok(raw_text):
                continue
            norm = normalize_for_length(raw_text)
            if not norm or len(norm) > Nlength:
                continue
            candidates.append((len(norm), norm))
    candidates.sort(key=lambda x: (x[0], x[1]))
    seen = set()
    with open(out_path, "w", encoding="utf-8") as fout:
        for _, norm in candidates:
            if norm not in seen:
                seen.add(norm)
                fout.write(norm + "\n")


if __name__ == "__main__":
    # Clean the Tatoeba English sentence export (fetched by download_tatoeba.sh)
    # into a normalized, name/citation-filtered corpus usable by AlphaWord Blocks.
    # Copy the result into ../data/ and add it to the corpus load list.
    write_tsv_corpus("eng_sentences.tsv", "tatoeba_clean.txt")
