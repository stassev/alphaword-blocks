// AlphaWord Blocks — glyph configuration
// Copyright (C) 2025- Svetlin Tassev
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// src/glyph_config.rs
//! Config loader + plan builder for glyph kits → (tokens, rewrites).
//! (unchanged comments shortened)

use anyhow::{bail, Context, Result};
use std::collections::{BTreeSet, HashSet};
use unicode_normalization::UnicodeNormalization;
use strum::IntoEnumIterator;

use crate::glyph_db::{Package, PRESETS};
use crate::glyph_db::sets as S;

/* ------------------------------ Config types ------------------------------ */

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub presets: Vec<String>,
    #[serde(default)]
    pub packages: Vec<String>,
    #[serde(default)]
    pub options: Options,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Options {
    pub case_mode: String,                    // "CapsOnly" | "UpperLower" | "LowerOnly"
    pub drop_diacritics: bool,

    // RENAMED: soft_diacritic_bridges -> language_specific_bridges
    pub language_specific_bridges: bool,

    // NEW: signage transliteration (ä/ö/ü→ae/oe/ue, ß→ss, etc.)
    pub signage_translits: bool,

    pub keep_ligatures: bool,
    pub rewrite_with_typographic_ligatures: bool,
    pub german_sharp_s_uppercase: String,     // "SS" | "ẞ"
}

impl Default for Options {
    fn default() -> Self {
        Self {
            case_mode: "UpperLower".to_string(),
            drop_diacritics: false,
            language_specific_bridges: true,   // same semantics as the former "soft" default
            signage_translits: false,          // default OFF to preserve prior default behavior
            keep_ligatures: true,
            rewrite_with_typographic_ligatures: false,
            german_sharp_s_uppercase: "ẞ".to_string(),
        }
    }
}

/* --------------------- Package name string ↔ enum mapping --------------------- */

fn package_by_name(s: &str) -> Option<Package> {
    use Package::*;
    Some(match s {
        // Latin
        "latin.ascii.upper" => LatinAsciiUpper,
        "latin.ascii.lower" => LatinAsciiLower,
        "spanish.core.upper" => SpanishCoreUpper,
        "spanish.core.lower" => SpanishCoreLower,
        "french.accents.upper" => FrenchAccentsUpper,
        "french.accents.lower" => FrenchAccentsLower,
        "german.extras.upper" => GermanExtrasUpper,
        "german.extras.lower" => GermanExtrasLower,
        "italian.core.upper" => ItalianCoreUpper,
        "italian.core.lower" => ItalianCoreLower,
        "portuguese.core.upper" => PortugueseCoreUpper,
        "portuguese.core.lower" => PortugueseCoreLower,
        "danish_norwegian.upper" => DanishNorwegianUpper,
        "danish_norwegian.lower" => DanishNorwegianLower,
        "swedish.upper" => SwedishUpper,
        "swedish.lower" => SwedishLower,
        "icelandic.upper" => IcelandicUpper,
        "icelandic.lower" => IcelandicLower,
        "lithuanian.upper" => LithuanianUpper,
        "lithuanian.lower" => LithuanianLower,
        "latvian.upper" => LatvianUpper,
        "latvian.lower" => LatvianLower,
        "czech.upper" => CzechUpper,
        "czech.lower" => CzechLower,
        "slovak.upper" => SlovakUpper,
        "slovak.lower" => SlovakLower,
        "slovene_bcs.upper" => SloveneBCSUpper,
        "slovene_bcs.lower" => SloveneBCSLower,
        "hungarian.upper" => HungarianUpper,
        "hungarian.lower" => HungarianLower,
        "polish.upper" => PolishUpper,
        "polish.lower" => PolishLower,
        "romanian.upper" => RomanianUpper,
        "romanian.lower" => RomanianLower,
        "welsh.upper" => WelshUpper,
        "welsh.lower" => WelshLower,
        "gaelic.upper" => GaelicUpper,
        "gaelic.lower" => GaelicLower,
        "maltese.upper" => MalteseUpper,
        "maltese.lower" => MalteseLower,
        "turkish_azeri.upper" => TurkishAzeriUpper,
        "turkish_azeri.lower" => TurkishAzeriLower,
        "greenlandic.upper" => GreenlandicUpper,
        "greenlandic.lower" => GreenlandicLower,
        "latin.ligatures.upper" => LatinLigaturesUpper,
        "latin.ligatures.lower" => LatinLigaturesLower,

        // Latin digraphs
        "latin.digraphs.common" => LatinDigraphsCommon,
        "latin.digraphs.catalan" => LatinDigraphsCatalan,
        "latin.digraphs.maltese" => LatinDigraphsMaltese,
        "latin.digraphs.all" => LatinDigraphsAll,

        // Latin extensions
        "latin.ipa.core"         => LatinIpaCore,
        "latin.ipa.modifiers"    => LatinIpaModifiers,
        "latin.clicks"           => LatinClicks,

        // Split Vietnamese / Esperanto / Yoruba / Kikuyu
        "vietnamese.upper"        => VietnameseUpper,
        "vietnamese.lower"        => VietnameseLower,
        "esperanto.upper"         => EsperantoUpper,
        "esperanto.lower"         => EsperantoLower,
        "yoruba.upper"            => YorubaUpper,
        "yoruba.lower"            => YorubaLower,
        "kikuyu.upper"            => KikuyuUpper,
        "kikuyu.lower"            => KikuyuLower,

        // Split Salish / Ogonek / Romanian legacy / Africanist
        "latin.salish.upper"      => LatinSalishUpper,
        "latin.salish.lower"      => LatinSalishLower,
        "latin.extended.ogonek.upper" => LatinExtendedOgonekUpper,
        "latin.extended.ogonek.lower" => LatinExtendedOgonekLower,
        "romanian.legacy.cedilla.upper" => RomanianLegacyCedillaUpper,
        "romanian.legacy.cedilla.lower" => RomanianLegacyCedillaLower,
        "latin.african.upper"     => LatinAfricanUpper,
        "latin.african.lower"     => LatinAfricanLower,

        // Yoruba tone sequences split
        "yoruba.tones.upper"      => YorubaTonesUpper,
        "yoruba.tones.lower"      => YorubaTonesLower,

        // Pinyin
        "latin.pinyin.upper"     => LatinPinyinUpper,
        "latin.pinyin.lower"     => LatinPinyinLower,

        // Greek
        "greek.caps" => GreekCaps,
        "greek.lower.base" => GreekLowerBase,
        "greek.monotonic.marks.upper" => GreekMonotonicMarksUpper,
        "greek.monotonic.marks.lower" => GreekMonotonicMarksLower,

        // Cyrillic (including Bulgarian)
        "cyr.ru.caps" => CyrillicRuCaps,
        "cyr.ru.lower" => CyrillicRuLower,
        "cyr.bg.caps" => CyrillicBgCaps,
        "cyr.bg.lower" => CyrillicBgLower,
        "cyr.uk.caps" => CyrillicUkCaps,
        "cyr.uk.lower" => CyrillicUkLower,
        "cyr.be.caps" => CyrillicBeCaps,
        "cyr.be.lower" => CyrillicBeLower,
        "cyr.srme.caps" => CyrillicSrMeCaps,
        "cyr.srme.lower" => CyrillicSrMeLower,
        "cyr.mk.caps" => CyrillicMkCaps,
        "cyr.mk.lower" => CyrillicMkLower,

        // Armenian / Georgian / Hebrew
        "armenian.caps" => ArmenianCaps,
        "armenian.lower" => ArmenianLower,
        "georgian.mkhedruli" => GeorgianMkhedruli,
        "georgian.mtavruli" => GeorgianMtavruli,
        "hebrew.core" => Hebrew,

        // Cherokee
        "cherokee.upper" => CherokeeUpper,
        "cherokee.small" => CherokeeSmall,

        // Canadian Aboriginal Syllabics
        "cas.inuktitut.core" => InuktitutCore,
        "cas.cree.core" => CreeCore,

        // Kana
        "kana.hiragana" => Hiragana,
        "kana.katakana" => Katakana,

        // Hangul Jamo
        "hangul.jamo.L" => HangulJamoL,
        "hangul.jamo.V" => HangulJamoV,
        "hangul.jamo.T" => HangulJamoT,
        "hangul.jamo.all" => HangulJamoAll,
        "hangul.jamo" => HangulJamoAll, // convenience alias

        // Ethiopic
        "ethiopic.core" => EthiopicCore,

        // Arabic & friends
        "arabic.core" => ArabicCore,
        "persian_urdu.extras" => PersianUrduExtras,

        // Syriac / Thaana
        "syriac.core" => SyriacCore,
        "thaana.core" => ThaanaCore,

        // Indic cores
        "devanagari.core" => DevanagariCore,
        "bengali.core" => BengaliCore,
        "gurmukhi.core" => GurmukhiCore,
        "gujarati.core" => GujaratiCore,
        "odia.core" => OdiaCore,
        "tamil.core" => TamilCore,
        "telugu.core" => TeluguCore,
        "kannada.core" => KannadaCore,
        "malayalam.core" => MalayalamCore,
        "sinhala.core" => SinhalaCore,

        // Thai & Lao
        "thai.core" => ThaiCore,
        "lao.core" => LaoCore,

        // Tibetan / Myanmar / Khmer
        "tibetan.core" => TibetanCore,
        "myanmar.core" => MyanmarCore,
        "khmer.core" => KhmerCore,

        // Digits
        "digits.ascii"                    => DigitsAscii,
        "digits.arabic_indic"             => DigitsArabicIndic,
        "digits.extended_arabic_indic"    => DigitsExtendedArabicIndic,
        "digits.devanagari"               => DigitsDevanagari,
        "digits.bengali"                  => DigitsBengali,
        "digits.gurmukhi"                 => DigitsGurmukhi,
        "digits.gujarati"                 => DigitsGujarati,
        "digits.odia"                     => DigitsOdia,
        "digits.tamil"                    => DigitsTamil,
        "digits.telugu"                   => DigitsTelugu,
        "digits.kannada"                  => DigitsKannada,
        "digits.malayalam"                => DigitsMalayalam,
        "digits.sinhala"                  => DigitsSinhala,
        "digits.thai"                     => DigitsThai,
        "digits.lao"                      => DigitsLao,
        "digits.tibetan"                  => DigitsTibetan,
        "digits.myanmar"                  => DigitsMyanmar,
        "digits.khmer"                    => DigitsKhmer,
        "digits.superscript.core"         => DigitsSuperscriptCore,
        "digits.subscript.core"           => DigitsSubscriptCore,

        // Math
        "math.basic_arithmetic"           => MathBasicArithmetic,
        "math.fullwidth.basic_east_asian" => MathFullwidthBasicEastAsian,
        "math.arabic.numeric_symbols"     => MathArabicNumericSymbols,
        "math.typographic.variants"       => MathTypographicVariants,
        "math.algebra_core"               => MathAlgebraCore,
        "math.set_relations"              => MathSetRelations,
        "math.calculus_core"              => MathCalculusCore,
        "math.logic_core"                 => MathLogicCore,
        "math.arrows.basic"               => MathArrowsBasic,
        "math.arrows.extended"            => MathArrowsExtended,
        "math.delimiters.extended"        => MathDelimitersExtended,
        "math.operators.extended"         => MathOperatorsExtended,
        "math.double_struck.sets"         => MathDoubleStruckSets,

        // Punctuation
        "punct.word.basic_latin"          => PunctuationWordBasicLatin,
        "punct.underscore.basic"          => PunctuationUnderscoreBasic,
        "punct.underscore.extended"       => PunctuationUnderscoreExtended,
        "punct.sentence.basic_latin"      => PunctuationSentenceBasicLatin,
        "punct.spanish.extras"            => PunctuationSpanishExtras,
        "punct.greek.ano_teleia"          => PunctuationGreekAnoTeleia,
        "punct.hebrew.core"               => PunctuationHebrewCore,
        "punct.arabic.core"               => PunctuationArabicCore,
        "punct.devanagari.core"           => PunctuationDevanagariCore,
        "punct.japanese.core"             => PunctuationJapaneseCore,
        "punct.thai.core"                 => PunctuationThaiCore,
        "punct.lao.core"                  => PunctuationLaoCore,
        "punct.tibetan.core"              => PunctuationTibetanCore,
        "punct.myanmar.core"              => PunctuationMyanmarCore,
        "punct.khmer.core"                => PunctuationKhmerCore,
        "punct.dashes.basic"              => PunctuationDashesBasic,
        "punct.dashes.extended"           => PunctuationDashesExtended,
        "punct.dashes.all"                => PunctuationDashesAll,

		"symbols.astro.basic"    =>                    SymbolsAstroBasic     ,
        "latin.small.caps.extended"      =>               LatinSmallCapsExtended       ,
        "braille.patterns"               =>BraillePatterns              ,
        "symbols.music.basic"            =>           SymbolsMusicBasic            ,
        "punctuation.currency.extended"  =>                     PunctuationCurrencyExtended  ,
        "punctuation.letterlike.symbols" =>                      PunctuationLetterlikeSymbols ,
        "punctuation.primes.basic"       =>                PunctuationPrimesBasic       ,
		"hawaiian.core.upper"            =>           HawaiianCoreUpper            ,
        "hawaiian.core.lower"            =>           HawaiianCoreLower            ,
        "hawaiian.okina"                 =>      HawaiianOkina                ,
		"punctuation.sections.basic" => PunctuationSectionsBasic,
        _ => return None,
    })
}

