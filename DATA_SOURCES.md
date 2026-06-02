# Data sources & licenses

AlphaWord Blocks designs a set of letter blocks so that a large corpus of
words is spellable. It needs word lists as input. This file documents the
provenance and license of every corpus, both the lists **bundled** in this
repository and the lists you can **fetch on demand** with the scripts in
`download_scripts/`.

The program code is licensed GPLv3 (see `LICENSE`). The data files have their
own licenses, summarized below. Only public-domain or permissively-licensed
lists that are redistributable alongside GPLv3 code are bundled here.

## Bundled in `data/`

| File | Source | License | Notes |
|------|--------|---------|-------|
| `webster.txt` | Webster's Unabridged Dictionary (1913), via Project Gutenberg | Public domain | Headwords tokenized to lowercase letters. |
| `enable1.txt` | ENABLE (Enhanced North American Benchmark LExicon) | Public domain | Released to the public domain by its author. |
| `wordnet.txt` | Princeton WordNet lemmas | WordNet License (permissive, BSD-style) | Redistribution allowed with the copyright/permission notice retained; see below. |
| `nltk_words.txt` | The Unix `words` list (as distributed with NLTK) | Public domain | The traditional `/usr/share/dict/words`-style list. |
| `blocked.txt` | This project | GPLv3 (part of this program) | Optional corpus blocklist. **Ships empty by design** — add your own tokens to exclude them from the corpus. Not third-party data. |

### WordNet notice

WordNet is distributed by Princeton University under a permissive license:

> WordNet Release 3.0 Copyright 2006 by Princeton University. All rights
> reserved. Permission to use, copy, modify and distribute this software and
> database and its documentation for any purpose and without fee or royalty
> is hereby granted, provided that you agree to comply with the following
> copyright notice and statements... THE SOFTWARE AND DATABASE IS PROVIDED
> "AS IS" AND PRINCETON UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES...

If you redistribute `wordnet.txt`, retain this notice. See
<https://wordnet.princeton.edu/license-and-commercial-use>.

## Fetched on demand via `download_scripts/`

These corpora are **not** redistributed here — either because of their size or
because their licenses require attribution that is cleaner to honor at the
point of download. Each script documents its source and license.

| Script | Source | License | Redistribution |
|--------|--------|---------|----------------|
| `download_tatoeba.sh` | Tatoeba English sentence export | CC-BY 2.0 FR | Attribution required if you redistribute the cleaned result. |
| `download_wiktionary.sh` | English Wiktionary page titles (ns0 dump) | CC-BY-SA 4.0 (+ GFDL) | Share-alike attribution required. |
| `download_wikipedia.sh` | English Wikipedia article dump | CC-BY-SA 4.0 (+ GFDL) | Share-alike attribution required; very large, gated behind `--download`. |
| `download_words_alpha.sh` | dwyl/english-words `words_alpha.txt` | The Unlicense (public domain) | Freely redistributable. |
| `download_scowl.sh` | SCOWL (Spell Checker Oriented Word Lists) | Permissive (SCOWL/aspell terms) | Redistributable with the SCOWL copyright notice. |

`download_scripts/filtered_sentences.py` is the cleaner used by
`download_tatoeba.sh`: it drops citation-, reference-, and proper-noun-looking
lines, then normalizes sentences into corpus tokens. Its optional word lists
ship empty by design; you can add your own terms.

## Adding a fetched corpus to the build

After running a download script and producing a token file, copy it into
`data/` and add its filename to the corpus load lists in:

- `src/interactive.rs` (the optimizer), and/or
- `check_spellable.jl` (the verifier).

## Deliberately excluded

Copyrighted word lists (e.g. Scrabble TWL06 / SOWPODS) and any list whose
provenance could not be verified are intentionally **not** included and have
no download script, because they cannot be redistributed alongside GPLv3 code.
