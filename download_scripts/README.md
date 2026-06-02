# download_scripts

Helpers that fetch additional word/sentence corpora that are **not** bundled
with this repository (because of size or attribution/share-alike terms). See
`../DATA_SOURCES.md` for the full provenance and license table.

Each script is self-documenting (run it with no arguments, or read its header)
and writes its output next to itself. After producing a token file, copy it
into `../data/` and add its filename to the corpus load list in
`../src/interactive.rs` and/or `../check_spellable.jl`.

## Scripts

| Script | Fetches | License | How to run |
|--------|---------|---------|------------|
| `download_tatoeba.sh` | Tatoeba English sentences, cleaned into corpus tokens | CC-BY 2.0 FR | `./download_tatoeba.sh` (downloads, decompresses, runs `filtered_sentences.py`). |
| `download_words_alpha.sh` | dwyl/english-words `words_alpha.txt` | Unlicense (public domain) | `./download_words_alpha.sh` |
| `download_wiktionary.sh` | English Wiktionary page titles (ns0) | CC-BY-SA 4.0 | `./download_wiktionary.sh`, then lowercase / strip non-letters / drop multi-word titles. |
| `download_wikipedia.sh` | English Wikipedia article dump → plain text | CC-BY-SA 4.0 | `./download_wikipedia.sh --download` (very large; needs `wikiextractor`). |
| `download_scowl.sh` | Instructions to build a word list from SCOWL | Permissive (SCOWL terms) | `./download_scowl.sh` prints the steps (SCOWL releases are versioned). |

## `filtered_sentences.py`

The cleaner invoked by `download_tatoeba.sh`. It reads the Tatoeba TSV export
(`eng_sentences.tsv`) and writes normalized corpus tokens to
`tatoeba_clean.txt`, dropping:

- citation-, reference-, acronym-, and title-case-looking lines;
- lines with internal capitalized proper nouns;
- anything longer than the normalized length cap;
- sentences matched by your own word lists (empty by default — see below).

The optional `remove_names`, `blocklist`, and `DEMONYMS_CAP` sets near the top
of the script ship empty; populate them with your own terms if you want to
drop sentences that mention them.

## Attribution

If you redistribute any result derived from these sources, honor the source
license: attribute Tatoeba (CC-BY 2.0 FR), Wiktionary/Wikipedia (CC-BY-SA 4.0),
and SCOWL (Kevin Atkinson) as required. `words_alpha.txt` is public domain.