fn expand_preset(name: &str) -> Option<(&'static [Package], &'static crate::glyph_db::Preset)> {
    PRESETS.iter().find(|p| p.name == name).map(|p| (p.packages, p))
}

/* ----------------------- Small Unicode utilities ----------------------- */

fn nfc(s: &str) -> String { s.nfc().collect::<String>() }
fn nfd_chars<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a { s.nfd() }

fn is_combining_mark(ch: char) -> bool {
    matches!(
        ch,
        '\u{0300}'..='\u{036F}'
        | '\u{0483}'..='\u{0489}'  
            | '\u{1AB0}'..='\u{1AFF}'
            | '\u{1DC0}'..='\u{1DFF}'
            | '\u{20D0}'..='\u{20FF}'
            | '\u{FE20}'..='\u{FE2F}'
    )
}

fn strip_marks(s: &str) -> String {
    nfd_chars(s).filter(|&c| !is_combining_mark(c)).collect::<String>().nfc().collect()
}

/// Uppercasing that may expand (ß→SS unless caller prefers ẞ in CapsOnly).
fn to_upper(s: &str, german_upper: &str) -> String {
    if s == "ß" && german_upper == "ẞ" { return "ẞ".into(); }
    s.chars().flat_map(|c| c.to_uppercase()).collect()
}
fn to_lower(s: &str) -> String { s.chars().flat_map(|c| c.to_lowercase()).collect() }

/* ---------------------------- Build result ---------------------------- */

#[derive(Debug)]
pub struct Build {
    pub tokens: Vec<String>,
    pub rewrites: Vec<(String, String)>,
    pub warnings: Vec<String>,
}

/* ---------------------------- Bridge helpers ---------------------------- */

fn present(tokens: &BTreeSet<String>, s: &str) -> bool { tokens.contains(s) }

fn push_if_target_present(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    from: &str,
    to: &str,
) {
    if present(tokens, to) && !present(tokens, from) {
        rewrites.push((from.to_string(), to.to_string()));
    }
}

fn push_to_any(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    from: &str,
    candidates: &[&str],
) {
    if present(tokens, from) { return; }
    for &to in candidates {
        if present(tokens, to) {
            rewrites.push((from.to_string(), to.to_string()));
            break;
        }
    }
}

fn push_case_pairs(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    uppers: &[&str],
    lowers: &[&str],
) {
    let n = uppers.len().min(lowers.len());
    for i in 0..n {
        let u = uppers[i];
        let l = lowers[i];
        if present(tokens, u) && !present(tokens, l) {
            rewrites.push((l.to_string(), u.to_string()));
        } else if present(tokens, l) && !present(tokens, u) {
            rewrites.push((u.to_string(), l.to_string()));
        }
    }
}

/* ---------------------------- Bridges per script ---------------------------- */

fn greek_bridges(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    language_specific_bridges: bool
) {
    // Always: map final ς → σ if σ is kept
    push_if_target_present(rewrites, tokens, "ς", "σ");

    // Monotonic precomposed forms only if language-specific pack is enabled
    if !language_specific_bridges { return; }

    if present(tokens, "ΐ") {
        rewrites.push(("ϊ\u{0301}".into(), "ΐ".into()));
        rewrites.push(("ι\u{0308}\u{0301}".into(), "ΐ".into()));
    }
    if present(tokens, "ΰ") {
        rewrites.push(("ϋ\u{0301}".into(), "ΰ".into()));
        rewrites.push(("υ\u{0308}\u{0301}".into(), "ΰ".into()));
    }
}


fn georgian_bridges(rewrites: &mut Vec<(String, String)>, tokens: &BTreeSet<String>) {
    let up = S::georgian_mtavruli;
    let lo = S::georgian_mkhedruli;
    let n = up.len().min(lo.len());
    for i in 0..n {
        let u = up[i]; let l = lo[i];
        if present(tokens, u) && !present(tokens, l) { rewrites.push((l.into(), u.into())); }
        else if present(tokens, l) && !present(tokens, u) { rewrites.push((u.into(), l.into())); }
    }
}

