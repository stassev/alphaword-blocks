// AlphaWord Blocks — glyph database
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

use strum_macros::EnumIter;

// src/glyph_db.rs

#[allow(non_upper_case_globals)]
pub mod sets {
    /* -------------------------- Latin: base & extras -------------------------- */

    pub const latin_ascii_upper: &[&str] = &[
        "A","B","C","D","E","F","G","H","I","J","K","L","M",
        "N","O","P","Q","R","S","T","U","V","W","X","Y","Z",
    ];
    pub const latin_ascii_lower: &[&str] = &[
        "a","b","c","d","e","f","g","h","i","j","k","l","m",
        "n","o","p","q","r","s","t","u","v","w","x","y","z",
    ];

    // Spanish core: Ñ/ñ (accents are handled elsewhere via options/bridges)
    pub const spanish_core_upper: &[&str] = &["Ñ"];
    pub const spanish_core_lower: &[&str] = &["ñ"];

    // French accents (no ligatures here; ligatures are in latin_ligatures_*)
    pub const french_accents_upper: &[&str] = &["À","Â","Ä","Ç","É","È","Ê","Ë","Î","Ï","Ô","Ö","Ù","Û","Ü","Ÿ"];
    pub const french_accents_lower: &[&str] = &["à","â","ä","ç","é","è","ê","ë","î","ï","ô","ö","ù","û","ü","ÿ"];

    // German umlauts + sharp s (modern ẞ)
    pub const german_extras_upper: &[&str] = &["Ä","Ö","Ü","ẞ"];
    pub const german_extras_lower: &[&str] = &["ä","ö","ü","ß"];

    // Italian core
    pub const italian_core_upper: &[&str] = &["À","È","É","Ì","Ò","Ù"];
    pub const italian_core_lower: &[&str] = &["à","è","é","ì","ò","ù"];

    // Portuguese
    pub const portuguese_core_upper: &[&str] = &["Ã","Õ","Â","Ê","Ô","Á","É","Í","Ó","Ú"];
    pub const portuguese_core_lower: &[&str] = &["ã","õ","â","ê","ô","á","é","í","ó","ú"];

    // Nordic
    pub const danish_norwegian_upper: &[&str] = &["Å","Æ","Ø"];
    pub const danish_norwegian_lower: &[&str] = &["å","æ","ø"];
    pub const swedish_upper: &[&str] = &["Å","Ä","Ö"];
    pub const swedish_lower: &[&str] = &["å","ä","ö"];
    pub const icelandic_upper: &[&str] = &["Á","Ð","É","Í","Ó","Ú","Ý","Þ","Æ","Ö"];
    pub const icelandic_lower: &[&str] = &["á","ð","é","í","ó","ú","ý","þ","æ","ö"];

    // Baltic
    pub const lithuanian_upper: &[&str] = &["Ą","Č","Ę","Ė","Į","Š","Ų","Ū","Ž"];
    pub const lithuanian_lower: &[&str] = &["ą","č","ę","ė","į","š","ų","ū","ž"];
    pub const latvian_upper: &[&str] = &["Ā","Č","Ē","Ģ","Ī","Ķ","Ļ","Ņ","Š","Ū","Ž"];
    pub const latvian_lower: &[&str] = &["ā","č","ē","ģ","ī","ķ","ļ","ņ","š","ū","ž"];

    // Central/Eastern
    pub const czech_upper: &[&str] = &["Á","Č","Ď","É","Ě","Í","Ň","Ó","Ř","Š","Ť","Ú","Ů","Ý","Ž"];
    pub const czech_lower: &[&str] = &["á","č","ď","é","ě","í","ň","ó","ř","š","ť","ú","ů","ý","ž"];
    pub const slovak_upper: &[&str] = &["Á","Ä","Č","Ď","É","Í","Ĺ","Ľ","Ň","Ó","Ô","Ŕ","Š","Ť","Ú","Ý","Ž"];
    pub const slovak_lower: &[&str] = &["á","ä","č","ď","é","í","ĺ","ľ","ň","ó","ô","ŕ","š","ť","ú","ý","ž"];
    pub const slovene_bcs_upper: &[&str] = &["Č","Ć","Đ","Š","Ž"];
    pub const slovene_bcs_lower: &[&str] = &["č","ć","đ","š","ž"];
    pub const hungarian_upper: &[&str] = &["Á","É","Í","Ó","Ö","Ő","Ú","Ü","Ű"];
    pub const hungarian_lower: &[&str] = &["á","é","í","ó","ö","ő","ú","ü","ű"];
    pub const polish_upper: &[&str] = &["Ą","Ć","Ę","Ł","Ń","Ó","Ś","Ź","Ż"];
    pub const polish_lower: &[&str] = &["ą","ć","ę","ł","ń","ó","ś","ź","ż"];
    pub const romanian_upper: &[&str] = &["Ă","Â","Î","Ș","Ț"];
    pub const romanian_lower: &[&str] = &["ă","â","î","ș","ț"];

    // Celtic
    pub const welsh_upper: &[&str] = &["Â","Ê","Î","Ô","Û","Ŵ","Ŷ"];
    pub const welsh_lower: &[&str] = &["â","ê","î","ô","û","ŵ","ŷ"];
    pub const gaelic_upper: &[&str] = &["À","È","Ì","Ò","Ù"];
    pub const gaelic_lower: &[&str] = &["à","è","ì","ò","ù"];

    // Maltese
    pub const maltese_upper: &[&str] = &["Ċ","Ġ","Ħ","Ż"];
    pub const maltese_lower: &[&str] = &["ċ","ġ","ħ","ż"];

    // Turkish/Azeri (Latin)
    pub const turkish_azeri_upper: &[&str] = &["Ç","Ğ","İ","Ö","Ş","Ü"];
    pub const turkish_azeri_lower: &[&str] = &["ç","ğ","ı","ö","ş","ü"];

    // Greenlandic historically
    pub const greenlandic_upper: &[&str] = &["Ŋ"];
    pub const greenlandic_lower: &[&str] = &["ŋ","ĸ"];

    // Latin ligatures/typographic forms
    pub const latin_ligatures_upper: &[&str] = &["Æ","Œ","Ĳ","ẞ"];
    pub const latin_ligatures_lower: &[&str] = &["æ","œ","ĳ","ß","ﬀ","ﬁ","ﬂ","ﬃ","ﬄ"];

    /* ------------------------ Latin digraphs split by language ----------------------- */
    // Common signage-ish clusters
    pub const latin_digraphs_common: &[&str] = &[
        "tsch","sch","stj","skj","tch","sci","gli","eau","que","qui","gue","gui",
        "gh","ph","qu","th","sh","zh","ch","ll","rr","gn","nh","lh","dd","ff","ng","rh",
        "kj","tj","sj","hv","hj","gj","ij","tx","ts","tz",
        // UPPERCASE
        "TSCH","SCH","STJ","SKJ","TCH","SCI","GLI","EAU","QUE","QUI","GUE","GUI",
        "GH","PH","QU","TH","SH","ZH","CH","LL","RR","GN","NH","LH","DD","FF","NG","RH",
        "KJ","TJ","SJ","HV","HJ","GJ","IJ","TX","TS","TZ",
    ];
    // Catalan
    pub const latin_digraphs_catalan: &[&str] = &["l·l","L·L"];
    // Maltese (multi-scalar)
    pub const latin_digraphs_maltese: &[&str] = &["għ","GĦ"];
    // Union (convenience)
    pub const latin_digraphs_all: &[&str] = &[
        "tsch","sch","stj","skj","tch","sci","gli","eau","que","qui","gue","gui",
        "gh","ph","qu","th","sh","zh","ch","ll","rr","gn","nh","lh","dd","ff","ng","rh",
        "kj","tj","sj","hv","hj","gj","ij","tx","ts","tz",
        "TSCH","SCH","STJ","SKJ","TCH","SCI","GLI","EAU","QUE","QUI","GUE","GUI",
        "GH","PH","QU","TH","SH","ZH","CH","LL","RR","GN","NH","LH","DD","FF","NG","RH",
        "KJ","TJ","SJ","HV","HJ","GJ","IJ","TX","TS","TZ",
        "l·l","L·L",
        "għ","GĦ",
    ];

    /* ------------------------ Latin IPA + modifiers (deduped) ----------------------- */
    // Non-ASCII IPA/Africanist letters (merged + deduped)
    pub const latin_ipa_core: &[&str] = &[
        // vowels
        "ɐ","ɑ","ɒ","æ","ə","ɚ","ɜ","ɞ","ɛ","ɘ","ɵ","ɤ","ɔ","ʌ","ɯ","ʊ","ɨ","ʉ","ɪ","ʏ",
        "ɶ",  // U+0276 LATIN LETTER SMALL CAPITAL OE (IPA)
        // pulmonic fricatives & others
        "β","θ","ð","ʃ","ʒ","ʂ","ʐ","ç","ʝ","ɣ","χ","ʁ","ħ","ʕ","ɦ","ɸ",
        // approximants / liquids
        "ʍ","ɥ","ʋ","ɹ","ɾ","ɽ","ɻ","ʀ","ɫ","ʎ","ɭ","ɮ","ɬ",
        // nasals
        "ɱ","ɲ","ɳ","ŋ","ɴ",
        // stops/affricate bases
        "ʈ","ɖ","ɟ","ɢ","ʔ","ɡ","ʡ","ʢ",
    ];
    // IPA stress, length, tie bars, frequent modifier letters & combining marks
    pub const latin_ipa_modifiers: &[&str] = &[
        "ˈ","ˌ","ː","ˑ","˞","ʰ","ʷ","ʲ","ʼ",
        "\u{0361}","\u{035C}", // tie bar above/below (͡, ͜)
        "\u{0303}","\u{0308}","\u{0329}","\u{032F}","\u{031F}","\u{0320}","\u{032A}","\u{033A}","\u{033B}",
    ];

    /* ------------------------------ Greek (monotonic) ------------------------------ */
    pub const greek_caps: &[&str] = &[
        "Α","Β","Γ","Δ","Ε","Ζ","Η","Θ","Ι","Κ","Λ","Μ","Ν","Ξ","Ο","Π","Ρ","Σ","Τ","Υ","Φ","Χ","Ψ","Ω",
    ];
    pub const greek_lower_base: &[&str] = &[
        "α","β","γ","δ","ε","ζ","η","θ","ι","κ","λ","μ","ν","ξ","ο","π","ρ","σ","τ","υ","φ","χ","ψ","ω","ς",
    ];
    pub const greek_monotonic_marks_upper: &[&str] = &["Ά","Έ","Ή","Ί","Ό","Ύ","Ώ","Ϊ","Ϋ"];
    pub const greek_monotonic_marks_lower: &[&str] = &["ά","έ","ή","ί","ό","ύ","ώ","ϊ","ΐ","ϋ","ΰ"];

    /* ------------------------------- Cyrillic families ------------------------------ */
    pub const cyrillic_ru_caps: &[&str] = &[
        "А","Б","В","Г","Д","Е","Ё","Ж","З","И","Й","К","Л","М","Н","О","П","Р","С","Т","У","Ф","Х","Ц","Ч","Ш","Щ","Ъ","Ы","Ь","Э","Ю","Я",
    ];
    pub const cyrillic_ru_lower: &[&str] = &[
        "а","б","в","г","д","е","ё","ж","з","и","й","к","л","м","н","о","п","р","с","т","у","ф","х","ц","ч","ш","щ","ъ","ы","ь","э","ю","я",
    ];

    // Bulgarian (separate, as requested)
    pub const cyrillic_bg_caps: &[&str] = &[
        "А","Б","В","Г","Д","Е","Ж","З","И","Й","К","Л","М","Н","О","П","Р","С","Т","У","Ф","Х","Ц","Ч","Ш","Щ","Ъ","Ь","Ю","Я",
    ];
    pub const cyrillic_bg_lower: &[&str] = &[
        "а","б","в","г","д","е","ж","з","и","й","к","л","м","н","о","п","р","с","т","у","ф","х","ц","ч","ш","щ","ъ","ь","ю","я",
    ];

    pub const cyrillic_uk_caps: &[&str] = &["Ґ","Є","І","Ї"];
    pub const cyrillic_uk_lower: &[&str] = &["ґ","є","і","ї"];
    pub const cyrillic_be_caps: &[&str] = &["Ў"];
    pub const cyrillic_be_lower: &[&str] = &["ў"];
    pub const cyrillic_sr_me_caps: &[&str] = &["Ђ","Ћ","Ј","Љ","Њ","Џ"];
    pub const cyrillic_sr_me_lower: &[&str] = &["ђ","ћ","ј","љ","њ","џ"];
    pub const cyrillic_mk_caps: &[&str] = &["Ѓ","Ќ","Ѕ"];
    pub const cyrillic_mk_lower: &[&str] = &["ѓ","ќ","ѕ"];