fn hebrew_final_bridges(rewrites: &mut Vec<(String, String)>, tokens: &BTreeSet<String>) {
    for (fin, base) in [("ך","כ"),("ם","מ"),("ן","נ"),("ף","פ"),("ץ","צ")] {
        push_if_target_present(rewrites, tokens, fin, base);
    }
}

fn kana_bridges(rewrites: &mut Vec<(String, String)>, tokens: &BTreeSet<String>) {
    let has_hira = S::hiragana.iter().any(|&h| present(tokens, h));
    let has_kata = S::katakana.iter().any(|&k| present(tokens, k));

    let to_hiragana = |kc: char| -> Option<char> {
        let u = kc as u32;
        if (0x30A1..=0x30F6).contains(&u) { char::from_u32(u - 0x60) } else { None }
    };
    let to_katakana = |hc: char| -> Option<char> {
        let u = hc as u32;
        if (0x3041..=0x3096).contains(&u) { char::from_u32(u + 0x60) } else { None }
    };

    if has_hira && !has_kata {
        for &k in S::katakana {
            if let Some(h) = k.chars().next().and_then(to_hiragana) {
                push_if_target_present(rewrites, tokens, k, &h.to_string());
            }
        }
    } else if has_kata && !has_hira {
        for &h in S::hiragana {
            if let Some(k) = h.chars().next().and_then(to_katakana) {
                push_if_target_present(rewrites, tokens, h, &k.to_string());
            }
        }
    }
}

fn cyrillic_subset_bridges(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    case_mode: &str,
) {
    // Only useful if we actually have any Cyrillic in the kit
    let ru_has_any =
        S::cyrillic_ru_caps.iter().any(|&t| present(tokens, t)) ||
        S::cyrillic_ru_lower.iter().any(|&t| present(tokens, t));
    if !ru_has_any { return; }

    let to_case = |s: &str| -> String {
        match case_mode {
            "LowerOnly" => to_lower(s),
            "CapsOnly"  => to_upper(s, "ẞ"), // safe to reuse helper
            _           => s.to_string(),    // UpperLower: keep as authored
        }
    };

    // Map extended letters to base sequences (digraphs too).
    // We only add a rule when the *source* isn't in the kit.
    // RHS is case-adapted so LowerOnly kits get lowercase digraphs (дж, ль, …), etc.
    for (src, tgt) in [
        // Ukrainian / Belarusian / Serbian / Macedonian helpers
        ("Ґ","Г"),("ґ","г"),("Є","Е"),("є","е"),("І","И"),("і","и"),
        ("Ї","ЙИ"),("ї","йи"),
        ("Ў","У"),("ў","у"),
        ("Ј","Й"),("ј","й"),
        ("Љ","ЛЬ"),("љ","ль"),
        ("Њ","НЬ"),("њ","нь"),
        ("Ђ","ДЬ"),("ђ","дь"),
        ("Ћ","ТЬ"),("ћ","ть"),
        ("Џ","ДЖ"),("џ","дж"),
        ("Ѓ","ГЬ"),("ѓ","гь"),
        ("Ќ","КЬ"),("ќ","кь"),
        ("Ѕ","З"),("ѕ","з"),

        // Bulgarian historic gravis (needed for your BG data)
        ("Ѐ","Е"),("ѐ","е"),
        ("Ѝ","И"),("ѝ","и"),
    ] {
        if !present(tokens, src) {
            rewrites.push((src.to_string(), to_case(tgt)));
        }
    }
}

fn latin_accent_bridges(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    blocked_sources: &BTreeSet<String>,
) {
    let map_upper = |rw: &mut Vec<(String, String)>, acc: &str, base_u: &str, base_l: &str| {
        if tokens.contains(acc) || blocked_sources.contains(acc) { return; }
        if tokens.contains(base_u) { rw.push((acc.into(), base_u.into())); }
        else if tokens.contains(base_l) { rw.push((acc.into(), base_l.into())); }
    };
    let map_lower = |rw: &mut Vec<(String, String)>, acc: &str, base_l: &str, base_u: &str| {
        if tokens.contains(acc) || blocked_sources.contains(acc) { return; }
        if tokens.contains(base_l) { rw.push((acc.into(), base_l.into())); }
        else if tokens.contains(base_u) { rw.push((acc.into(), base_u.into())); }
    };

    // UPPER
    for (a,u,l) in [
        ("À","A","a"),("Á","A","a"),("Â","A","a"),("Ã","A","a"),("Ä","A","a"),
        ("Ā","A","a"),("Ă","A","a"),("Ą","A","a"),("Å","A","a"),
        ("Ç","C","c"),("Ć","C","c"),("Ĉ","C","c"),("Ċ","C","c"),("Č","C","c"),
        ("Ď","D","d"),("Đ","D","d"),
        ("È","E","e"),("É","E","e"),("Ê","E","e"),("Ë","E","e"),
        ("Ē","E","e"),("Ė","E","e"),("Ę","E","e"),("Ě","E","e"),
        ("Ĝ","G","g"),("Ğ","G","g"),("Ġ","G","g"),("Ģ","G","g"),
        ("Ĥ","H","h"),("Ħ","H","h"),
        ("Ì","I","i"),("Í","I","i"),("Î","I","i"),("Ï","I","i"),
        ("Ī","I","i"),("Į","I","i"),("Ĩ","I","i"),
        ("Ĵ","J","j"),
        ("Ķ","K","k"),
        ("Ĺ","L","l"),("Ļ","L","l"),("Ľ","L","l"),("Ł","L","l"),
        ("Ñ","N","n"),("Ń","N","n"),("Ņ","N","n"),("Ň","N","n"),
        ("Ò","O","o"),("Ó","O","o"),("Ô","O","o"),("Õ","O","o"),
        ("Ö","O","o"),("Ō","O","o"),("Ő","O","o"),("Ø","O","o"),
        ("Ŕ","R","r"),("Ř","R","r"),("Ŗ","R","r"),
        ("Ś","S","s"),("Ŝ","S","s"),("Ş","S","s"),("Š","S","s"),
        ("Ţ","T","t"),("Ť","T","t"),("Ŧ","T","t"),
        ("Ù","U","u"),("Ú","U","u"),("Û","U","u"),("Ü","U","u"),
        ("Ū","U","u"),("Ů","U","u"),("Ų","U","u"),("Ű","U","u"),("Ũ","U","u"),
        ("Ý","Y","y"),("Ÿ","Y","y"),
        ("Ź","Z","z"),("Ż","Z","z"),("Ž","Z","z"),
        ("Ǫ","O","o"),("Ḵ","K","k"),
    ] { map_upper(rewrites, a, u, l); }

    // lower
    for (a,l,u) in [
        ("à","a","A"),("á","a","A"),("â","a","A"),("ã","a","A"),("ä","a","A"),
        ("ā","a","A"),("ă","a","A"),("ą","a","A"),("å","a","A"),
        ("ç","c","C"),("ć","c","C"),("ĉ","c","C"),("ċ","c","C"),("č","c","C"),
        ("ď","d","D"),("đ","d","D"),
        ("è","e","E"),("é","e","E"),("ê","e","E"),("ë","e","E"),
        ("ē","e","E"),("ė","e","E"),("ę","e","E"),("ě","e","E"),
        ("ĝ","g","G"),("ğ","g","G"),("ġ","g","G"),("ģ","g","G"),
        ("ĥ","h","H"),("ħ","h","H"),
        ("ì","i","I"),("í","i","I"),("î","i","I"),("ï","i","I"),
        ("ī","i","I"),("į","i","I"),("ĩ","i","I"),
        ("ı","i","I"),
        ("ĵ","j","J"),
        ("ķ","k","K"),
        ("ĺ","l","L"),("ļ","l","L"),("ľ","l","L"),("ł","l","L"),
        ("ñ","n","N"),("ń","n","N"),("ņ","n","N"),("ň","n","N"),
        ("ŋ","n","N"),
        ("ò","o","O"),("ó","o","O"),("ô","o","O"),("õ","o","O"),
        ("ö","o","O"),("ō","o","O"),("ő","o","O"),("ø","o","O"),
        ("ŕ","r","R"),("ř","r","R"),("ŗ","r","R"),
        ("ś","s","S"),("ŝ","s","S"),("ş","s","S"),("š","s","S"),
        ("ţ","t","T"),("ť","t","T"),("ŧ","t","T"),
        ("ù","u","U"),("ú","u","U"),("û","u","U"),("ü","u","U"),
        ("ū","u","U"),("ů","u","U"),("ų","u","U"),("ű","u","U"),("ũ","u","U"),
        ("ý","y","Y"),("ÿ","y","Y"),
        ("ź","z","Z"),("ż","z","Z"),("ž","z","Z"),
        ("č","c","C"),("ď","d","D"),("ě","e","E"),("ň","n","N"),
        ("ř","r","R"),("š","s","S"),("ť","t","T"),("ů","u","U"),("ž","z","Z"),
        ("ǫ","o","O"),("ḵ","k","K"),
    ] { map_lower(rewrites, a, l, u); }

    if tokens.contains("ẞ") && !tokens.contains("ß") { rewrites.push(("ß".into(),"ẞ".into())); }
    if tokens.contains("ß") && !tokens.contains("ẞ") { rewrites.push(("ẞ".into(),"ß".into())); }
}

/* ---------------------- Signage-style translit pack ---------------------- */
fn signage_translits(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,   // <-- added
    has_lat_upper: bool,
    has_lat_lower: bool,
) {
    #[inline]
    fn emit_bicase(
        rewrites: &mut Vec<(String, String)>,
        tokens: &BTreeSet<String>,
        has_lat_upper: bool,
        has_lat_lower: bool,
        lhs_lo: &str, lhs_up: &str, rhs_lo: &str, rhs_up: &str,
    ) {
        // Do not transliterate if either source form is kept
        if present(tokens, lhs_lo) || present(tokens, lhs_up) { return; }

        match (has_lat_upper, has_lat_lower) {
            (true,  true ) => {
                rewrites.push((lhs_lo.into(), rhs_lo.into()));
                rewrites.push((lhs_up.into(), rhs_up.into()));
            }
            (true,  false) => {
                rewrites.push((lhs_lo.into(), rhs_up.into()));
                rewrites.push((lhs_up.into(), rhs_up.into()));
            }
            (false, true ) => {
                rewrites.push((lhs_lo.into(), rhs_lo.into()));
                rewrites.push((lhs_up.into(), rhs_lo.into()));
            }
            (false, false) => {}
        }
    }

    #[inline]
    fn emit_mono(
        rewrites: &mut Vec<(String, String)>,
        tokens: &BTreeSet<String>,
        has_lat_upper: bool,
        has_lat_lower: bool,
        lhs: &str, rhs_lo: &str, rhs_up: &str,
    ) {
        // Do not transliterate if source is kept
        if present(tokens, lhs) { return; }

        match (has_lat_upper, has_lat_lower) {
            (false, false) => {}
            (_,     true ) => rewrites.push((lhs.into(), rhs_lo.into())),
            (true,  false) => rewrites.push((lhs.into(), rhs_up.into())),
        }
    }

    // Ligatures & specials
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "æ","Æ","ae","AE");
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "œ","Œ","oe","OE");
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "ß","ẞ","ss","SS");
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "ĳ","Ĳ","ij","IJ");

    // Germanic vowels → ae/oe/ue
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "ä","Ä","ae","AE");
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "ö","Ö","oe","OE");
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "ü","Ü","ue","UE");

    // Icelandic / Old English
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "þ","Þ","th","TH");
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "ð","Ð","th","TH");

    // IPA / specials
    emit_mono(rewrites, tokens, has_lat_upper, has_lat_lower, "ɶ","oe","OE");
    emit_mono(rewrites, tokens, has_lat_upper, has_lat_lower, "ɡ","g","G");
    emit_mono(rewrites, tokens, has_lat_upper, has_lat_lower, "ə","e","E");
    emit_mono(rewrites, tokens, has_lat_upper, has_lat_lower, "ʷ","w","W");
    
    emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "a\u{0308}","A\u{0308}","ae","AE");
	emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "o\u{0308}","O\u{0308}","oe","OE");
	emit_bicase(rewrites, tokens, has_lat_upper, has_lat_lower, "u\u{0308}","U\u{0308}","ue","UE");
}
    /* IPA vowels */
    //emit("ɐ","a","A");
    //emit("ɑ","a","A");
    //emit("ɒ","o","O");
    //emit("ə","e","E");
    //emit("ɚ","er","ER");
    //emit("ɜ","e","E");
    //emit("ɞ","oe","OE");
    //emit("ɛ","e","E");
    //emit("ɘ","e","E");
    //emit("ɵ","o","O");
    //emit("ɤ","o","O");
    //emit("ɔ","o","O");
    //emit("ʌ","u","U");
    //emit("ɯ","u","U");
    //emit("ʊ","u","U");
    //emit("ɨ","i","I");
    //emit("ʉ","u","U");
    //emit("ɪ","i","I");
    //emit("ʏ","y","Y");
	//
    //* IPA fricatives & friends */
    //emit("β","v","V");
    //emit("θ","th","TH");
    //emit("ð","th","TH");
    //emit("ʃ","sh","SH");
    //emit("ʒ","zh","ZH");
    //emit("ʂ","sh","SH");
    //emit("ʐ","zh","ZH");
    //emit("ç","h","H");    // if you prefer, swap to "ch"
    //emit("ʝ","y","Y");
    //emit("ɣ","gh","GH");
    //emit("χ","kh","KH");
    //emit("ʁ","r","R");
    //emit("ħ","h","H");
    //rewrites.push(("ʕ".into(), "'".into())); // pharyngeal → apostrophe
    //emit("ɦ","h","H");
    //emit("ɸ","f","F");
	//
    //* Approximants / liquids */
    //emit("ʍ","wh","WH");
    //emit("ɥ","y","Y");
    //emit("ʋ","v","V");
    //emit("ɹ","r","R");
    //emit("ɾ","r","R");
    //emit("ɽ","r","R");
    //emit("ɻ","r","R");
    //emit("ʀ","r","R");
    //emit("ɫ","l","L");
    //emit("ʎ","ly","LY");
    //emit("ɭ","l","L");
    //emit("ɮ","lz","LZ");
    //emit("ɬ","hl","HL"); // Welsh-style could be "ll" if you prefer
	//
    //* Nasals */
    //emit("ɱ","m","M");
    //emit("ɲ","ny","NY");
    //emit("ɳ","n","N");
    //emit("ŋ","ng","NG");
    //emit("ɴ","n","N");
	//
    //* Stops & misc. */
    //emit("ʈ","t","T");
    //emit("ɖ","d","D");
    //emit("ɟ","j","J");
    //emit("ɢ","g","G");
    //emit("ɡ","g","G");    // IPA "script g" → ASCII g
    //rewrites.push(("ʔ".into(), "'".into())); // glottal stop
    //rewrites.push(("ʡ".into(), "'".into())); // epiglottal stop
    //emit("ʢ","h","H");    // epiglottal fricative
    // Typographic ligatures U+FB00..U+FB04 (single-case) — intentionally disabled
    // Japanese chōonpu can be dropped when mapping to Hiragana-only kits — not here


/* ---------------------- Universal diacritics auto-pack ---------------------- */
fn append_auto_diacritic_pack_case_strict(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    blocked_sources: &BTreeSet<String>,
) {
    const RANGES: &[(u32, u32)] = &[
        (0x00C0, 0x00FF),
        (0x0100, 0x017F),
        (0x0180, 0x024F),
        (0x1E00, 0x1EFF),
        (0x0400, 0x04FF),
        (0x0500, 0x052F),
    ];

    let mut push_case_preferring_match = |lhs: &str, base: &str| {
        if base.is_empty() || base == lhs { return; }
        if tokens.contains(lhs) || blocked_sources.contains(lhs) { return; } // NEW

        let ch = lhs.chars().next().unwrap_or_default();
        let up = to_upper(base, "ẞ");
        let lo = to_lower(base);

        let target = if ch.is_uppercase() {
            if present(tokens, &up) { up }
            else if present(tokens, &lo) { lo }
            else { return; }
        } else {
            if present(tokens, &lo) { lo }
            else if present(tokens, &up) { up }
            else { return; }
        };

        rewrites.push((lhs.to_string(), target));
    };

    for (lo, hi) in RANGES {
        let mut cp = *lo;
        while cp <= *hi {
            if let Some(ch) = char::from_u32(cp) {
                let lhs = ch.to_string();
                let base = strip_marks(&lhs);

                if base != lhs && !base.is_empty() {
                    // NEW: do not generate if we keep the LHS or any case-bridged counterpart
                    if present(tokens, &lhs) { cp += 1; continue; }
                    let lhs_lower = to_lower(&lhs);
                    let lhs_upper = to_upper(&lhs, "ẞ");
                    if present(tokens, &lhs_lower) || present(tokens, &lhs_upper) {
                        cp += 1; continue; // rely on case bridges instead (Й→й), don't strip (Й→и)
                    }

                    push_case_preferring_match(&lhs, &base);
                }
            }
            cp += 1;
        }
    }
}