    /* ------------------------ Pinyin (Latin with tone marks) ------------------------ */
    pub const latin_pinyin_upper: &[&str] = &[
        "Ā","Á","Ǎ","À","Ē","É","Ě","È","Ī","Í","Ǐ","Ì",
        "Ō","Ó","Ǒ","Ò","Ū","Ú","Ǔ","Ù",
        "Ǖ","Ǘ","Ǚ","Ǜ","Ü",
        "M̄","Ḿ","M̌","M̀","N̄","Ń","Ň","Ǹ",
    ];
    pub const latin_pinyin_lower: &[&str] = &[
        "ā","á","ǎ","à","ē","é","ě","è","ī","í","ǐ","ì",
        "ō","ó","ǒ","ò","ū","ú","ǔ","ù",
        "ǖ","ǘ","ǚ","ǜ","ü",
        "m̄","ḿ","m̌","m̀","n̄","ń","ň","ǹ",
    ];

    /* --------------------------------- Armenian ------------------------------------- */
    pub const armenian_caps: &[&str] = &[
        "Ա","Բ","Գ","Դ","Ե","Զ","Է","Ը","Թ","Ժ","Ի","Լ","Խ","Ծ","Կ","Հ","Ձ","Ղ","Ճ","Մ","Յ","Ն","Շ","Ո","Չ","Պ","Ջ","Ռ","Ս","Վ","Տ","Ր","Ց","Ւ","Փ","Ք","Օ","Ֆ",
    ];
    pub const armenian_lower: &[&str] = &[
        "ա","բ","գ","դ","ե","զ","է","ը","թ","ժ","ի","լ","խ","ծ","կ","հ","ձ","ղ","ճ","մ","յ","ն","շ","ո","չ","պ","ջ","ռ","ս","վ","տ","ր","ց","ւ","փ","ք","օ","ֆ",
        
    ];

    /* --------------------------------- Georgian ------------------------------------- */
    pub const georgian_mkhedruli: &[&str] = &[
        "ა","ბ","გ","დ","ე","ვ","ზ","თ","ი","კ","ლ","მ","ნ","ო","პ","ჟ","რ","ს","ტ","უ","ფ","ქ","ღ","ყ","შ","ჩ","ც","ძ","წ","ჭ","ხ","ჯ","ჰ",
    ];
    pub const georgian_mtavruli: &[&str] = &[
        "Ა","Ბ","Გ","Დ","Ე","Ვ","Ზ","Თ","Ი","Კ","Ლ","Მ","Ნ","Ო","Პ","Ჟ","Რ","Ს","Ტ","Უ","Ფ","Ქ","Ღ","Ყ","Შ","Ჩ","Ც","Ძ","Წ","Ჭ","Ხ","Ჯ","Ჰ",
    ];

    /* ---------------------------------- Hebrew -------------------------------------- */
    pub const hebrew_letters: &[&str] = &[
        "א","ב","ג","ד","ה","ו","ז","ח","ט","י","כ","ך","ל","מ","ם","נ","ן","ס","ע","פ","ף","צ","ץ","ק","ר","ש","ת",
    ];

    /* ---------------------------------- Cherokee ------------------------------------ */
    pub const cherokee_upper: &[&str] = &[
        "Ꭰ","Ꭱ","Ꭲ","Ꭳ","Ꭴ","Ꭵ","Ꭶ","Ꭷ","Ꭸ","Ꭹ","Ꭺ","Ꭻ","Ꭼ","Ꭽ","Ꭾ","Ꭿ","Ꮀ","Ꮁ","Ꮂ","Ꮃ","Ꮄ","Ꮅ","Ꮆ","Ꮇ","Ꮈ","Ꮉ","Ꮊ","Ꮋ","Ꮌ","Ꮍ","Ꮎ","Ꮏ","Ꮐ","Ꮑ","Ꮒ","Ꮓ","Ꮔ","Ꮕ","Ꮖ","Ꮗ","Ꮘ","Ꮙ","Ꮚ","Ꮛ","Ꮜ","Ꮝ","Ꮞ","Ꮟ","Ꮠ","Ꮡ","Ꮢ","Ꮣ","Ꮤ","Ꮥ","Ꮦ","Ꮧ","Ꮨ","Ꮩ","Ꮪ","Ꮫ","Ꮬ","Ꮭ","Ꮮ","Ꮯ","Ꮰ","Ꮱ","Ꮲ","Ꮳ","Ꮴ","Ꮵ","Ꮶ","Ꮷ","Ꮸ","Ꮹ","Ꮺ","Ꮻ","Ꮼ","Ꮽ","Ꮾ","Ꮿ","Ᏸ","Ᏹ","Ᏺ","Ᏻ","Ᏼ",
    ];
    pub const cherokee_small: &[&str] = &[
        "ꭰ","ꭱ","ꭲ","ꭳ","ꭴ","ꭵ","ꭶ","ꭷ","ꭸ","ꭹ","ꭺ","ꭻ","ꭼ","ꭽ","ꭾ","ꭿ","ꮀ","ꮁ","ꮂ","ꮃ","ꮄ","ꮅ","ꮆ","ꮇ","ꮈ","ꮉ","ꮊ","ꮋ","ꮌ","ꮍ","ꮎ","ꮏ","ꮐ","ꮑ","ꮒ","ꮓ","ꮔ","ꮕ","ꮖ","ꮗ","ꮘ","ꮙ","ꮚ","ꮛ","ꮜ","ꮝ","ꮞ","ꮟ","ꮠ","ꮡ","ꮢ","ꮣ","ꮤ","ꮥ","ꮦ","ꮧ","ꮨ","ꮩ","ꮪ","ꮫ","ꮬ","ꮭ","ꮮ","ꮯ","ꮰ","ꮱ","ꮲ","ꮳ","ꮴ","ꮵ","ꮶ","ꮷ","ꮸ","ꮹ","ꮺ","ꮻ","ꮼ","ꮽ","ꮾ","ꮿ","ꯀ","ꯁ","ꯂ","ꯃ","ꯄ",
    ];

    /* -------------------- Canadian Aboriginal Syllabics (cores) --------------------- */
    pub const inuktitut_core: &[&str] = &[
        "ᐁ","ᐃ","ᐅ","ᐊ","ᐯ","ᐱ","ᐳ","ᐸ","ᑌ","ᑎ","ᑐ","ᑕ","ᑭ","ᑯ","ᑲ","ᒋ","ᒍ","ᒐ",
        "ᒥ","ᒧ","ᒪ","ᓂ","ᓄ","ᓇ","ᓯ","ᓱ","ᓴ","ᔑ","ᔓ","ᔕ","ᕕ","ᕗ","ᕙ","ᕿ","ᖁ","ᖃ","ᖏ","ᖑ","ᖓ","ᖠ","ᖢ","ᖤ"
    ];
    pub const cree_core: &[&str] = &[
        "ᐁ","ᐃ","ᐄ","ᐅ","ᐆ","ᐊ","ᐋ","ᐯ","ᐱ","ᐳ","ᐸ","ᑌ","ᑎ","ᑐ","ᑕ","ᑭ","ᑯ","ᑲ","ᒋ","ᒍ","ᒐ","ᒥ","ᒧ","ᒪ",
        "ᓂ","ᓄ","ᓇ","ᓯ","ᓰ","ᓱ","ᓴ","ᔑ","ᔓ","ᔕ","ᕁ","ᕃ"
    ];

    /* -------------------------------- Japanese Kana --------------------------------- */
    /* ----------------------- Kana: complete “core” sets ----------------------- */
	/* Hiragana core = U+3041..U+3096 (no iteration marks; includes small forms) */
	pub const hiragana: &[&str] = &[
		"あ","い","う","え","お",
		"か","き","く","け","こ","が","ぎ","ぐ","げ","ご",
		"さ","し","す","せ","そ","ざ","じ","ず","ぜ","ぞ",
		"た","ち","つ","て","と","だ","ぢ","づ","で","ど",
		"な","に","ぬ","ね","の",
		"は","ひ","ふ","へ","ほ","ば","び","ぶ","べ","ぼ","ぱ","ぴ","ぷ","ぺ","ぽ",
		"ま","み","む","め","も",
		"や","ゆ","よ",
		"ら","り","る","れ","ろ",
		"わ","ゐ","ゑ","を","ん",
		// smalls + extras (complete for 3041–3096)
		"ぁ","ぃ","ぅ","ぇ","ぉ","ゃ","ゅ","ょ","っ","ゎ","ゔ","ゕ","ゖ"
	];

	/* Katakana core = U+30A1..U+30FA, plus U+30FC prolonged mark; 
	   includes small WA/KA/KE and VA/VI/VE/VO letters */
	pub const katakana: &[&str] = &[
		"ア","イ","ウ","エ","オ",
		"カ","キ","ク","ケ","コ","ガ","ギ","グ","ゲ","ゴ",
		"サ","シ","ス","セ","ソ","ザ","ジ","ズ","ゼ","ゾ",
		"タ","チ","ツ","テ","ト","ダ","ヂ","ヅ","デ","ド",
		"ナ","ニ","ヌ","ネ","ノ",
		"ハ","ヒ","フ","ヘ","ホ","バ","ビ","ブ","ベ","ボ",
		"パ","ピ","プ","ペ","ポ",
		"マ","ミ","ム","メ","モ",
		"ヤ","ユ","ヨ",
		"ラ","リ","ル","レ","ロ",
		"ワ","ヰ","ヱ","ヲ","ン",
		// smalls + extended smalls
		"ァ","ィ","ゥ","ェ","ォ","ャ","ュ","ョ","ッ","ヮ","ヵ","ヶ",
		// voiced/extended
		"ヴ","ヷ","ヸ","ヹ","ヺ",
		// prolonged sound mark
		"ー"
	];

    /* ------------------------------------ Hangul Jamo -------------------------------- */
    pub const hangul_jamo_l: &[&str] = &[
        "ᄀ","ᄁ","ᄂ","ᄃ","ᄄ","ᄅ","ᄆ","ᄇ","ᄈ","ᄉ","ᄊ","ᄋ","ᄌ","ᄍ","ᄎ","ᄏ","ᄐ","ᄑ","ᄒ",
    ];
    pub const hangul_jamo_v: &[&str] = &[
        "ᅡ","ᅢ","ᅣ","ᅤ","ᅥ","ᅦ","ᅧ","ᅨ","ᅩ","ᅪ","ᅫ","ᅬ","ᅭ","ᅮ","ᅯ","ᅰ","ᅱ","ᅲ","ᅳ","ᅴ","ᅵ",
    ];
    pub const hangul_jamo_t: &[&str] = &[
        "ᆨ","ᆩ","ᆪ","ᆫ","ᆬ","ᆭ","ᆮ","ᆯ","ᆰ","ᆱ","ᆲ","ᆳ","ᆴ","ᆵ","ᆶ","ᆷ","ᆸ","ᆹ","ᆺ","ᆻ","ᆼ","ᆽ","ᆾ","ᆿ","ᇀ","ᇁ","ᇂ",
    ];
    pub const hangul_jamo_all: &[&str] = &[
        "ᄀ","ᄁ","ᄂ","ᄃ","ᄄ","ᄅ","ᄆ","ᄇ","ᄈ","ᄉ","ᄊ","ᄋ","ᄌ","ᄍ","ᄎ","ᄏ","ᄐ","ᄑ","ᄒ",
        "ᅡ","ᅢ","ᅣ","ᅤ","ᅥ","ᅦ","ᅧ","ᅨ","ᅩ","ᅪ","ᅫ","ᅬ","ᅭ","ᅮ","ᅯ","ᅰ","ᅱ","ᅲ","ᅳ","ᅴ","ᅵ",
        "ᆨ","ᆩ","ᆪ","ᆫ","ᆬ","ᆭ","ᆮ","ᆯ","ᆰ","ᆱ","ᆲ","ᆳ","ᆴ","ᆵ","ᆶ","ᆷ","ᆸ","ᆹ","ᆺ","ᆻ","ᆼ","ᆽ","ᆾ","ᆿ","ᇀ","ᇁ","ᇂ",
    ];

    /* ------------------------------------ Ethiopic ----------------------------------- */
    pub const ethiopic_core: &[&str] = &[
        "ሀ","ለ","ሐ","መ","ሠ","ረ","ሰ","ሸ","ቀ","ቈ","ቐ","ቘ","በ","ቨ","ተ","ቸ","ኀ","ኈ","ነ","ኘ","አ","ከ","ኸ","ወ","ዐ","ዘ","ዠ","የ","ደ","ጀ","ገ","ጘ","ጠ","ጨ","ጰ","ጸ","ፀ","ፈ","ፐ",
    ];