/* ---------------------- Typographic ligature rules ---------------------- */
fn apply_ligature_rules(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    keep_ligatures: bool,
    rewrite_with_typographic_ligatures: bool,
) {
    if !keep_ligatures {
        // Only decompose if the ligature itself is NOT in the kit
        if !present(tokens, "ﬀ")  { rewrites.push(("ﬀ".into(),  "ff".into())); }
        if !present(tokens, "ﬁ")  { rewrites.push(("ﬁ".into(),  "fi".into())); }
        if !present(tokens, "ﬂ")  { rewrites.push(("ﬂ".into(),  "fl".into())); }
        if !present(tokens, "ﬃ")  { rewrites.push(("ﬃ".into(), "ffi".into())); }
        if !present(tokens, "ﬄ")  { rewrites.push(("ﬄ".into(), "ffl".into())); }
    }

    if keep_ligatures && rewrite_with_typographic_ligatures {
        if present(tokens, "ﬃ") { rewrites.push(("ffi".into(), "ﬃ".into())); }
        if present(tokens, "ﬄ") { rewrites.push(("ffl".into(), "ﬄ".into())); }
        if present(tokens, "ﬀ")  { rewrites.push(("ff".into(),  "ﬀ".into())); }
        if present(tokens, "ﬁ")  { rewrites.push(("fi".into(),  "ﬁ".into())); }
        if present(tokens, "ﬂ")  { rewrites.push(("fl".into(),  "ﬂ".into())); }
    }
}

fn append_combining_strip_rules_for_kept_tokens(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
) {
    // Single-scalar alphabetic bases from the kit
    let mut bases = Vec::<char>::new();
    for t in tokens {
        let mut it = t.chars();
        if let (Some(ch), None) = (it.next(), it.next()) {
            if ch.is_alphabetic() && !is_combining_mark(ch) { bases.push(ch); }
        }
    }

    const COMMON_MARKS: &[char] = &[
        // above
        '\u{0300}','\u{0301}','\u{0302}','\u{0303}','\u{0304}','\u{0306}','\u{0307}','\u{0308}',
        '\u{030A}','\u{030B}','\u{030C}','\u{0311}','\u{031B}',
        // below / overlays
        '\u{0323}','\u{0326}','\u{0327}','\u{0328}','\u{032D}','\u{0331}','\u{0332}',
        '\u{0335}','\u{0336}','\u{0337}','\u{0338}',
        // Cyrillic combining
        '\u{0483}','\u{0484}','\u{0485}','\u{0486}','\u{0487}','\u{0488}','\u{0489}',
    ];

    for b in bases {
        let base_str = b.to_string();
        // NOTE the `&m` pattern: loop variable is a `char`, not `&char`
        for &m in COMMON_MARKS {
            let mut lhs = String::with_capacity(2);
            lhs.push(b);
            lhs.push(m);

            // Skip if (somehow) the LHS itself is in tokens
            if present(tokens, &lhs) { continue; }

            // Prefer composing to a kept precomposed letter, else drop to base.
            let composed = nfc(&lhs);
            if composed != lhs && present(tokens, &composed) {
                rewrites.push((lhs.clone(), composed));
            } else if present(tokens, &base_str) {
                rewrites.push((lhs, base_str.clone()));
            }
        }
    }
}
/* ------------------------- Core builder (public) ------------------------- */
pub fn build_from_toml(toml_src: &str) -> Result<Build> {
    let mut cfg: Config = toml::from_str(toml_src).context("parsing alphabet TOML config")?;

    let user_opts = cfg.options.clone();
    cfg.options = Options::default();

    // Accumulate packages (presets first)
    let mut packages: Vec<Package> = Vec::new();
    let mut warnings = Vec::<String>::new();

    // ===== family helpers (dynamic groups) =====
    let is_digits = |p: &Package| matches!(*p,
        Package::DigitsAscii
        | Package::DigitsArabicIndic
        | Package::DigitsExtendedArabicIndic
        | Package::DigitsDevanagari
        | Package::DigitsBengali
        | Package::DigitsGurmukhi
        | Package::DigitsGujarati
        | Package::DigitsOdia
        | Package::DigitsTamil
        | Package::DigitsTelugu
        | Package::DigitsKannada
        | Package::DigitsMalayalam
        | Package::DigitsSinhala
        | Package::DigitsThai
        | Package::DigitsLao
        | Package::DigitsTibetan
        | Package::DigitsMyanmar
        | Package::DigitsKhmer
        | Package::DigitsSuperscriptCore
        | Package::DigitsSubscriptCore
    );

    let is_math = |p: &Package| matches!(*p,
        Package::MathBasicArithmetic
        | Package::MathFullwidthBasicEastAsian
        | Package::MathArabicNumericSymbols
        | Package::MathTypographicVariants
        | Package::MathAlgebraCore
        | Package::MathSetRelations
        | Package::MathCalculusCore
        | Package::MathLogicCore
        | Package::MathArrowsBasic
        | Package::MathArrowsExtended
        | Package::MathDelimitersExtended
        | Package::MathOperatorsExtended
        | Package::MathDoubleStruckSets
    );

    let is_punct = |p: &Package| matches!(*p,
        Package::PunctuationWordBasicLatin
        | Package::PunctuationUnderscoreBasic
        | Package::PunctuationUnderscoreExtended
        | Package::PunctuationSentenceBasicLatin
        | Package::PunctuationSpanishExtras
        | Package::PunctuationGreekAnoTeleia
        | Package::PunctuationHebrewCore
        | Package::PunctuationArabicCore
        | Package::PunctuationDevanagariCore
        | Package::PunctuationJapaneseCore
        | Package::PunctuationThaiCore
        | Package::PunctuationLaoCore
        | Package::PunctuationTibetanCore
        | Package::PunctuationMyanmarCore
        | Package::PunctuationKhmerCore
        | Package::PunctuationDashesBasic
        | Package::PunctuationDashesExtended
        | Package::PunctuationDashesAll
    );

    // ----- language/script groups -----
    let is_latin = |p: &Package| matches!(*p,
        // ASCII
        Package::LatinAsciiUpper | Package::LatinAsciiLower
        // national sets
        | Package::SpanishCoreUpper | Package::SpanishCoreLower
        | Package::FrenchAccentsUpper | Package::FrenchAccentsLower
        | Package::GermanExtrasUpper | Package::GermanExtrasLower
        | Package::ItalianCoreUpper | Package::ItalianCoreLower
        | Package::PortugueseCoreUpper | Package::PortugueseCoreLower
        | Package::DanishNorwegianUpper | Package::DanishNorwegianLower
        | Package::SwedishUpper | Package::SwedishLower
        | Package::IcelandicUpper | Package::IcelandicLower
        | Package::LithuanianUpper | Package::LithuanianLower
        | Package::LatvianUpper | Package::LatvianLower
        | Package::CzechUpper | Package::CzechLower
        | Package::SlovakUpper | Package::SlovakLower
        | Package::SloveneBCSUpper | Package::SloveneBCSLower
        | Package::HungarianUpper | Package::HungarianLower
        | Package::PolishUpper | Package::PolishLower
        | Package::RomanianUpper | Package::RomanianLower
        | Package::WelshUpper | Package::WelshLower
        | Package::GaelicUpper | Package::GaelicLower
        | Package::MalteseUpper | Package::MalteseLower
        | Package::TurkishAzeriUpper | Package::TurkishAzeriLower
        | Package::GreenlandicUpper | Package::GreenlandicLower
        // digraphs/ligatures/pinyin/IPA/etc.
        | Package::LatinLigaturesUpper | Package::LatinLigaturesLower
        | Package::LatinDigraphsCommon | Package::LatinDigraphsCatalan
        | Package::LatinDigraphsMaltese | Package::LatinDigraphsAll
        | Package::LatinPinyinUpper | Package::LatinPinyinLower
        | Package::LatinIpaCore | Package::LatinIpaModifiers
        | Package::LatinClicks
        | Package::EsperantoUpper | Package::EsperantoLower
        | Package::VietnameseUpper | Package::VietnameseLower
        | Package::YorubaUpper | Package::YorubaLower
        | Package::KikuyuUpper | Package::KikuyuLower
        | Package::LatinSalishUpper | Package::LatinSalishLower
        | Package::LatinExtendedOgonekUpper | Package::LatinExtendedOgonekLower
        | Package::RomanianLegacyCedillaUpper | Package::RomanianLegacyCedillaLower
        | Package::LatinAfricanUpper | Package::LatinAfricanLower
        | Package::YorubaTonesUpper | Package::YorubaTonesLower
    );

    let is_greek = |p: &Package| matches!(*p,
        Package::GreekCaps | Package::GreekLowerBase
        | Package::GreekMonotonicMarksUpper | Package::GreekMonotonicMarksLower
    );

    let is_cyrillic = |p: &Package| matches!(*p,
        Package::CyrillicRuCaps | Package::CyrillicRuLower
        | Package::CyrillicUkCaps | Package::CyrillicUkLower
        | Package::CyrillicBeCaps | Package::CyrillicBeLower
        | Package::CyrillicSrMeCaps | Package::CyrillicSrMeLower
        | Package::CyrillicMkCaps | Package::CyrillicMkLower
    );

    let is_armenian = |p: &Package| matches!(*p, Package::ArmenianCaps | Package::ArmenianLower);
    let is_georgian = |p: &Package| matches!(*p, Package::GeorgianMkhedruli | Package::GeorgianMtavruli);
    let is_hebrew = |p: &Package| matches!(*p, Package::Hebrew);
    let is_cherokee = |p: &Package| matches!(*p, Package::CherokeeUpper | Package::CherokeeSmall);
    let is_cas = |p: &Package| matches!(*p, Package::InuktitutCore | Package::CreeCore);
    let is_kana = |p: &Package| matches!(*p, Package::Hiragana | Package::Katakana);
    let is_hangul_jamo = |p: &Package| matches!(*p, Package::HangulJamoL | Package::HangulJamoV | Package::HangulJamoT | Package::HangulJamoAll);
    let is_ethiopic = |p: &Package| matches!(*p, Package::EthiopicCore);
    let is_arabic_script = |p: &Package| matches!(*p, Package::ArabicCore | Package::PersianUrduExtras);
    let is_syriac = |p: &Package| matches!(*p, Package::SyriacCore);
    let is_thaana = |p: &Package| matches!(*p, Package::ThaanaCore);

    let is_indic = |p: &Package| matches!(*p,
        Package::DevanagariCore | Package::BengaliCore | Package::GurmukhiCore
        | Package::GujaratiCore | Package::OdiaCore | Package::TamilCore
        | Package::TeluguCore | Package::KannadaCore | Package::MalayalamCore
        | Package::SinhalaCore
    );

    let is_thai_lao = |p: &Package| matches!(*p, Package::ThaiCore | Package::LaoCore);
    let is_tibetan = |p: &Package| matches!(*p, Package::TibetanCore);
    let is_myanmar = |p: &Package| matches!(*p, Package::MyanmarCore);
    let is_khmer = |p: &Package| matches!(*p, Package::KhmerCore);

    // --- Special-case: "everything" preset includes every Package variant ---
    let mut use_everything = false;
    if cfg.presets.iter().any(|p| p == "everything") {
        use_everything = true;
        for p in Package::iter() {
            packages.push(p);
        }
    }
    if !use_everything {
        for name in &cfg.presets {
            if let Some((pkgs, preset)) = expand_preset(name) {
                packages.extend_from_slice(pkgs);
                // Merge preset options into cfg.options (builder-time defaults)
                if let Some(v) = preset.drop_diacritics { cfg.options.drop_diacritics = v; }
                if let Some(v) = preset.keep_ligatures { cfg.options.keep_ligatures = v; }
                if let Some(v) = preset.case_mode { cfg.options.case_mode = v.to_string(); }
                if let Some(v) = preset.german_sharp_s_uppercase { cfg.options.german_sharp_s_uppercase = v.to_string(); }
            } else {
                warnings.push(format!("unknown preset '{}'", name));
            }
        }
    }

    for p in &cfg.presets {
        match p.as_str() {
            // === Dynamic “ALL” ===
            "all_packs" | "all_packages" | "everything" => {
                packages.extend(Package::iter());
            }

            // === Family bundles (already present) ===
            "digits_all" | "numbers_all" => {
                packages.extend(Package::iter().filter(is_digits));
            }
            "math_all" => {
                packages.extend(Package::iter().filter(is_math));
            }
            "punct_all" | "punctuation_all" => {
                packages.extend(Package::iter().filter(is_punct));
            }

            // === NEW: Language/script bundles ===
            "latin_all"                 => packages.extend(Package::iter().filter(is_latin)),
            "greek_all"                 => packages.extend(Package::iter().filter(is_greek)),
            "cyrillic_all"              => packages.extend(Package::iter().filter(is_cyrillic)),
            "armenian_all"              => packages.extend(Package::iter().filter(is_armenian)),
            "georgian_all"              => packages.extend(Package::iter().filter(is_georgian)),
            "hebrew_all"                => packages.extend(Package::iter().filter(is_hebrew)),
            "cherokee_all"              => packages.extend(Package::iter().filter(is_cherokee)),
            "canadian_syllabics_all"    => packages.extend(Package::iter().filter(is_cas)),
            "kana_all"                  => packages.extend(Package::iter().filter(is_kana)),
            "hangul_jamo_all"           => packages.extend(Package::iter().filter(is_hangul_jamo)),
            "ethiopic_all"              => packages.extend(Package::iter().filter(is_ethiopic)),
            "arabic_script_all"         => packages.extend(Package::iter().filter(is_arabic_script)),
            "syriac_all"                => packages.extend(Package::iter().filter(is_syriac)),
            "thaana_all"                => packages.extend(Package::iter().filter(is_thaana)),
            "indic_all"                 => packages.extend(Package::iter().filter(is_indic)),
            "thai_lao_all"              => packages.extend(Package::iter().filter(is_thai_lao)),
            "tibetan_all"               => packages.extend(Package::iter().filter(is_tibetan)),
            "myanmar_all"               => packages.extend(Package::iter().filter(is_myanmar)),
            "khmer_all"                 => packages.extend(Package::iter().filter(is_khmer)),

            // Fall back to static PRESETS
            _ => {
                if let Some((pkgs, preset)) = expand_preset(p) {
                    packages.extend_from_slice(pkgs);
                    if let Some(x) = preset.case_mode { cfg.options.case_mode = x.to_string(); }
                    if let Some(x) = preset.drop_diacritics { cfg.options.drop_diacritics = x; }
                    if let Some(x) = preset.keep_ligatures { cfg.options.keep_ligatures = x; }
                    if let Some(x) = preset.german_sharp_s_uppercase {
                        cfg.options.german_sharp_s_uppercase = x.to_string();
                    }
                } else {
                    return Err(anyhow::anyhow!("Unknown preset: {}", p));
                }
            }
        }
    }

    // User options take precedence
    cfg.options = Options {
		case_mode: user_opts.case_mode,
		drop_diacritics: user_opts.drop_diacritics,
		language_specific_bridges: user_opts.language_specific_bridges,
		signage_translits: user_opts.signage_translits,
		keep_ligatures: user_opts.keep_ligatures,
		rewrite_with_typographic_ligatures: user_opts.rewrite_with_typographic_ligatures,
		german_sharp_s_uppercase: user_opts.german_sharp_s_uppercase,
	};


    // Append explicit packages
    for name in &cfg.packages {
        let Some(p) = package_by_name(name) else {
            return Err(anyhow::anyhow!("Unknown package: {}", name));
        };
        packages.push(p);
    }

    // Option sanity
    if cfg.options.case_mode != "CapsOnly" && cfg.options.german_sharp_s_uppercase == "SS" {
        warnings.push("german_sharp_s_uppercase=SS has no effect unless case_mode=CapsOnly".to_string());
    }

    // Include typographic ligatures tokens iff requested & Latin is present
    if cfg.options.keep_ligatures && cfg.options.rewrite_with_typographic_ligatures {
        let seen_latin = packages.iter().any(|p| matches!(p,
            Package::LatinAsciiUpper | Package::LatinAsciiLower |
            Package::FrenchAccentsUpper | Package::FrenchAccentsLower |
            Package::GermanExtrasUpper | Package::GermanExtrasLower |
            Package::ItalianCoreUpper | Package::ItalianCoreLower |
            Package::PortugueseCoreUpper | Package::PortugueseCoreLower |
            Package::LatinLigaturesUpper | Package::LatinLigaturesLower |
            Package::SpanishCoreUpper | Package::SpanishCoreLower
        ));
        if seen_latin {
            packages.push(Package::LatinLigaturesUpper);
            packages.push(Package::LatinLigaturesLower);
        }
    }

    // Build token list
    let mut ordered: Vec<String> = Vec::new();
    for p in packages {
        for &t in p.tokens() { ordered.push(nfc(t)); }
    }

    // Case filtering / expansion
    match cfg.options.case_mode.as_str() {
        "CapsOnly" => {
            let mut only_upper = Vec::with_capacity(ordered.len());
            for t in ordered {
                let up = to_upper(&t, &cfg.options.german_sharp_s_uppercase);
                if up == t { only_upper.push(t); } else { only_upper.push(up); }
            }
            ordered = only_upper;
        }
        "LowerOnly" => {
            let mut only_lower = Vec::with_capacity(ordered.len());
            for t in ordered {
                let lo = to_lower(&t);
                if lo == t { only_lower.push(t); } else { only_lower.push(lo); }
            }
            ordered = only_lower;
        }
        "UpperLower" => { /* keep assembled */ }
        other => bail!("Invalid options.case_mode: {}", other),
    }

    // Dedup, preserving first occurrence
    let mut seen = HashSet::<String>::new();
    let mut tokens: Vec<String> = Vec::new();
    for t in ordered {
        if seen.insert(t.clone()) { tokens.push(t); }
    }

    // Presence set for bridge logic
    let token_set: BTreeSet<String> = tokens.iter().cloned().collect();

    // REWRITES — part 1: configuration-driven policy
    let mut rewrites: Vec<(String, String)> = Vec::new();

    // Latin base presence
	let has_lat_upper = S::latin_ascii_upper.iter().any(|&t| token_set.contains(t));
	let has_lat_lower = S::latin_ascii_lower.iter().any(|&t| token_set.contains(t));

	// Signage transliterations — NEW: controlled by cfg.options.signage_translits
	if cfg.options.signage_translits {
		signage_translits(&mut rewrites, &token_set, has_lat_upper, has_lat_lower);
	}

	// If user disables ligatures, signage_translits is now independent;
	// keep this special SS/ẞ handling as-is.
	if cfg.options.case_mode == "CapsOnly"
		&& cfg.options.keep_ligatures
		&& cfg.options.german_sharp_s_uppercase == "ẞ"
	{
		rewrites.push(("ß".into(), "ẞ".into()));
	}

	// Ligatures: decompose/compose per options
	apply_ligature_rules(&mut rewrites, &token_set, cfg.options.keep_ligatures, cfg.options.rewrite_with_typographic_ligatures);

	// ===== Guard set after “informative” passes (signage + ligatures)
	let mut blocked_sources: BTreeSet<String> =
		rewrites.iter().map(|(from, _)| from.clone()).collect();

	// REWRITES — part 2: bridges derived from the final token set (guarded)
	let mut bridges = build_bridges_for_tokens(&tokens, &cfg.options, &blocked_sources);
	rewrites.extend(bridges.drain(..));

	// Refresh guard (bridges may have introduced more LHS)
	blocked_sources = rewrites.iter().map(|(from, _)| from.clone()).collect();

	// REWRITES — part 3: universal diacritics pack (auto; appended last, guarded)
	if cfg.options.drop_diacritics {
		append_combining_strip_rules_for_kept_tokens(&mut rewrites, &token_set);
		append_auto_diacritic_pack_case_strict(&mut rewrites, &token_set, &blocked_sources);
	}

	// FINAL GUARD: drop any rule whose LHS is a kept token (strongest guarantee)
	rewrites.retain(|(from, _)| !token_set.contains(from));


    // Rewrite order: longest LHS first (greedy max-munch), tie → lexicographic LHS
    rewrites.sort_by(|(a1,_),(a2,_)| a2.len().cmp(&a1.len()).then(a1.cmp(a2)));
    rewrites.dedup();

    // Token order: multi-scalar first
    tokens.sort_by(|a,b| {
        let la = a.chars().count();
        let lb = b.chars().count();
        lb.cmp(&la).then(a.cmp(b))
    });

    // Cosmetic warning
    if cfg.options.case_mode == "CapsOnly" {
        let cas = tokens.iter().any(|t| t.chars().any(|c| ('\u{1400}'..='\u{167F}').contains(&c)));
        if cas {
            warnings.push("case_mode=CapsOnly has no effect on Canadian syllabics (unicameral)".to_string());
        }
    }

    Ok(Build { tokens, rewrites, warnings })
}