    /* ------------------------------------ Arabic & friends --------------------------- */
    // Core Arabic letters + hamza forms (NFC, unified order)
    /* Arabic “core” + the 063B–063F letters and U+0640 TATWEEL to match your first list */
	pub const arabic_core: &[&str] = &[
		"ء","آ","أ","ؤ","إ","ئ","ا","ب","ت","ث","ج","ح","خ","د","ذ","ر","ز",
		"س","ش","ص","ض","ط","ظ","ع","غ","ف","ق","ك","ل","م","ن","ه","ة","و","ى","ي",
		// additions you had in your first module:
		"ػ","ؼ","ؽ","ؾ","ؿ","ـ"
	];
    // Persian/Urdu extras & variants (merged + deduped; includes user list)
    pub const persian_urdu_extras: &[&str] = &[
        "پ","چ","ژ","گ","ک","ی","ٹ","ڈ","ڑ","ں",
        "ھ","ہ","ۂ","ۃ","ے","ۓ","ڤ","ڠ"
    ];

    /* ------------------------------------- Syriac ------------------------------------ */
	pub const syriac_core: &[&str] = &[
		"ܐ","ܒ","ܓ","ܕ","ܗ","ܘ","ܙ","ܚ","ܛ","ܝ","ܟ","ܠ","ܡ","ܢ","ܣ","ܥ","ܦ","ܨ","ܩ","ܪ","ܫ","ܬ",
		// additions you had in your first module:
		"ܑ","ܔ","ܜ","ܞ","ܤ","ܧ"
	];

    /* ------------------------------------- Thaana ------------------------------------ */
    pub const thaana_core: &[&str] = &[
		"ހ","ށ","ނ","ރ","ބ","ޅ","ކ","އ","ވ","މ","ފ","ދ","ތ","ލ","ގ","ޏ","ސ","ޑ","ޒ","ޓ","ޔ","ޕ","ޖ","ޗ",
		"ޘ","ޙ","ޚ","ޛ","ޜ","ޝ","ޞ","ޟ","ޠ","ޡ","ޢ","ޣ","ޤ","ޥ",
		// vowels/diacritics block you were already including:
		"ަ","ާ","ި","ީ","ު","ޫ","ެ","ޭ","ޮ","ޯ","ް",
		// added legacy letter:
		"ޱ"
	];

    /* ---------------------------------- Indic abugidas -------------------------------- */
    pub const devanagari_core: &[&str] = &[
        "अ","आ","इ","ई","उ","ऊ","ऋ","ॠ","ऌ","ॡ","ए","ऐ","ओ","औ","अं","अः",
        "ँ","ं","ः","्",
        "क","ख","ग","घ","ङ","च","छ","ज","झ","ञ","ट","ठ","ड","ढ","ण","त","थ","द","ध","न","प","फ","ब","भ","म","य","र","ल","व","श","ष","स","ह","ळ","क्ष","ज्ञ",
    ];

    // Bengali
    pub const bengali_core: &[&str] = &[
        "অ","আ","ই","ঈ","উ","ঊ","ঋ","ৠ","ঌ","ৡ","এ","ঐ","ও","ঔ",
        "ঁ","ং","ঃ","্",
        "ক","খ","গ","ঘ","ঙ","চ","ছ","জ","ঝ","ঞ","ট","ঠ","ড","ঢ","ণ","ত","থ","দ","ধ","ন","প","ফ","ব","ভ","ম","য","র","ল","শ","ষ","স","হ","ড\u{09BC}","ঢ\u{09BC}","য\u{09BC}","ৎ",
    ];

    // Gurmukhi
    pub const gurmukhi_core: &[&str] = &[
        "ਅ","ਆ","ਇ","ਈ","ਉ","ਊ","ਏ","ਐ","ਓ","ਔ","ਅੰ","ਅਃ",
        "ਂ","ੰ","ਃ","੍",
        "ਕ","ਖ","ਗ","ਘ","ਙ","ਚ","ਛ","ਜ","ਝ","ਞ","ਟ","ਠ","ਡ","ਢ","ਣ","ਤ","ਥ","ਦ","ਧ","ਨ",
        "ਪ","ਫ","ਬ","ਭ","ਮ","ਯ","ਰ","ਲ","ਵ","ਸ","ਹ",
        "ਸ\u{0A3C}","ਖ\u{0A3C}","ਗ\u{0A3C}","ਜ\u{0A3C}","ਫ\u{0A3C}","ਲ\u{0A3C}","ੜ",
    ];

    // Gujarati
    pub const gujarati_core: &[&str] = &[
        "અ","આ","ઇ","ઈ","ઉ","ઊ","ઋ","ૠ","ઌ","ૡ","એ","ઐ","ઓ","ઔ","અં","અઃ","ઁ","ં","ઃ","્",
        "ક","ખ","ગ","ઘ","ઙ","ચ","છ","જ","ઝ","ઞ","ટ","ઠ","ડ","ઢ","ણ","ત","થ","દ","ધ","ન","પ","ફ","બ","ભ","મ","ય","ર","લ","વ","શ","ષ","સ","હ","ળ","ક્ષ","જ્ઞ",
    ];

    // Odia
    pub const odia_core: &[&str] = &[
        "ଅ","ଆ","ଇ","ୀ","ଉ","ୂ","ଋ","ୠ","ଌ","ୡ","ଏ","ଐ","ଓ","ଔ","ଁ","ଂ","ଃ","୍",
        "କ","ଖ","ଗ","ଘ","ଙ","ଚ","ଛ","ଜ","ଝ","ଞ","ଟ","ଠ","ଡ","ଢ","ଣ","ତ","ଥ","ଦ","ଧ","ନ","ପ","ଫ","ବ","ଭ","ମ","ଯ","ର","ଲ","ଵ","ଶ","ଷ","ସ","ହ","ଳ","କ୍ଷ",
    ];

    // Tamil
    pub const tamil_core: &[&str] = &[
        "அ","ஆ","இ","ஈ","உ","ஊ","எ","ஏ","ஐ","ஒ","ஓ","ஔ","ஃ",
        "க்","ங்","ச்","ஞ்","ட்","ண்","த்","ந்","ப்","ம்","ய்","ர்","ல்","வ்","ழ்","ள்","று","ன்",
        "க","ங","ச","ஞ","ட","ண","த","ந","ப","ம","ய","ர","ல","வ","ழ","ள","ற","ன",
    ];

    // Telugu
    pub const telugu_core: &[&str] = &[
        "అ","ఆ","ఇ","ఈ","ఉ","ఊ","ఋ","ౠ","ఌ","ౡ","ఎ","ఏ","ఐ","ఒ","ఓ","ఔ","ఁ","ం","ః","్",
        "క","ఖ","గ","ఘ","ఙ","చ","ఛ","జ","ఝ","ఞ","ట","ఠ","డ","ఢ","ణ","త","థ","ద","ధ","న","ప","ఫ","బ","భ","మ","య","ర","ల","వ","శ","ష","స","హ","ళ","క్ష","జ్ఞ",
    ];

    // Kannada
    pub const kannada_core: &[&str] = &[
        "ಅ","ಆ","ಇ","ಈ","ಉ","ಊ","ಋ","ೠ","ಌ","ೡ","ಎ","ಏ","ಐ","ಒ","ಓ","ಔ","ಂ","ಃ","್",
        "ಕ","ಖ","ಗ","ಘ","ಙ","ಚ","ಛ","ಜ","ಝ","ಞ","ಟ","ಠ","ಡ","ಢ","ಣ","ತ","ಥ","ದ","ಧ","ನ","ಪ","ಫ","ಬ","ಭ","ಮ","ಯ","ರ","ಲ","ವ","ಶ","ಷ","ಸ","ಹ","ಳ","ೞ","ಕ್ಷ","ಜ್ಞ",
    ];

    // Malayalam
    pub const malayalam_core: &[&str] = &[
        "അ","ആ","ഇ","ഈ","ഉ","ഊ","ഋ","ൠ","ഌ","ൡ","എ","ഏ","ഐ","ഒ","ഓ","ഔ","ം","ഃ","്",
        "ക","ഖ","ഗ","ഘ","ങ","ച","ഛ","ജ","ഝ","ഞ","ട","ഠ","ഡ","ഢ","ണ","ത","ഥ","ദ","ധ","ന","പ","ഫ","ബ","ഭ","മ","യ","ര","ല","വ","ശ","ഷ","സ","ഹ","ള","ഴ","റ","ക്ഷ","ജ്ഞ",
    ];

    // Sinhala
    pub const sinhala_core: &[&str] = &[
        "අ","ආ","ඉ","ඊ","උ","ඌ","ඍ","ඎ","ඏ","ඐ","එ","ඒ","ඓ","ඔ","ඕ","ඖ","ං","ඃ","්",
        "ක","ඛ","ග","ඝ","ඞ","ච","ඡ","ජ","ඣ","ඤ","ට","ඨ","ඩ","ඪ","ණ","ත","ථ","ද","ධ","න","ප","ඵ","බ","භ","ම","ය","ර","ල","ව","ශ","ෂ","ස","හ","ළ","ෆ","ක්‍ෂ",
    ];

    /* ------------------------------------- Thai & Lao -------------------------------- */
    pub const thai_core: &[&str] = &[
        "ก","ข","ฃ","ค","ฅ","ฆ","ง","จ","ฉ","ช","ซ","ฌ","ญ","ฎ","ฏ","ฐ","ฑ","ฒ","ณ","ด","ต","ถ","ท","ธ","น","บ","ป","ผ","ฝ","พ","ฟ","ภ","ม","ย","ร","ล","ฤ","ฦ","ว","ศ","ษ","ส","ห","ฬ","อ","ฮ",
        "ะ","า","ิ","ี","ึ","ื","ุ","ู","โ","ไ","ใ","เ","แ","ำ","ั","่","้","๊","๋","์","็","ๅ","ๆ",
    ];
    pub const lao_core: &[&str] = &[
        "ກ","ຂ","ຄ","ງ","ຈ","ຊ","ຍ","ດ","ຕ","ຖ","ທ","ນ","ບ","ປ","ຜ","ຝ","ພ","ຟ","ມ","ຢ","ຣ","ລ","ວ","ສ","ຫ","ອ","ຮ",
        "ະ","າ","ິ","ີ","ຶ","ື","ຸ","ູ","ໂ","ໄ","ໃ","ເ","ແ","ັ","່","້","໊","໋","໌","ໆ",
    ];

    /* ------------------------------------- Tibetan ----------------------------------- */
    pub const tibetan_core: &[&str] = &[
		"ཀ","ཁ","ག","ང","ཅ","ཆ","ཇ","ཉ","ཏ","ཐ","ད","ན","པ","ཕ","བ","མ",
		"ཙ","ཚ","ཛ","ཝ","ཞ","ཟ","འ","ཡ","ར","ལ","ཤ","ས","ཧ","ཨ",
		"་","།","༎",
		"གྷ",        // U+0F43 GHA
		"ཊ","ཋ","ཌ","ཌྷ","ཎ", // U+0F4A..U+0F4E retroflex series
		"དྷ","བྷ","ཛྷ",          // U+0F52 DHA, U+0F57 BHA, U+0F5C DZHA
		"ཥ",                  // U+0F65 SSA
		"ཀྵ","ཪ"               // U+0F69 KSSA, U+0F6A FIXED-FORM RA
	];


    /* ------------------------------------- Myanmar ----------------------------------- */
    pub const myanmar_core: &[&str] = &[
        "က","ခ","ဂ","ဃ","င","စ","ဆ","ဇ","ဈ","ည","ဋ","ဌ","ဍ","ဎ","ဏ","တ","ထ","ဒ","ဓ","န","ပ","ဖ","ဗ","ဘ","မ","ယ","ရ","လ","ဝ","သ","ဟ","ဠ","အ","ါ","ား","ိ","ီ","ု","ူ","ေ","ဲ","ံ","့","္",
    ];

    /* -------------------------------------- Khmer ------------------------------------ */
    pub const khmer_core: &[&str] = &[
        // Consonants
        "ក","ខ","គ","ឃ","ង","ច","ឆ","ជ","ឈ","ញ","ដ","ឋ","ឌ","ឍ","ណ","ត","ថ","ទ","ធ","ន",
        "ប","ផ","ព","ភ","ម","យ","រ","ល","វ","ឝ","ឞ","ស","ហ","ឡ","អ",
        // Dependent vowels & signs
        "ា","ិ","ី","ឹ","ឺ","ុ","ូ","ើ","ឿ","ោះ","ែ","ៃ","ោ","ៅ","ំ","ះ","់","៌","៍","៎","៏","័","៉","៊",
    ];

    /* --------------------------- Extra Latin extensions --------------------------- */
    