/* --------------------------- Bridges (internal) --------------------------- */
fn cyrillic_confusables(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    case_mode: &str,
    blocked_sources: &BTreeSet<String>,   // NEW
){
    // Adapt a RHS string to the kit’s case policy.
    let to_case = |s: &str| -> String {
        match case_mode {
            "LowerOnly" => to_lower(s),
            "CapsOnly"  => to_upper(s, "ẞ"), // helper already available; ẞ irrelevant for Cyrillic
            _           => s.to_string(),    // UpperLower: keep as-authored
        }
    };

    // Core ASCII confusables (both cases).
    // Latin → Cyrillic (and later we’ll mirror Cyrillic → Latin).
    // These fix cases like "ааa" (Latin 'a' leaks) and "джепанe" (Latin 'e' leaks).
    const PAIRS_ASCII: [(&str, &str); 23] = [
        ("A","А"),("a","а"),
        ("B","В"),("b","в"),
        ("C","С"),("c","с"),
        ("E","Е"),("e","е"),
        ("H","Н"),
        ("K","К"),("k","к"),
        ("M","М"),("m","м"),
        ("O","О"),("o","о"),
        ("P","Р"),("p","р"),
        ("T","Т"),("t","т"),
        ("X","Х"),("x","х"),
        ("Y","У"),("y","у"),
    ];

    // LATIN → CYRILLIC (ASCII)
    for &(lat, cyr) in &PAIRS_ASCII {
		if blocked_sources.contains(lat) { continue; }  // NEW
		push_if_target_present(rewrites, tokens, lat, &to_case(cyr));
	}
	
  // If the kit already contains Latin ASCII, prefer staying in Latin (ȯ→o).
    let have_lat_ascii = tokens.contains("A") || tokens.contains("a");

    // Handle accented Latin letters by stripping marks and mapping case-preserving.
    // E.g. ȯ (lower) → o → о (lower), Ȯ (upper) → O → О (upper).
    const LATIN_RANGES: &[(u32,u32)] = &[(0x00C0,0x00FF),(0x0100,0x017F),(0x0180,0x024F)];
    for (lo, hi) in LATIN_RANGES {
        let mut cp = *lo;
        while cp <= *hi {
            if let Some(ch) = char::from_u32(cp) {
                let s = ch.to_string();
                if blocked_sources.contains(&s) { cp += 1; continue; }

                let base = strip_marks(&s);
                if base.len() != 1 { cp += 1; continue; }
                let bch = base.chars().next().unwrap();

                // If Latin ASCII is present, let diacritic stripping (ȯ→o) handle it later.
                if have_lat_ascii {
                    cp += 1;
                    continue;
                }

                // Case-preserving map Latin → Cyrillic for confusable cores
                let cyr_opt: Option<&'static str> = match bch {
                    'A' => Some("А"), 'a' => Some("а"),
                    'B' => Some("В"), 'b' => Some("в"),
                    'C' => Some("С"), 'c' => Some("с"),
                    'E' => Some("Е"), 'e' => Some("е"),
                    'H' => Some("Н"), 'h' => Some("н"),
                    'K' => Some("К"), 'k' => Some("к"),
                    'M' => Some("М"), 'm' => Some("м"),
                    'O' => Some("О"), 'o' => Some("о"),
                    'P' => Some("Р"), 'p' => Some("р"),
                    'T' => Some("Т"), 't' => Some("т"),
                    'X' => Some("Х"), 'x' => Some("х"),
                    'Y' => Some("У"), 'y' => Some("у"),
                    _ => None,
                };

                if let Some(cyr) = cyr_opt {
                    // Adapt to CapsOnly/LowerOnly; UpperLower preserves case as chosen above.
                    let rhs = match case_mode {
                        "LowerOnly" => to_lower(cyr),
                        "CapsOnly"  => to_upper(cyr, "ẞ"),
                        _           => cyr.to_string(),
                    };
                    push_if_target_present(rewrites, tokens, &s, &rhs);
                }
            }
            cp += 1;
        }
    }


    // Leet that shows up in your BG data: 4 → Ч (auto-lowers to ч under LowerOnly).
    push_if_target_present(rewrites, tokens, "4", &to_case("Ч"));

    // Optional: add more leet if you want, e.g. "0"→"О", "3"→"З". Kept minimal per your examples.
    // push_if_target_present(rewrites, tokens, "0", &to_case("О"));
    // push_if_target_present(rewrites, tokens, "3", &to_case("З"));

    // --- Mirror rules (CYRILLIC → LATIN) ---
    // Only added when the Latin *target* exists in the kit and the Cyrillic *source* is not present.
    // Safe for Cyrillic-only kits: Latin tokens won’t be present, so nothing emits.
	for &(lat, cyr) in &PAIRS_ASCII {
		if blocked_sources.contains(cyr) { continue; }  // optional, defensive
		push_if_target_present(rewrites, tokens, cyr, &to_case(lat));
	}
}