    pub const latin_clicks: &[&str] = &["ǀ","ǁ","ǂ","ǃ"];
   // --- Vietnamese (split) ---
	pub const vietnamese_upper: &[&str] = &[
		"A","Á","À","Ả","Ã","Ạ","Ă","Ắ","Ằ","Ẳ","Ẵ","Ặ","Â","Ấ","Ầ","Ẩ","Ẫ","Ậ",
		"E","É","È","Ẻ","Ẽ","Ẹ","Ê","Ế","Ề","Ể","Ễ","Ệ",
		"I","Í","Ì","Ỉ","Ĩ","Ị",
		"O","Ó","Ò","Ỏ","Õ","Ọ","Ô","Ố","Ồ","Ổ","Ỗ","Ộ","Ơ","Ớ","Ờ","Ở","Ỡ","Ợ",
		"U","Ú","Ù","Ủ","Ũ","Ụ","Ư","Ứ","Ừ","Ử","Ữ","Ự",
		"Y","Ý","Ỳ","Ỷ","Ỹ","Ỵ",
		"Đ",
	];
	pub const vietnamese_lower: &[&str] = &[
		"a","á","à","ả","ã","ạ","ă","ắ","ằ","ẳ","ẵ","ặ","â","ấ","ầ","ẩ","ẫ","ậ",
		"e","é","è","ẻ","ẽ","ẹ","ê","ế","ề","ể","ễ","ệ",
		"i","í","ì","ỉ","ĩ","ị",
		"o","ó","ò","ỏ","õ","ọ","ô","ố","ồ","ổ","ỗ","ộ","ơ","ớ","ờ","ở","ỡ","ợ",
		"u","ú","ù","ủ","ũ","ụ","ư","ứ","ừ","ử","ữ","ự",
		"y","ý","ỳ","ỷ","ỹ","ỵ",
		"đ",
	];

	// --- Esperanto (split) ---
	pub const esperanto_upper: &[&str] = &["Ĉ","Ĝ","Ĥ","Ĵ","Ŝ","Ŭ"];
	pub const esperanto_lower: &[&str] = &["ĉ","ĝ","ĥ","ĵ","ŝ","ŭ"];

	// --- Yoruba (split) ---
	pub const yoruba_upper: &[&str] = &["Ẹ","Ọ","Ṣ"];
	pub const yoruba_lower: &[&str] = &["ẹ","ọ","ṣ"];

	// --- Yoruba tone sequences (split) ---
	pub const yoruba_tone_seqs_upper: &[&str] = &[
		"Á","À","Ā","É","È","Ē","Í","Ì","Ī","Ó","Ò","Ō","Ú","Ù","Ū",
		"Ẹ́","Ẹ̀","Ẹ̄","Ọ́","Ọ̀","Ọ̄",
		"Ń","Ǹ","N̄","Ḿ","M̀","M̄",
	];
	pub const yoruba_tone_seqs_lower: &[&str] = &[
		"á","à","ā","é","è","ē","í","ì","ī","ó","ò","ō","ú","ù","ū",
		"ẹ́","ẹ̀","ẹ̄","ọ́","ọ̀","ọ̄",
		"ń","ǹ","n̄","ḿ","m̀","m̄",
	];

	// --- Kikuyu (split) ---
	pub const kikuyu_upper: &[&str] = &["Ĩ","Ũ"];
	pub const kikuyu_lower: &[&str] = &["ĩ","ũ"];

	// --- Salish (split) ---
	pub const latin_salish_upper: &[&str] = &["Ḵ"];
	pub const latin_salish_lower: &[&str] = &[
		"ḵ",
		"k\u{0331}","g\u{0331}","q\u{0331}","x\u{0331}",
		"k\u{02BC}","q\u{02BC}","x\u{02BC}","t\u{02BC}","p\u{02BC}","s\u{02BC}",
		"k\u{0331}\u{02BC}","q\u{0331}\u{02BC}","x\u{0331}\u{02BC}",
	];

	// --- Latin extended ogonek (split) ---
	pub const latin_extended_ogonek_upper: &[&str] = &["Ǫ","Ǭ"];
	pub const latin_extended_ogonek_lower: &[&str] = &["ǫ","ǭ"];

	// --- Romanian legacy cedilla (split) ---
	pub const romanian_legacy_cedilla_upper: &[&str] = &["Ş","Ţ"];
	pub const romanian_legacy_cedilla_lower: &[&str] = &["ş","ţ"];

	// --- Africanist (split) ---
	pub const latin_african_upper: &[&str] = &[
		"Ɓ","Ɗ","Ɠ","Ƙ","Ƴ","Ɛ","Ɔ","Ʋ","Ŋ","Ɣ","Ƈ","Ƥ","Ƭ","Ɲ","Ɩ","Ɨ","Ƒ",
	];
	pub const latin_african_lower: &[&str] = &[
		"ɓ","ɗ","ɠ","ƙ","ƴ","ɛ","ɔ","ʋ","ŋ","ɣ","ƈ","ƥ","ƭ","ɲ","ɩ","ɨ","ƒ",
	];

    
        /* ================================ Digits ================================ */
    // Western (ASCII) digits
    pub const ascii_digits: &[&str] = &["0","1","2","3","4","5","6","7","8","9"];

    // Arabic-script digit systems
    pub const arabic_indic_digits: &[&str] = &["٠","١","٢","٣","٤","٥","٦","٧","٨","٩"]; // U+0660..0669
    pub const extended_arabic_indic_digits: &[&str] = &["۰","۱","۲","۳","۴","۵","۶","۷","۸","۹"]; // U+06F0..06F9 (Persian/Urdu)

    // Indic-family digit systems (for scripts you already include)
    pub const devanagari_digits: &[&str] = &["०","१","२","३","४","५","६","७","८","९"];
    pub const bengali_digits:   &[&str] = &["০","১","২","৩","৪","৫","৬","৭","৮","৯"];
    pub const gurmukhi_digits:  &[&str] = &["੦","੧","੨","੩","੪","੫","੬","੭","੮","੯"];
    pub const gujarati_digits:  &[&str] = &["૦","૧","૨","૩","૪","૫","૬","૭","૮","૯"];
    pub const odia_digits:      &[&str] = &["୦","୧","୨","୩","୪","୫","୬","୭","୮","୯"];
    pub const tamil_digits:     &[&str] = &["௦","௧","௨","௩","௪","௫","௬","௭","௮","௯"];
    pub const telugu_digits:    &[&str] = &["౦","౧","౨","౩","౪","౫","౬","౭","౮","౯"];
    pub const kannada_digits:   &[&str] = &["೦","೧","೨","೩","೪","೫","೬","೭","೮","೯"];
    pub const malayalam_digits: &[&str] = &["൦","൧","൨","൩","൪","൫","൬","൭","൮","൯"];
    pub const sinhala_digits:   &[&str] = &["෦","෧","෨","෩","෪","෫","෬","෭","෮","෯"];

    // SE & East/South digits
    pub const thai_digits:      &[&str] = &["๐","๑","๒","๓","๔","๕","๖","๗","๘","๙"];
    pub const lao_digits:       &[&str] = &["໐","໑","໒","໓","໔","໕","໖","໗","໘","໙"];
    pub const tibetan_digits:   &[&str] = &["༠","༡","༢","༣","༤","༥","༦","༧","༨","༩"];
    pub const myanmar_digits:   &[&str] = &["၀","၁","၂","၃","၄","၅","၆","၇","၈","၉"];
    pub const khmer_digits:     &[&str] = &["០","១","២","៣","៤","៥","៦","៧","៨","៩"];

    // Superscript/subscript digits (+ a few signs often needed in math text)
    pub const superscript_digits_core: &[&str] = &[
        "⁰","¹","²","³","⁴","⁵","⁶","⁷","⁸","⁹","⁺","⁻","⁼","⁽","⁾"
    ];
    pub const subscript_digits_core: &[&str] = &[
        "₀","₁","₂","₃","₄","₅","₆","₇","₈","₉","₊","₋","₌","₍","₎"
    ];

    /* =========================== Math symbols (tiers) =========================== */
    // Basic arithmetic & grouping (keep hyphen-minus here; typographic minus is separate)
    pub const math_basic_arithmetic: &[&str] = &[
        "+","-","*","×","÷","/","=","%", "^", "(",")","[","]"
    ];

    // Language/context variants of basic ops
    pub const math_fullwidth_basic_east_asian: &[&str] = &[
        "＋","－","＊","／","＝","％","＾","（","）","［","］"
    ];
    pub const math_arabic_numeric_symbols: &[&str] = &[
        "٪","٫","٬" // Arabic percent; decimal sep; thousands sep
    ];

    // Typographic minus (U+2212) & common dot operators (LaTeX \cdot)
    pub const math_typographic_variants: &[&str] = &[
        "−","∙","⋅","·" // minus sign; bullet operator; dot operator; middle dot
    ];

	/* ---------- Astronomy symbols (minimal planetary set) ---------- */
	pub const symbols_astro_basic: &[&str] = &[
		    "☉","☽","☿","♀","♂","♃","♄","♅","♆","♇","♁","⊕",
	];

	/* ---------- Latin small-caps that show up in the wild ---------- */
	/* (You can extend this later; these three are the ones you hit.) */
	pub const latin_small_caps_extended: &[&str] = &["ᴀ","ꜱ","ᴢ", // the ones you saw
    "ᴄ","ᴅ","ᴊ","ᴋ","ᴍ","ᴏ","ᴘ","ᴛ","ᴜ","ᴠ","ᴡ","ᴇ",
    "ʙ","ʀ","ʏ","ɢ","ɴ","ʟ","ʜ",];

	/* ---------- Braille patterns (minimal: the ones you actually saw) ---------- */
	/* If you want full U+2800–U+28FF later, generate them in build.rs. */
	// println("[", join(["\"$(Char(cp))\"" for cp in 0x2800:0x28FF], ","), "]")
	pub const braille_patterns: &[&str] = &["⠀","⠁","⠂","⠃","⠄","⠅","⠆","⠇","⠈","⠉","⠊","⠋","⠌","⠍","⠎","⠏","⠐","⠑","⠒","⠓","⠔","⠕","⠖","⠗","⠘","⠙","⠚","⠛","⠜","⠝","⠞","⠟","⠠","⠡","⠢","⠣","⠤","⠥","⠦","⠧","⠨","⠩","⠪","⠫","⠬","⠭","⠮","⠯","⠰","⠱","⠲","⠳","⠴","⠵","⠶","⠷","⠸","⠹","⠺","⠻","⠼","⠽","⠾","⠿","⡀","⡁","⡂","⡃","⡄","⡅","⡆","⡇","⡈","⡉","⡊","⡋","⡌","⡍","⡎","⡏","⡐","⡑","⡒","⡓","⡔","⡕","⡖","⡗","⡘","⡙","⡚","⡛","⡜","⡝","⡞","⡟","⡠","⡡","⡢","⡣","⡤","⡥","⡦","⡧","⡨","⡩","⡪","⡫","⡬","⡭","⡮","⡯","⡰","⡱","⡲","⡳","⡴","⡵","⡶","⡷","⡸","⡹","⡺","⡻","⡼","⡽","⡾","⡿","⢀","⢁","⢂","⢃","⢄","⢅","⢆","⢇","⢈","⢉","⢊","⢋","⢌","⢍","⢎","⢏","⢐","⢑","⢒","⢓","⢔","⢕","⢖","⢗","⢘","⢙","⢚","⢛","⢜","⢝","⢞","⢟","⢠","⢡","⢢","⢣","⢤","⢥","⢦","⢧","⢨","⢩","⢪","⢫","⢬","⢭","⢮","⢯","⢰","⢱","⢲","⢳","⢴","⢵","⢶","⢷","⢸","⢹","⢺","⢻","⢼","⢽","⢾","⢿","⣀","⣁","⣂","⣃","⣄","⣅","⣆","⣇","⣈","⣉","⣊","⣋","⣌","⣍","⣎","⣏","⣐","⣑","⣒","⣓","⣔","⣕","⣖","⣗","⣘","⣙","⣚","⣛","⣜","⣝","⣞","⣟","⣠","⣡","⣢","⣣","⣤","⣥","⣦","⣧","⣨","⣩","⣪","⣫","⣬","⣭","⣮","⣯","⣰","⣱","⣲","⣳","⣴","⣵","⣶","⣷","⣸","⣹","⣺","⣻","⣼","⣽","⣾","⣿"];

	/* ---------- Music symbols ---------- */
	pub const symbols_music_basic: &[&str] = &["♯","♭","♮"];

	/* ---------- Currency symbols (extended, common) ---------- */
	pub const punctuation_currency_extended: &[&str] = &[
		  "¢","£","€","¥","¤",
    "₱","₪","₹","₨","₩","₫","₽","₴","₺","₵",
    "₣","₡","₭","₮","₲","₳","₸","₼","₾","₥","₢","₤",
	];

	/* ---------- Letterlike symbols that behave like punctuation/tokens ---------- */
	pub const punctuation_letterlike_symbols: &[&str] = &[
		"℻", "◌", // U+25CC dotted circle
    "№","℀","℁","℅","℆",
    "℞","℗","℡","℠","™"
	];

	/* ---------- Prime marks (feet/minutes/derivatives) ---------- */
	pub const punctuation_primes_basic: &[&str] = &["′","″","‴","⁗","‵","‶","‷"];
	pub const punctuation_sections_basic: &[&str] = &["§","¶"];
	/* ---------- Hawaiian packs ---------- */
	/* Hawaiian considers the okina (ʻ U+02BB) a LETTER; include it here. */
	/* Macrons (kahakō) on AEIOU are part of standard orthography. */
	pub const hawaiian_core_upper: &[&str] = &[
		"A","E","I","O","U","H","K","L","M","N","P","W",
		"Ā","Ē","Ī","Ō","Ū",
	];
	pub const hawaiian_core_lower: &[&str] = &[
		"a","e","i","o","u","h","k","l","m","n","p","w",
		"ā","ē","ī","ō","ū",
	];
	pub const hawaiian_okina: &[&str] = &["ʻ"]; // U+02BB MODIFIER LETTER TURNED COMMA


    // Algebra & inequalities
    pub const math_algebra_core: &[&str] = &[
        "±","∓","<",">","≤","≥","≠","≈","∼","∣","∥"
    ];

    // Sets & relations
    pub const math_set_relations: &[&str] = &[
        "∈","∉","∋","⊂","⊄","⊆","⊇","⊃","∪","∩","∅","≡","≅","≃","≍","≔","≕","≜","∝"
    ];

    // Calculus & analysis
    pub const math_calculus_core: &[&str] = &[
        "∑","∏","∫","∬","∮","∂","∇","∞","√","∆"
    ];

    // Logic/connectives & entailment
    pub const math_logic_core: &[&str] = &[
        "¬","∧","∨","⇒","⇔","∀","∃","∴","∵","⊢","⊨"
    ];

    // Arrows (LaTeX-friendly split)
    pub const math_arrows_basic: &[&str] = &[
        "→","←","↔","↑","↓"
    ];
    pub const math_arrows_extended: &[&str] = &[
        "⇒","⇐","⇔","↦","↣","↩","↪","↗","↘","↙","↖"
    ];

    // Delimiters (LaTeX: \langle \rangle \lfloor \rfloor …)
    pub const math_delimiters_extended: &[&str] = &[
        "⟨","⟩","〈","〉","⌈","⌉","⌊","⌋","⎡","⎤","⎣","⎦"
    ];

    // Linear algebra / group theory style operators
    pub const math_operators_extended: &[&str] = &[
        "⊕","⊖","⊗","⊘","⊙","⊚","⊛","⊞","⊟","⊠"
    ];

    // Double-struck (common sets in LaTeX: \mathbb)
    pub const math_double_struck_sets: &[&str] = &[
        "ℕ","ℤ","ℚ","ℝ","ℂ","ℍ","ℙ"
    ];

    /* ============================== Punctuation (tiers) ============================== */
    // Word-level (Latin contexts): add underscore here (requested)
    // Include U+02BC (modifier apostrophe) because you use it in Salish/IPA.
    pub const punctuation_word_basic_latin: &[&str] = &[
        "_","-","'","’","ʼ","`","·"
    ];

    // Standalone underscore packs (if you want explicit control)
    pub const punctuation_underscore_basic: &[&str] = &["_"];
    pub const punctuation_underscore_extended: &[&str] = &["_","‗","﹍","﹎","﹏"]; // low line, double low line, dashed, center, wavy

    // Sentence-level (Latin contexts): core marks, quotes, dashes, slashes, common symbols
    pub const punctuation_sentence_basic_latin: &[&str] = &[
        ".",
        ",",
        ";",
        ":",
        "?",
        "!",
        "…",
        "(",")","[","]","{","}",
        "\"","“","”","‘","’",
        "–","—",
        "/","\\",
        "&","@",
        "$","~","|"
    ];


    // Iberian extras
    pub const punctuation_spanish_extras: &[&str] = &["¿","¡"];

    // Greek-specific
    pub const punctuation_greek_ano_teleia: &[&str] = &["·"]; // ano teleia

    // Hebrew-specific
    pub const punctuation_hebrew_core: &[&str] = &["־","׃","׳","״"]; // maqaf, sof pasuq, geresh, gershayim

    // Arabic-specific
    pub const punctuation_arabic_core: &[&str] = &["،","؛","؟"];

    // Devanagari-specific
    pub const punctuation_devanagari_core: &[&str] = &["।","॥"]; // danda, double danda

    // Japanese/CJK punctuation commonly used with Kana
    pub const punctuation_japanese_core: &[&str] = &["、","。","・","「","」","『","』","〜","ー"];

    // Thai & Lao
    pub const punctuation_thai_core: &[&str] = &["ๆ","ฯ"];
    pub const punctuation_lao_core:  &[&str] = &["ໆ"];

    // Tibetan (also present in your tibetan_core, but exposed here as a punctuation pack)
    pub const punctuation_tibetan_core: &[&str] = &["་","།","༎"];

    // Myanmar
    pub const punctuation_myanmar_core: &[&str] = &["၊","။"];

    // Khmer
    pub const punctuation_khmer_core: &[&str] = &["។","៕","៖","ៗ"];

    /* ============================== Dashes / Hyphens ============================== */
    // Basic: hyphen-minus, en dash, em dash (what most signage/typography needs)
    pub const punctuation_dashes_basic: &[&str] = &[
        "-",    // U+002D hyphen-minus
        "–",    // U+2013 en dash
        "—",    // U+2014 em dash
    ];

    // Extended: add true hyphen, non-breaking hyphen, figure dash, horizontal bar
    pub const punctuation_dashes_extended: &[&str] = &[
        "-",    // U+002D hyphen-minus
        "‐",    // U+2010 hyphen
        "-",    // U+2011 non-breaking hyphen
        "‒",    // U+2012 figure dash
        "–",    // U+2013 en dash
        "—",    // U+2014 em dash
        "―",    // U+2015 horizontal bar
    ];

    // All: include editorial two-em/three-em dashes and soft hyphen
    pub const punctuation_dashes_all: &[&str] = &[
        "-",    // U+002D hyphen-minus
        "‐",    // U+2010 hyphen
        "-",    // U+2011 non-breaking hyphen
        "‒",    // U+2012 figure dash
        "–",    // U+2013 en dash
        "—",    // U+2014 em dash
        "―",    // U+2015 horizontal bar
        "⸺",    // U+2E3A two-em dash
        "⸻",    // U+2E3B three-em dash
        "­",     // U+00AD soft hyphen (discretionary)
    ];

}

/* ============================== Package catalog ============================== */

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Package {
    // Latin cores
    LatinAsciiUpper,
    LatinAsciiLower,
    SpanishCoreUpper,
    SpanishCoreLower,
    FrenchAccentsUpper,
    FrenchAccentsLower,
    GermanExtrasUpper,
    GermanExtrasLower,
    ItalianCoreUpper,
    ItalianCoreLower,
    PortugueseCoreUpper,
    PortugueseCoreLower,
    DanishNorwegianUpper,
    DanishNorwegianLower,
    SwedishUpper,
    SwedishLower,
    IcelandicUpper,
    IcelandicLower,
    LithuanianUpper,
    LithuanianLower,
    LatvianUpper,
    LatvianLower,
    CzechUpper,
    CzechLower,
    SlovakUpper,
    SlovakLower,
    SloveneBCSUpper,
    SloveneBCSLower,
    HungarianUpper,
    HungarianLower,
    PolishUpper,
    PolishLower,
    RomanianUpper,
    RomanianLower,
    WelshUpper,
    WelshLower,
    GaelicUpper,
    GaelicLower,
    MalteseUpper,
    MalteseLower,
    TurkishAzeriUpper,
    TurkishAzeriLower,
    GreenlandicUpper,
    GreenlandicLower,
    LatinLigaturesUpper,
    LatinLigaturesLower,

    // Latin digraphs
    LatinDigraphsCommon,
    LatinDigraphsCatalan,
    LatinDigraphsMaltese,
    LatinDigraphsAll,

    // Latin extensions
    LatinIpaCore,
    LatinIpaModifiers,
    LatinClicks,
    
    // Pinyin
    LatinPinyinUpper,
    LatinPinyinLower,

    // Greek
    GreekCaps,
    GreekLowerBase,
    GreekMonotonicMarksUpper,
    GreekMonotonicMarksLower,

    // Cyrillic
    CyrillicRuCaps,
    CyrillicRuLower,
    CyrillicBgCaps,
    CyrillicBgLower,
    CyrillicUkCaps,
    CyrillicUkLower,
    CyrillicBeCaps,
    CyrillicBeLower,
    CyrillicSrMeCaps,
    CyrillicSrMeLower,
    CyrillicMkCaps,
    CyrillicMkLower,

    // Armenian
    ArmenianCaps,
    ArmenianLower,

    // Georgian
    GeorgianMkhedruli,
    GeorgianMtavruli,

    // Hebrew
    Hebrew,

    // Cherokee
    CherokeeUpper,
    CherokeeSmall,

    // Canadian syllabics
    InuktitutCore,
    CreeCore,

    // Japanese kana
    Hiragana,
    Katakana,

    // Hangul Jamo
    HangulJamoL,
    HangulJamoV,
    HangulJamoT,
    HangulJamoAll,

    // Ethiopic
    EthiopicCore,

    // Arabic & friends
    ArabicCore,
    PersianUrduExtras,

    // Syriac
    SyriacCore,

    // Thaana
    ThaanaCore,

    // Indic cores
    DevanagariCore,
    BengaliCore,
    GurmukhiCore,
    GujaratiCore,
    OdiaCore,
    TamilCore,
    TeluguCore,
    KannadaCore,
    MalayalamCore,
    SinhalaCore,

    // Thai & Lao
    ThaiCore,
    LaoCore,

    // Tibetan, Myanmar, Khmer
    TibetanCore,
    MyanmarCore,
    KhmerCore,
    
    LatinSalishUpper,
    LatinSalishLower,
    LatinExtendedOgonekUpper,
    LatinExtendedOgonekLower,
    RomanianLegacyCedillaUpper,
    RomanianLegacyCedillaLower,
    LatinAfricanUpper,
    LatinAfricanLower,
    YorubaUpper,
    YorubaLower,
    KikuyuUpper,
    KikuyuLower,
    EsperantoUpper,
    EsperantoLower,
    VietnameseUpper,
    VietnameseLower,
    YorubaTonesUpper,
    YorubaTonesLower,
    
    
        // Digits
    DigitsAscii,
    DigitsArabicIndic,
    DigitsExtendedArabicIndic,
    DigitsDevanagari,
    DigitsBengali,
    DigitsGurmukhi,
    DigitsGujarati,
    DigitsOdia,
    DigitsTamil,
    DigitsTelugu,
    DigitsKannada,
    DigitsMalayalam,
    DigitsSinhala,
    DigitsThai,
    DigitsLao,
    DigitsTibetan,
    DigitsMyanmar,
    DigitsKhmer,
    DigitsSuperscriptCore,
    DigitsSubscriptCore,

    // Math (core + variants)
    MathBasicArithmetic,
    MathFullwidthBasicEastAsian,
    MathArabicNumericSymbols,
    MathTypographicVariants,
    MathAlgebraCore,
    MathSetRelations,
    MathCalculusCore,
    MathLogicCore,
    MathArrowsBasic,
    MathArrowsExtended,
    MathDelimitersExtended,
    MathOperatorsExtended,
    MathDoubleStruckSets,

    // Punctuation
    PunctuationWordBasicLatin,
    PunctuationUnderscoreBasic,
    PunctuationUnderscoreExtended,
    PunctuationSentenceBasicLatin,
    PunctuationSpanishExtras,
    PunctuationGreekAnoTeleia,
    PunctuationHebrewCore,
    PunctuationArabicCore,
    PunctuationDevanagariCore,
    PunctuationJapaneseCore,
    PunctuationThaiCore,
    PunctuationLaoCore,
    PunctuationTibetanCore,
    PunctuationMyanmarCore,
    PunctuationKhmerCore,
    PunctuationDashesBasic,
    PunctuationDashesExtended,
    PunctuationDashesAll,
    
    SymbolsAstroBasic,
    LatinSmallCapsExtended,
    BraillePatterns,
    SymbolsMusicBasic,
    PunctuationCurrencyExtended,
    PunctuationLetterlikeSymbols,
    PunctuationPrimesBasic,
    HawaiianCoreUpper,
    HawaiianCoreLower,
    HawaiianOkina,
    PunctuationSectionsBasic
    
}