fn build_bridges_for_tokens(
    tokens_vec: &[String],
    opts: &Options,
    blocked_sources: &BTreeSet<String>,
) -> Vec<(String, String)> {
    let mut rewrites = Vec::<(String, String)>::new();
    let mut tokens = BTreeSet::<String>::new();
    for t in tokens_vec { tokens.insert(t.clone()); }

    // Case crosswalks (explicit per-script pairs)
    push_case_pairs(&mut rewrites, &tokens, S::latin_ascii_upper, S::latin_ascii_lower);
    push_case_pairs(&mut rewrites, &tokens, S::spanish_core_upper, S::spanish_core_lower);
    push_case_pairs(&mut rewrites, &tokens, S::french_accents_upper, S::french_accents_lower);
    push_case_pairs(&mut rewrites, &tokens, S::german_extras_upper, S::german_extras_lower);
    push_case_pairs(&mut rewrites, &tokens, S::italian_core_upper, S::italian_core_lower);
    push_case_pairs(&mut rewrites, &tokens, S::portuguese_core_upper, S::portuguese_core_lower);
    push_case_pairs(&mut rewrites, &tokens, S::danish_norwegian_upper, S::danish_norwegian_lower);
    push_case_pairs(&mut rewrites, &tokens, S::swedish_upper, S::swedish_lower);
    push_case_pairs(&mut rewrites, &tokens, S::icelandic_upper, S::icelandic_lower);
    push_case_pairs(&mut rewrites, &tokens, S::lithuanian_upper, S::lithuanian_lower);
    push_case_pairs(&mut rewrites, &tokens, S::latvian_upper, S::latvian_lower);
    push_case_pairs(&mut rewrites, &tokens, S::czech_upper, S::czech_lower);
    push_case_pairs(&mut rewrites, &tokens, S::slovak_upper, S::slovak_lower);
    push_case_pairs(&mut rewrites, &tokens, S::slovene_bcs_upper, S::slovene_bcs_lower);
    push_case_pairs(&mut rewrites, &tokens, S::hungarian_upper, S::hungarian_lower);
    push_case_pairs(&mut rewrites, &tokens, S::polish_upper, S::polish_lower);
    push_case_pairs(&mut rewrites, &tokens, S::romanian_upper, S::romanian_lower);
    push_case_pairs(&mut rewrites, &tokens, S::welsh_upper, S::welsh_lower);
    push_case_pairs(&mut rewrites, &tokens, S::gaelic_upper, S::gaelic_lower);
    push_case_pairs(&mut rewrites, &tokens, S::maltese_upper, S::maltese_lower);
    push_case_pairs(&mut rewrites, &tokens, S::turkish_azeri_upper, S::turkish_azeri_lower);
    push_case_pairs(&mut rewrites, &tokens, S::latin_pinyin_upper, S::latin_pinyin_lower);

    // Greenlandic specifics
    push_to_any(&mut rewrites, &tokens, "ĸ", &["k","K"]);
    push_to_any(&mut rewrites, &tokens, "Ŋ", &["ŋ"]);
    push_to_any(&mut rewrites, &tokens, "ŋ", &["Ŋ"]);

    // Romanian cedilla hints
    push_to_any(&mut rewrites, &tokens, "ţ", &["ț","t","T"]);
    push_to_any(&mut rewrites, &tokens, "Ţ", &["Ț","T","t"]);
    push_to_any(&mut rewrites, &tokens, "ş", &["ș","s","S"]);
    push_to_any(&mut rewrites, &tokens, "Ş", &["Ș","S","s"]);

    // Greek/Cyrillic/Armenian
    push_case_pairs(&mut rewrites, &tokens, S::greek_caps, S::greek_lower_base);
    push_case_pairs(&mut rewrites, &tokens, S::cyrillic_ru_caps, S::cyrillic_ru_lower);
    push_case_pairs(&mut rewrites, &tokens, S::cyrillic_uk_caps, S::cyrillic_uk_lower);
    push_case_pairs(&mut rewrites, &tokens, S::cyrillic_be_caps, S::cyrillic_be_lower);
    push_case_pairs(&mut rewrites, &tokens, S::cyrillic_sr_me_caps, S::cyrillic_sr_me_lower);
    push_case_pairs(&mut rewrites, &tokens, S::cyrillic_mk_caps, S::cyrillic_mk_lower);
    push_case_pairs(&mut rewrites, &tokens, S::armenian_caps, S::armenian_lower);

    // Script-specific bridges
    greek_bridges(&mut rewrites, &tokens, opts.language_specific_bridges);


    // Soft Latin diacritic fallbacks
    if opts.language_specific_bridges {
        georgian_bridges(&mut rewrites, &tokens);
        hebrew_final_bridges(&mut rewrites, &tokens);
        kana_bridges(&mut rewrites, &tokens);
        cyrillic_subset_bridges(&mut rewrites, &tokens, &opts.case_mode);
        cyrillic_confusables(&mut rewrites, &tokens, &opts.case_mode, blocked_sources);
        latin_accent_bridges(&mut rewrites, &tokens, blocked_sources); // ← guarded
    }

    // Unicode-wide automatic case bridges — restricted by case_mode and filters.
    auto_case_bridges(&mut rewrites, &tokens, &opts.case_mode);

    // Order + dedup
    rewrites.sort_by(|(a1,_),(a2,_)| a2.len().cmp(&a1.len()).then(a1.cmp(a2)));
    rewrites.dedup();
    rewrites
}

/* --------------------------- Human-friendly preview --------------------------- */

fn auto_case_bridges(
    rewrites: &mut Vec<(String, String)>,
    tokens: &BTreeSet<String>,
    case_mode: &str,
) {
    // In UpperLower we already have both cases; avoid synthesizing any case fallbacks.
    match case_mode {
        "UpperLower" => return,
        "CapsOnly" | "LowerOnly" => {},
        _ => return,
    }

    let german_upper = "ẞ";
    let mut added = HashSet::<(String, String)>::new();

    // Helpers to block unwanted pairs:
    const LIGS: [&str; 5] = ["ﬀ","ﬁ","ﬂ","ﬃ","ﬄ"];
    let is_forbidden = |u: &str, l: &str| -> bool {
        // Block any pair touching typographic ligature scalars
        if LIGS.iter().any(|&s| u.contains(s) || l.contains(s)) { return true; }
        // Block Turkish dotted-I bridges entirely (both composed and decomposed forms)
        if u.contains('\u{0130}') || l.contains('\u{0130}') || u.contains('\u{0307}') || l.contains('\u{0307}') {
            return true;
        }
        false
    };

    for t in tokens.iter() {
        let u = to_upper(t, german_upper);
        let l = to_lower(t);

        if u == l { continue; }

        match case_mode {
            "CapsOnly" => {
                // Only synthesize lower → UPPER
                if present(tokens, &u) && !present(tokens, &l) && !is_forbidden(&u, &l) {
                    let pair = (l.clone(), u.clone());
                    if added.insert(pair.clone()) { rewrites.push(pair); }
                }
            }
            "LowerOnly" => {
                // Only synthesize UPPER → lower
                if present(tokens, &l) && !present(tokens, &u) && !is_forbidden(&u, &l) {
                    let pair = (u.clone(), l.clone());
                    if added.insert(pair.clone()) { rewrites.push(pair); }
                }
            }
            _ => {}
        }
    }
}

pub fn print_preview(build: &Build) {
    println!("===== Glyph Kit =====");
    println!("Letters: {}", build.tokens.len());
    println!("Rewrites: {}", build.rewrites.len());
    if !build.warnings.is_empty() {
        println!("Warnings:");
        for w in &build.warnings { println!("  - {}", w); }
    }

    // Tokens preview
    println!("Tokens:");
    for t in build.tokens.iter() { print!("{} ", t); }
    println!();

    // Simple presence check for digraph tokens
    let have_rh = build.tokens.iter().any(|t| t == "rh" || t == "RH");
    let have_lh = build.tokens.iter().any(|t| t == "lh" || t == "LH");
    println!("Digraphs present → rh/RH: {have_rh}, lh/LH: {have_lh}");

    // Print ALL rewrites
    println!("Rewrites (full list):");
    for (from, to) in &build.rewrites {
        println!("  {} -> {}", from, to);
    }

    // Focus view: rewrites that mention rh / lh (if any)
    let needles = ["rh", "RH", "lh", "LH"];
    let mut hits = 0usize;
    println!("Rewrites mentioning rh/lh:");
    for (from, to) in &build.rewrites {
        if needles.iter().any(|n| from.contains(n) || to.contains(n)) {
            println!("  {} -> {}", from, to);
            hits += 1;
        }
    }
    if hits == 0 {
        println!("  (none)");
    }
}