impl Package {
    pub fn tokens(self) -> &'static [&'static str] {
        use sets::*;
        match self {
			Package::PunctuationSectionsBasic => punctuation_sections_basic,
            // Latin
            Package::LatinAsciiUpper => latin_ascii_upper,
            Package::LatinAsciiLower => latin_ascii_lower,
            Package::SpanishCoreUpper => spanish_core_upper,
            Package::SpanishCoreLower => spanish_core_lower,
            Package::FrenchAccentsUpper => french_accents_upper,
            Package::FrenchAccentsLower => french_accents_lower,
            Package::GermanExtrasUpper => german_extras_upper,
            Package::GermanExtrasLower => german_extras_lower,
            Package::ItalianCoreUpper => italian_core_upper,
            Package::ItalianCoreLower => italian_core_lower,
            Package::PortugueseCoreUpper => portuguese_core_upper,
            Package::PortugueseCoreLower => portuguese_core_lower,
            Package::DanishNorwegianUpper => danish_norwegian_upper,
            Package::DanishNorwegianLower => danish_norwegian_lower,
            Package::SwedishUpper => swedish_upper,
            Package::SwedishLower => swedish_lower,
            Package::IcelandicUpper => icelandic_upper,
            Package::IcelandicLower => icelandic_lower,
            Package::LithuanianUpper => lithuanian_upper,
            Package::LithuanianLower => lithuanian_lower,
            Package::LatvianUpper => latvian_upper,
            Package::LatvianLower => latvian_lower,
            Package::CzechUpper => czech_upper,
            Package::CzechLower => czech_lower,
            Package::SlovakUpper => slovak_upper,
            Package::SlovakLower => slovak_lower,
            Package::SloveneBCSUpper => slovene_bcs_upper,
            Package::SloveneBCSLower => slovene_bcs_lower,
            Package::HungarianUpper => hungarian_upper,
            Package::HungarianLower => hungarian_lower,
            Package::PolishUpper => polish_upper,
            Package::PolishLower => polish_lower,
            Package::RomanianUpper => romanian_upper,
            Package::RomanianLower => romanian_lower,
            Package::WelshUpper => welsh_upper,
            Package::WelshLower => welsh_lower,
            Package::GaelicUpper => gaelic_upper,
            Package::GaelicLower => gaelic_lower,
            Package::MalteseUpper => maltese_upper,
            Package::MalteseLower => maltese_lower,
            Package::TurkishAzeriUpper => turkish_azeri_upper,
            Package::TurkishAzeriLower => turkish_azeri_lower,
            Package::GreenlandicUpper => greenlandic_upper,
            Package::GreenlandicLower => greenlandic_lower,
            Package::LatinLigaturesUpper => latin_ligatures_upper,
            Package::LatinLigaturesLower => latin_ligatures_lower,

            // Latin digraphs
            Package::LatinDigraphsCommon => latin_digraphs_common,
            Package::LatinDigraphsCatalan => latin_digraphs_catalan,
            Package::LatinDigraphsMaltese => latin_digraphs_maltese,
            Package::LatinDigraphsAll => latin_digraphs_all,

            // Latin extensions
            Package::LatinIpaCore => latin_ipa_core,
            Package::LatinIpaModifiers => latin_ipa_modifiers,
            Package::LatinClicks => latin_clicks,
  
            // Pinyin
            Package::LatinPinyinUpper => latin_pinyin_upper,
            Package::LatinPinyinLower => latin_pinyin_lower,

            // Greek
            Package::GreekCaps => greek_caps,
            Package::GreekLowerBase => greek_lower_base,
            Package::GreekMonotonicMarksUpper => greek_monotonic_marks_upper,
            Package::GreekMonotonicMarksLower => greek_monotonic_marks_lower,

            // Cyrillic
            Package::CyrillicRuCaps => cyrillic_ru_caps,
            Package::CyrillicRuLower => cyrillic_ru_lower,
            Package::CyrillicBgCaps => cyrillic_bg_caps,
            Package::CyrillicBgLower => cyrillic_bg_lower,
            Package::CyrillicUkCaps => cyrillic_uk_caps,
            Package::CyrillicUkLower => cyrillic_uk_lower,
            Package::CyrillicBeCaps => cyrillic_be_caps,
            Package::CyrillicBeLower => cyrillic_be_lower,
            Package::CyrillicSrMeCaps => cyrillic_sr_me_caps,
            Package::CyrillicSrMeLower => cyrillic_sr_me_lower,
            Package::CyrillicMkCaps => cyrillic_mk_caps,
            Package::CyrillicMkLower => cyrillic_mk_lower,

            // Armenian
            Package::ArmenianCaps => armenian_caps,
            Package::ArmenianLower => armenian_lower,

            // Georgian
            Package::GeorgianMkhedruli => georgian_mkhedruli,
            Package::GeorgianMtavruli => georgian_mtavruli,

            // Hebrew
            Package::Hebrew => hebrew_letters,

            // Cherokee
            Package::CherokeeUpper => cherokee_upper,
            Package::CherokeeSmall => cherokee_small,

            // CAS
            Package::InuktitutCore => inuktitut_core,
            Package::CreeCore => cree_core,

            // Kana
            Package::Hiragana => hiragana,
            Package::Katakana => katakana,

            // Hangul Jamo
            Package::HangulJamoL => hangul_jamo_l,
            Package::HangulJamoV => hangul_jamo_v,
            Package::HangulJamoT => hangul_jamo_t,
            Package::HangulJamoAll => hangul_jamo_all,

            // Ethiopic
            Package::EthiopicCore => ethiopic_core,

            // Arabic & friends
            Package::ArabicCore => arabic_core,
            Package::PersianUrduExtras => persian_urdu_extras,

            // Syriac
            Package::SyriacCore => syriac_core,

            // Thaana
            Package::ThaanaCore => thaana_core,

            // Indic cores
            Package::DevanagariCore => devanagari_core,
            Package::BengaliCore => bengali_core,
            Package::GurmukhiCore => gurmukhi_core,
            Package::GujaratiCore => gujarati_core,
            Package::OdiaCore => odia_core,
            Package::TamilCore => tamil_core,
            Package::TeluguCore => telugu_core,
            Package::KannadaCore => kannada_core,
            Package::MalayalamCore => malayalam_core,
            Package::SinhalaCore => sinhala_core,

            // Thai & Lao
            Package::ThaiCore => thai_core,
            Package::LaoCore => lao_core,

            // Tibetan, Myanmar, Khmer
            Package::TibetanCore => tibetan_core,
            Package::MyanmarCore => myanmar_core,
            Package::KhmerCore => khmer_core,
            
            // Latin extensions (split)
            Package::LatinSalishUpper => latin_salish_upper,
            Package::LatinSalishLower => latin_salish_lower,
            Package::LatinExtendedOgonekUpper => latin_extended_ogonek_upper,
            Package::LatinExtendedOgonekLower => latin_extended_ogonek_lower,
            Package::RomanianLegacyCedillaUpper => romanian_legacy_cedilla_upper,
            Package::RomanianLegacyCedillaLower => romanian_legacy_cedilla_lower,
            Package::LatinAfricanUpper => latin_african_upper,
            Package::LatinAfricanLower => latin_african_lower,
            Package::YorubaUpper => yoruba_upper,
            Package::YorubaLower => yoruba_lower,
            Package::KikuyuUpper => kikuyu_upper,
            Package::KikuyuLower => kikuyu_lower,
            Package::EsperantoUpper => esperanto_upper,
            Package::EsperantoLower => esperanto_lower,
            Package::VietnameseUpper => vietnamese_upper,
            Package::VietnameseLower => vietnamese_lower,
            Package::YorubaTonesUpper => yoruba_tone_seqs_upper,
            Package::YorubaTonesLower => yoruba_tone_seqs_lower,
            
                        // Digits
            Package::DigitsAscii                 => ascii_digits,
            Package::DigitsArabicIndic           => arabic_indic_digits,
            Package::DigitsExtendedArabicIndic   => extended_arabic_indic_digits,
            Package::DigitsDevanagari            => devanagari_digits,
            Package::DigitsBengali               => bengali_digits,
            Package::DigitsGurmukhi              => gurmukhi_digits,
            Package::DigitsGujarati              => gujarati_digits,
            Package::DigitsOdia                  => odia_digits,
            Package::DigitsTamil                 => tamil_digits,
            Package::DigitsTelugu                => telugu_digits,
            Package::DigitsKannada               => kannada_digits,
            Package::DigitsMalayalam             => malayalam_digits,
            Package::DigitsSinhala               => sinhala_digits,
            Package::DigitsThai                  => thai_digits,
            Package::DigitsLao                   => lao_digits,
            Package::DigitsTibetan               => tibetan_digits,
            Package::DigitsMyanmar               => myanmar_digits,
            Package::DigitsKhmer                 => khmer_digits,
            Package::DigitsSuperscriptCore       => superscript_digits_core,
            Package::DigitsSubscriptCore         => subscript_digits_core,

            // Math
            Package::MathBasicArithmetic         => math_basic_arithmetic,
            Package::MathFullwidthBasicEastAsian => math_fullwidth_basic_east_asian,
            Package::MathArabicNumericSymbols    => math_arabic_numeric_symbols,
            Package::MathTypographicVariants     => math_typographic_variants,
            Package::MathAlgebraCore             => math_algebra_core,
            Package::MathSetRelations            => math_set_relations,
            Package::MathCalculusCore            => math_calculus_core,
            Package::MathLogicCore               => math_logic_core,
            Package::MathArrowsBasic             => math_arrows_basic,
            Package::MathArrowsExtended          => math_arrows_extended,
            Package::MathDelimitersExtended      => math_delimiters_extended,
            Package::MathOperatorsExtended       => math_operators_extended,
            Package::MathDoubleStruckSets        => math_double_struck_sets,

            // Punctuation
            Package::PunctuationWordBasicLatin       => punctuation_word_basic_latin,
            Package::PunctuationUnderscoreBasic      => punctuation_underscore_basic,
            Package::PunctuationUnderscoreExtended   => punctuation_underscore_extended,
            Package::PunctuationSentenceBasicLatin   => punctuation_sentence_basic_latin,
            Package::PunctuationSpanishExtras        => punctuation_spanish_extras,
            Package::PunctuationGreekAnoTeleia       => punctuation_greek_ano_teleia,
            Package::PunctuationHebrewCore           => punctuation_hebrew_core,
            Package::PunctuationArabicCore           => punctuation_arabic_core,
            Package::PunctuationDevanagariCore       => punctuation_devanagari_core,
            Package::PunctuationJapaneseCore         => punctuation_japanese_core,
            Package::PunctuationThaiCore             => punctuation_thai_core,
            Package::PunctuationLaoCore              => punctuation_lao_core,
            Package::PunctuationTibetanCore          => punctuation_tibetan_core,
            Package::PunctuationMyanmarCore          => punctuation_myanmar_core,
            Package::PunctuationKhmerCore            => punctuation_khmer_core,
            Package::PunctuationDashesBasic          => punctuation_dashes_basic,
            Package::PunctuationDashesExtended       => punctuation_dashes_extended,
            Package::PunctuationDashesAll            => punctuation_dashes_all,


			Package::SymbolsAstroBasic      => symbols_astro_basic,
            Package::LatinSmallCapsExtended        => latin_small_caps_extended,
            Package::BraillePatterns               => braille_patterns,
            Package::SymbolsMusicBasic             => symbols_music_basic,
            Package::PunctuationCurrencyExtended   => punctuation_currency_extended,
            Package::PunctuationLetterlikeSymbols  => punctuation_letterlike_symbols,
            Package::PunctuationPrimesBasic        => punctuation_primes_basic,

            // Hawaiian
            Package::HawaiianCoreUpper             => hawaiian_core_upper,
            Package::HawaiianCoreLower             => hawaiian_core_lower,
            Package::HawaiianOkina                 => hawaiian_okina,

        }
    }
}

/* --------------------------- Presets (signage-first) --------------------------- */

#[derive(Clone, Debug)]
pub struct Preset {
    pub name: &'static str,
    pub packages: &'static [Package],
    pub drop_diacritics: Option<bool>,
    pub keep_ligatures: Option<bool>,
    pub case_mode: Option<&'static str>, // "CapsOnly" | "UpperLower" | "LowerOnly"
    pub german_sharp_s_uppercase: Option<&'static str>, // "SS" | "ẞ"
}

pub const PRESETS: &[Preset] = &[
    /* ============================ Existing (kept) ============================ */

    // Minimal English caps
    Preset {
        name: "english_caps",
        packages: &[Package::LatinAsciiUpper],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // Spanish signage style: A–Z + Ñ caps, drop accents
    Preset {
        name: "spanish_caps_min",
        packages: &[
            Package::LatinAsciiUpper,
            Package::SpanishCoreUpper
        ],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // Pan-Western caps, keep accents (no ligatures)
    Preset {
        name: "pan_western_caps",
        packages: &[
            Package::LatinAsciiUpper,
            Package::SpanishCoreUpper,
            Package::FrenchAccentsUpper,
            Package::GermanExtrasUpper,
            Package::ItalianCoreUpper,
            Package::PortugueseCoreUpper,
            Package::DanishNorwegianUpper,
            Package::SwedishUpper,
            Package::IcelandicUpper,
            Package::LithuanianUpper,
            Package::LatvianUpper,
            Package::CzechUpper,
            Package::SlovakUpper,
            Package::SloveneBCSUpper,
            Package::HungarianUpper,
            Package::PolishUpper,
            Package::RomanianUpper,
            Package::WelshUpper,
            Package::GaelicUpper,
            Package::MalteseUpper,
            Package::TurkishAzeriUpper,
            Package::GreenlandicUpper,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("ẞ"),
    },
    // Greek (monotonic) full
    Preset {
        name: "greek_monotonic_full",
        packages: &[
            Package::GreekCaps,
            Package::GreekLowerBase,
            Package::GreekMonotonicMarksUpper,
            Package::GreekMonotonicMarksLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    // Russian-only upper+lower
    Preset {
        name: "cyrillic_ru_full",
        packages: &[
            Package::CyrillicRuCaps, Package::CyrillicRuLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    // Pan-Slavic Cyrillic (kept as requested)
    Preset {
        name: "cyrillic_pan_slavic",
        packages: &[
            Package::CyrillicRuCaps, Package::CyrillicRuLower,
            Package::CyrillicUkCaps, Package::CyrillicUkLower,
            Package::CyrillicBeCaps, Package::CyrillicBeLower,
            Package::CyrillicSrMeCaps, Package::CyrillicSrMeLower,
            Package::CyrillicMkCaps, Package::CyrillicMkLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    // Hebrew letters (no niqqud)
    Preset {
        name: "hebrew",
        packages: &[Package::Hebrew],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // irrelevant; unicameral
        german_sharp_s_uppercase: None,
    },
    // Georgian Mkhedruli only
    Preset {
        name: "georgian_mkhedruli",
        packages: &[Package::GeorgianMkhedruli],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // unicameral
        german_sharp_s_uppercase: None,
    },
    // Cherokee (upper + small)
    Preset {
        name: "cherokee",
        packages: &[Package::CherokeeUpper, Package::CherokeeSmall],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    // Kana (kept as requested)
    Preset {
        name: "kana_full",
        packages: &[Package::Hiragana, Package::Katakana],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // no effect; unicameral
        german_sharp_s_uppercase: None,
    },

    /* =========================== Latin — generic tiers =========================== */

    // Latin signage, STRICT minimal: A–Z only (all diacritics dropped, no ligatures)
    Preset {
        name: "latin_signage_caps_strict_ascii",
        packages: &[Package::LatinAsciiUpper],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // Latin signage with common digraphs (helpful for wayfinding names)
    Preset {
        name: "latin_signage_caps_ascii_plus_common_digraphs",
        packages: &[
            Package::LatinAsciiUpper,
            Package::LatinDigraphsCommon,
        ],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // Latin pan-western caps (accents kept) — similar to your pan_western_caps, but explicit alias
    Preset {
        name: "latin_caps_pan_western_with_accents",
        packages: &[
            Package::LatinAsciiUpper,
            Package::SpanishCoreUpper,
            Package::FrenchAccentsUpper,
            Package::GermanExtrasUpper,
            Package::ItalianCoreUpper,
            Package::PortugueseCoreUpper,
            Package::DanishNorwegianUpper,
            Package::SwedishUpper,
            Package::IcelandicUpper,
            Package::LithuanianUpper,
            Package::LatvianUpper,
            Package::CzechUpper,
            Package::SlovakUpper,
            Package::SloveneBCSUpper,
            Package::HungarianUpper,
            Package::PolishUpper,
            Package::RomanianUpper,
            Package::WelshUpper,
            Package::GaelicUpper,
            Package::MalteseUpper,
            Package::TurkishAzeriUpper,
            Package::GreenlandicUpper,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("ẞ"),
    },
    // Latin upper+lower, pan-western, ligatures OFF
    Preset {
        name: "latin_upperlower_pan_western_core_no_ligatures",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::SpanishCoreUpper, Package::SpanishCoreLower,
            Package::FrenchAccentsUpper, Package::FrenchAccentsLower,
            Package::GermanExtrasUpper, Package::GermanExtrasLower,
            Package::ItalianCoreUpper, Package::ItalianCoreLower,
            Package::PortugueseCoreUpper, Package::PortugueseCoreLower,
            Package::DanishNorwegianUpper, Package::DanishNorwegianLower,
            Package::SwedishUpper, Package::SwedishLower,
            Package::IcelandicUpper, Package::IcelandicLower,
            Package::LithuanianUpper, Package::LithuanianLower,
            Package::LatvianUpper, Package::LatvianLower,
            Package::CzechUpper, Package::CzechLower,
            Package::SlovakUpper, Package::SlovakLower,
            Package::SloveneBCSUpper, Package::SloveneBCSLower,
            Package::HungarianUpper, Package::HungarianLower,
            Package::PolishUpper, Package::PolishLower,
            Package::RomanianUpper, Package::RomanianLower,
            Package::WelshUpper, Package::WelshLower,
            Package::GaelicUpper, Package::GaelicLower,
            Package::MalteseUpper, Package::MalteseLower,
            Package::TurkishAzeriUpper, Package::TurkishAzeriLower,
            Package::GreenlandicUpper, Package::GreenlandicLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("ẞ"),
    },
    // Latin upper+lower, pan-western, ligatures ON
    Preset {
        name: "latin_upperlower_pan_western_core_plus_ligatures",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::SpanishCoreUpper, Package::SpanishCoreLower,
            Package::FrenchAccentsUpper, Package::FrenchAccentsLower,
            Package::GermanExtrasUpper, Package::GermanExtrasLower,
            Package::ItalianCoreUpper, Package::ItalianCoreLower,
            Package::PortugueseCoreUpper, Package::PortugueseCoreLower,
            Package::DanishNorwegianUpper, Package::DanishNorwegianLower,
            Package::SwedishUpper, Package::SwedishLower,
            Package::IcelandicUpper, Package::IcelandicLower,
            Package::LithuanianUpper, Package::LithuanianLower,
            Package::LatvianUpper, Package::LatvianLower,
            Package::CzechUpper, Package::CzechLower,
            Package::SlovakUpper, Package::SlovakLower,
            Package::SloveneBCSUpper, Package::SloveneBCSLower,
            Package::HungarianUpper, Package::HungarianLower,
            Package::PolishUpper, Package::PolishLower,
            Package::RomanianUpper, Package::RomanianLower,
            Package::WelshUpper, Package::WelshLower,
            Package::GaelicUpper, Package::GaelicLower,
            Package::MalteseUpper, Package::MalteseLower,
            Package::TurkishAzeriUpper, Package::TurkishAzeriLower,
            Package::GreenlandicUpper, Package::GreenlandicLower,
            Package::LatinLigaturesUpper, Package::LatinLigaturesLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(true),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("ẞ"),
    },

    /* =============================== Spanish tiers =============================== */

    // CAPS with full Spanish diacritics: Ñ + ÁÉÍÓÚ + Ü
    Preset {
        name: "spanish_caps_with_diacritics",
        packages: &[
            Package::LatinAsciiUpper,
            Package::SpanishCoreUpper,
            Package::PortugueseCoreUpper, // Á É Í Ó Ú
            Package::GermanExtrasUpper,   // Ü
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // Full Spanish upper+lower with diacritics
    Preset {
        name: "spanish_upperlower_full_with_diacritics",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::SpanishCoreUpper, Package::SpanishCoreLower,
            Package::PortugueseCoreUpper, Package::PortugueseCoreLower,
            Package::GermanExtrasUpper, Package::GermanExtrasLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    /* =============================== French tiers =============================== */

    // French signage caps (accents dropped)
    Preset {
        name: "french_signage_caps_drop_accents",
        packages: &[Package::LatinAsciiUpper],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // French caps with accents, no ligatures
    Preset {
        name: "french_caps_with_accents_no_ligatures",
        packages: &[
            Package::LatinAsciiUpper,
            Package::FrenchAccentsUpper,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("ẞ"),
    },
    // French full upper+lower with accents and ligatures (Œ/Æ/Ĳ + fl-ligatures)
    Preset {
        name: "french_upperlower_full_with_accents_and_ligatures",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::FrenchAccentsUpper, Package::FrenchAccentsLower,
            Package::LatinLigaturesUpper, Package::LatinLigaturesLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(true),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("ẞ"),
    },

    /* =============================== German tiers =============================== */

    // German signage caps (Ä/Ö/Ü → AE/OE/UE, ß → SS)
    Preset {
        name: "german_signage_caps_transliterated",
        packages: &[Package::LatinAsciiUpper],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // German caps preserving umlauts and ẞ
    Preset {
        name: "german_caps_with_umlauts_and_eszett",
        packages: &[
            Package::LatinAsciiUpper, Package::GermanExtrasUpper
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("ẞ"),
    },
    // German full upper+lower preserving diacritics
    Preset {
        name: "german_upperlower_full_with_umlauts_and_eszett",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::GermanExtrasUpper, Package::GermanExtrasLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("ẞ"),
    },

    /* ========================== Italian / Portuguese tiers ========================== */

    Preset {
        name: "italian_signage_caps_drop_accents",
        packages: &[Package::LatinAsciiUpper],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "italian_caps_with_accents",
        packages: &[Package::LatinAsciiUpper, Package::ItalianCoreUpper],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "italian_upperlower_full_with_accents",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::ItalianCoreUpper, Package::ItalianCoreLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    Preset {
        name: "portuguese_signage_caps_drop_accents",
        packages: &[Package::LatinAsciiUpper],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "portuguese_caps_with_accents",
        packages: &[Package::LatinAsciiUpper, Package::PortugueseCoreUpper],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "portuguese_upperlower_full_with_accents",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::PortugueseCoreUpper, Package::PortugueseCoreLower,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    /* ========================= Nordic & Baltic language tiers ========================= */

    Preset {
        name: "danish_norwegian_caps_with_æøå",
        packages: &[Package::LatinAsciiUpper, Package::DanishNorwegianUpper],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "danish_norwegian_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::DanishNorwegianUpper, Package::DanishNorwegianLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    Preset {
        name: "swedish_caps_with_åäö",
        packages: &[Package::LatinAsciiUpper, Package::SwedishUpper],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "swedish_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::SwedishUpper, Package::SwedishLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    Preset {
        name: "icelandic_caps_core",
        packages: &[Package::LatinAsciiUpper, Package::IcelandicUpper],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "icelandic_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::IcelandicUpper, Package::IcelandicLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    Preset {
        name: "lithuanian_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::LithuanianUpper, Package::LithuanianLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "latvian_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::LatvianUpper, Package::LatvianLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    /* ====================== Central/Eastern European tiers ====================== */

    Preset {
        name: "czech_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::CzechUpper, Package::CzechLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "slovak_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::SlovakUpper, Package::SlovakLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "slovene_bcs_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::SloveneBCSUpper, Package::SloveneBCSLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "hungarian_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::HungarianUpper, Package::HungarianLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "polish_upperlower_full",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::PolishUpper, Package::PolishLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "romanian_upperlower_full_modern",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::RomanianUpper, Package::RomanianLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    /* ============================ Celtic / Maltese tiers ============================ */

    Preset {
        name: "welsh_upperlower_core",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::WelshUpper, Package::WelshLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "gaelic_upperlower_core",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::GaelicUpper, Package::GaelicLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "maltese_upperlower_core",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::MalteseUpper, Package::MalteseLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    /* ============================ Turkish / Greenlandic ============================ */

    Preset {
        name: "turkish_azeri_upperlower_core",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::TurkishAzeriUpper, Package::TurkishAzeriLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },
    Preset {
        name: "greenlandic_upperlower_core",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::GreenlandicUpper, Package::GreenlandicLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    /* ================================ Vietnamese / Pinyin ================================ */

    // Vietnamese signage caps (drop diacritics) — for legacy signage
    Preset {
        name: "vietnamese_signage_caps_drop_accents",
        packages: &[Package::LatinAsciiUpper],
        drop_diacritics: Some(true),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: Some("SS"),
    },
    // Vietnamese full (precomposed, both cases)
    Preset {
    name: "vietnamese_upperlower_full",
    packages: &[Package::VietnameseUpper, Package::VietnameseLower],
    drop_diacritics: Some(false),
    keep_ligatures: Some(false),
    case_mode: Some("UpperLower"),
    german_sharp_s_uppercase: Some("SS"),
},

    // Hanyu Pīnyīn tone vowels
    Preset {
        name: "pinyin_upperlower_tone_vowels",
        packages: &[
            Package::LatinAsciiUpper, Package::LatinAsciiLower,
            Package::LatinPinyinUpper, Package::LatinPinyinLower
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: Some("SS"),
    },

    /* ===================================== IPA / Africanist ===================================== */

    // Bare IPA letters (no modifiers)
    Preset {
        name: "ipa_letters_core_only",
        packages: &[Package::LatinIpaCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("LowerOnly"),
        german_sharp_s_uppercase: None,
    },
    // IPA letters + common modifiers/diacritics
    Preset {
        name: "ipa_core_plus_modifiers",
        packages: &[Package::LatinIpaCore, Package::LatinIpaModifiers],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("LowerOnly"),
        german_sharp_s_uppercase: None,
    },
    // Africanist add-ons (letters + clicks)
    Preset {
    name: "ipa_africanist_extended_with_clicks",
    packages: &[
        Package::LatinIpaCore,
        Package::LatinIpaModifiers,
        Package::LatinAfricanLower, // was LatinAfricanCore
Package::LatinAfricanUpper, 
        Package::LatinClicks,
    ],
    drop_diacritics: Some(false),
    keep_ligatures: Some(false),
    case_mode: Some("UpperLower"),
    german_sharp_s_uppercase: None,
},

    /* =================================== Celtic/West African (Latin extensions) =================================== */

    Preset {
    name: "yoruba_upperlower_core",
    packages: &[Package::LatinAsciiUpper, Package::LatinAsciiLower,
                Package::YorubaUpper, Package::YorubaLower],
    drop_diacritics: Some(false),
    keep_ligatures: Some(false),
    case_mode: Some("UpperLower"),
    german_sharp_s_uppercase: Some("SS"),
},
    Preset {
    name: "yoruba_upperlower_with_tone_sequences",
    packages: &[Package::LatinAsciiUpper, Package::LatinAsciiLower,
                Package::YorubaUpper, Package::YorubaLower,
                Package::YorubaTonesUpper, Package::YorubaTonesLower],
    drop_diacritics: Some(false),
    keep_ligatures: Some(false),
    case_mode: Some("UpperLower"),
    german_sharp_s_uppercase: Some("SS"),
},
    Preset {
    name: "kikuyu_upperlower_core",
    packages: &[Package::LatinAsciiUpper, Package::LatinAsciiLower,
                Package::KikuyuUpper, Package::KikuyuLower],
    drop_diacritics: Some(false),
    keep_ligatures: Some(false),
    case_mode: Some("UpperLower"),
    german_sharp_s_uppercase: Some("SS"),
},
    Preset {
    name: "salish_upperlower_core",
    packages: &[Package::LatinAsciiUpper, Package::LatinAsciiLower,
                Package::LatinSalishUpper, Package::LatinSalishLower],
    drop_diacritics: Some(false),
    keep_ligatures: Some(false),
    case_mode: Some("UpperLower"),
    german_sharp_s_uppercase: Some("SS"),
},
    Preset {
    name: "esperanto_upperlower_core",
    packages: &[Package::LatinAsciiUpper, Package::LatinAsciiLower,
                Package::EsperantoUpper, Package::EsperantoLower],
    drop_diacritics: Some(false),
    keep_ligatures: Some(false),
    case_mode: Some("UpperLower"),
    german_sharp_s_uppercase: Some("SS"),
},

    /* =================================== Greek tiers =================================== */

    // Caps only, no accents (monotonic marks dropped)
    Preset {
        name: "greek_caps_no_marks",
        packages: &[Package::GreekCaps],
        drop_diacritics: Some(true),  // drop tonos/dialytika in signage context
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    // Caps with monotonic marks
    Preset {
        name: "greek_caps_with_monotonic_marks",
        packages: &[Package::GreekCaps, Package::GreekMonotonicMarksUpper],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },

    /* ================================= Cyrillic per-language ================================= */

    Preset {
        name: "cyrillic_russian_caps_signage",
        packages: &[Package::CyrillicRuCaps],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "cyrillic_russian_upperlower_full",
        packages: &[Package::CyrillicRuCaps, Package::CyrillicRuLower],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },

    Preset {
        name: "cyrillic_ukrainian_upperlower_full",
        packages: &[Package::CyrillicUkCaps, Package::CyrillicUkLower],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "cyrillic_belarusian_upperlower_full",
        packages: &[Package::CyrillicBeCaps, Package::CyrillicBeLower],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "cyrillic_serbian_montenegrin_upperlower_full",
        packages: &[Package::CyrillicSrMeCaps, Package::CyrillicSrMeLower],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "cyrillic_macedonian_upperlower_full",
        packages: &[Package::CyrillicMkCaps, Package::CyrillicMkLower],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },
    // Bulgarian: uses RU core in current packs; we can split later if we add Ѝ/ѝ explicitly
    Preset {
        name: "cyrillic_bulgarian_upperlower_full",
        packages: &[Package::CyrillicRuCaps, Package::CyrillicRuLower],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("UpperLower"),
        german_sharp_s_uppercase: None,
    },

    /* ============================== Semitic / Perso-Arabic ============================== */

    Preset {
        name: "arabic_core_basic",
        packages: &[Package::ArabicCore],
        drop_diacritics: Some(false),  // harakat not in core set
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),   // unicameral
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "persian_urdu_core",
        packages: &[Package::ArabicCore, Package::PersianUrduExtras],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),   // unicameral
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "hebrew_basic_unicameral",
        packages: &[Package::Hebrew],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),   // unicameral
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "syriac_core_basic",
        packages: &[Package::SyriacCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "thaana_core_basic",
        packages: &[Package::ThaanaCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },

    /* =================================== Indic cores =================================== */

    Preset {
        name: "devanagari_core_basic",
        packages: &[Package::DevanagariCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // bicameral concept doesn't apply
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "bengali_core_basic",
        packages: &[Package::BengaliCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "gurmukhi_core_basic",
        packages: &[Package::GurmukhiCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "gujarati_core_basic",
        packages: &[Package::GujaratiCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "odia_core_basic",
        packages: &[Package::OdiaCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "tamil_core_basic",
        packages: &[Package::TamilCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "telugu_core_basic",
        packages: &[Package::TeluguCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "kannada_core_basic",
        packages: &[Package::KannadaCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "malayalam_core_basic",
        packages: &[Package::MalayalamCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "sinhala_core_basic",
        packages: &[Package::SinhalaCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },

    /* ============================ SE Asian & others ============================ */

    Preset {
        name: "thai_core_basic",
        packages: &[Package::ThaiCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "lao_core_basic",
        packages: &[Package::LaoCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "tibetan_core_basic",
        packages: &[Package::TibetanCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "myanmar_core_basic",
        packages: &[Package::MyanmarCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "khmer_core_basic",
        packages: &[Package::KhmerCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },

    /* ============================== East Asian & CAS ============================== */

    Preset {
        name: "hiragana_only",
        packages: &[Package::Hiragana],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // no effect
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "katakana_only",
        packages: &[Package::Katakana],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // no effect
        german_sharp_s_uppercase: None,
    },

    Preset {
        name: "hangul_jamo_all",
        packages: &[Package::HangulJamoAll],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // no effect
        german_sharp_s_uppercase: None,
    },

    Preset {
        name: "ethiopic_core_basic",
        packages: &[Package::EthiopicCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // no effect
        german_sharp_s_uppercase: None,
    },

    Preset {
        name: "inuktitut_core_basic",
        packages: &[Package::InuktitutCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // no effect
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "cree_core_basic",
        packages: &[Package::CreeCore],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"), // no effect
        german_sharp_s_uppercase: None,
    },
        // ======= Numbers / Digits =======
    Preset {
        name: "numbers_ascii_only_signage",
        packages: &[Package::DigitsAscii],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "numbers_multiscript_common_global",
        packages: &[
            Package::DigitsAscii,
            Package::DigitsArabicIndic,
            Package::DigitsExtendedArabicIndic,
            Package::DigitsDevanagari,
            Package::DigitsThai,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "numbers_multiscript_south_asia",
        packages: &[
            Package::DigitsAscii,
            Package::DigitsDevanagari,
            Package::DigitsBengali,
            Package::DigitsGurmukhi,
            Package::DigitsGujarati,
            Package::DigitsOdia,
            Package::DigitsTamil,
            Package::DigitsTelugu,
            Package::DigitsKannada,
            Package::DigitsMalayalam,
            Package::DigitsSinhala,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },

    // ======= Math =======
    Preset {
        name: "math_signage_minimal",
        packages: &[
            Package::MathBasicArithmetic,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "math_algebra_full",
        packages: &[
            Package::MathBasicArithmetic,
            Package::MathAlgebraCore,
            Package::MathSetRelations,
            Package::MathTypographicVariants, // add typographic minus/dot ops
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(true),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "math_advanced",
        packages: &[
            Package::MathBasicArithmetic,
            Package::MathAlgebraCore,
            Package::MathSetRelations,
            Package::MathCalculusCore,
            Package::MathLogicCore,
            Package::MathArrowsBasic,
            Package::MathTypographicVariants,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(true),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "math_latex_extended",
        packages: &[
            Package::MathBasicArithmetic,
            Package::MathAlgebraCore,
            Package::MathSetRelations,
            Package::MathCalculusCore,
            Package::MathLogicCore,
            Package::MathArrowsBasic,
            Package::MathArrowsExtended,
            Package::MathDelimitersExtended,
            Package::MathOperatorsExtended,
            Package::MathDoubleStruckSets,
            Package::DigitsSuperscriptCore,
            Package::DigitsSubscriptCore,
            Package::MathTypographicVariants,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(true),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "math_context_arabic",
        packages: &[
            Package::MathBasicArithmetic,
            Package::MathArabicNumericSymbols,
            Package::DigitsArabicIndic,
            Package::DigitsExtendedArabicIndic,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(true),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "math_fullwidth_east_asian_basic",
        packages: &[
            Package::MathFullwidthBasicEastAsian,
            Package::DigitsAscii, // many contexts still use ASCII digits with fullwidth ops
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(true),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },

    // ======= Punctuation =======
    Preset {
        name: "punct_basic_latin_word_sentence",
        packages: &[
            Package::PunctuationWordBasicLatin,
            Package::PunctuationSentenceBasicLatin,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "punct_language_spanish",
        packages: &[
            Package::PunctuationWordBasicLatin,
            Package::PunctuationSentenceBasicLatin,
            Package::PunctuationSpanishExtras,
            Package::PunctuationDashesBasic,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "punct_language_arabic",
        packages: &[
            Package::PunctuationArabicCore,
            Package::PunctuationDashesBasic,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "punct_language_devanagari",
        packages: &[
            Package::PunctuationDevanagariCore,
            Package::PunctuationDashesBasic,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "punct_language_japanese",
        packages: &[
            Package::PunctuationJapaneseCore,
            Package::PunctuationDashesBasic,
        ],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },

    // ======= Dashes =======
    Preset {
        name: "punct_dashes_minimal_signage",
        packages: &[Package::PunctuationDashesBasic],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
        name: "punct_dashes_full_editorial",
        packages: &[Package::PunctuationDashesAll],
        drop_diacritics: Some(false),
        keep_ligatures: Some(false),
        case_mode: Some("CapsOnly"),
        german_sharp_s_uppercase: None,
    },
    Preset {
		name: "hawaiian_upperlower_core",
		packages: &[
			Package::LatinAsciiUpper, Package::LatinAsciiLower, // base A–Z a–z
			Package::HawaiianCoreUpper, Package::HawaiianCoreLower,
			Package::HawaiianOkina,
		],
		drop_diacritics: Some(false),  // keep macrons
		keep_ligatures: Some(false),
		case_mode: Some("UpperLower"),
		german_sharp_s_uppercase: Some("SS"),
	},

	Preset {
		name: "punct_editorial_basic",
		packages: &[
			Package::PunctuationSentenceBasicLatin,
			Package::PunctuationDashesBasic,
			Package::PunctuationSectionsBasic,          // ← adds § and ¶
			Package::PunctuationLetterlikeSymbols,      // ← has № etc.
		],
		drop_diacritics: Some(false),
		keep_ligatures: Some(false),
		case_mode: Some("CapsOnly"),
		german_sharp_s_uppercase: None,
	},

];


/* --------------------------- (rest unchanged) --------------------------- */
/* Everything below here is your builder/bridge code exactly as in your
   current glyph_config.rs expectations: no symbol name changes needed
   except the lowercased set names we already aligned above. */



// (No changes here to your builder/bridges — they live in glyph_config.rs)
