//! Auto-generated German phonemizer module.
//! Distilled via TBenni Neuro-Symbolic CART Decision Tree Pipeline.

/// Static, sorted exception lexicon for German (zero-allocation O(log N) lookup).
pub static DE_LEXICON: &[(&str, &str)] = &[
    ("aa", "aː"),
    ("aach", "ax"),
    ("aachen", "axn̩"),
    ("aachenaufenthalt", "axn̩aʊfn̩thalt"),
    ("aachenbesuch", "axn̩bəzuːx"),
    ("aachener", "axeːnɐ"),
    ("aachenreise", "axn̩ʁaɪzə"),
    ("aachtal", "axtaːl"),
    ("aal", "aːl"),
    ("aalartig", "aːlaʁtɪk"),
    ("aalbestand", "aːlbəʃtant"),
    ("aalbestände", "aːlbəʃtɛndə"),
    ("aalen", "aːlən"),
    ("aalener", "aːlənɐ"),
    ("aalfang", "aːlfaŋ"),
    ("aalfischer", "aːlfɪʃɐ"),
    ("aalfischerei", "aːlfɪʃəʁaɪ"),
    ("aalförmig", "aːlfœʁmɪk"),
    ("aalglatt", "aːlɡlat"),
    ("aalkasten", "aːlkastn̩"),
    ("aalkiste", "aːlkɪstə"),
    ("aalkorb", "aːlkɔʁp"),
    ("aalkästchen", "aːlkɛstçən"),
    ("aalkästen", "aːlkɛstn̩"),
    ("aalkörbe", "aːlkœʁbə"),
    ("aalmutter", "aːlmʊtɐ"),
    ("aalreuse", "aːlʁɔɪzə"),
    ("aalräucherei", "aːlʁɔɪçəʁaɪ"),
    ("aalspeer", "aːlspəʁ"),
    ("aalstechen", "aːlʃtɛçn̩"),
    ("aalstrich", "aːlʃtʁɪç"),
    ("aalsuppe", "aːlzʊpə"),
    ("aalsüppchen", "aːlʃʏpçn̩"),
    ("aaltierchen", "altiːɐsn̩"),
    ("aalähnlich", "aːlʔɛːnlɪç"),
    ("aar", "aːɐ"),
    ("aarau", "aʁaʊ"),
    ("aarauer", "aʁaʊɐ"),
    ("aarburg", "aːɐbʊʁk"),
    ("aare", "aːʁə"),
    ("aareabschnitt", "aʁeːapʃnɪt"),
    ("aareabwärts", "aːʁəapvɛʁts"),
    ("aaregletscher", "aʁeːɡlɛt͡ʃɐ"),
    ("aareschlaufe", "aʁɛʃlaʊfə"),
    ("aaretal", "aʁeːtal"),
    ("aargau", "aːɐɡaʊ"),
    ("aargauer", "aːɐɡaʊɐ"),
    ("aargauisch", "aːɐɡaʊɪʃ"),
    ("aaron", "aːʁɔn"),
    ("aaronitisch", "aʁoniːtɪʃ"),
    ("aartalbahn", "aʁtaːlbaːn"),
    ("aas", "aːs"),
    ("aasartig", "aːzaʁtɪk"),
    ("aasee", "aːzə"),
    ("aaseestadt", "aːzəʃtaːt"),
    ("aaseestdter", "aːzəʃtttɐ"),
    ("aaseeterrasse", "azeːtɐʁasə"),
    ("aasfliege", "aːsfliːɡə"),
    ("aasfresser", "asfʁɛsɐ"),
    ("aasgeier", "aːsɡaɪɐ"),
    ("aasgeruch", "asɡəʁuːx"),
    ("aasgestank", "aːsɡəʃtaŋk"),
    ("aasinsekt", "aːzɪnzɛkt"),
    ("aasjger", "asjasjɐ"),
    ("aaskfer", "askfɐ"),
    ("aaskrhe", "askʁə"),
    ("aaskrhengelege", "askʁɛŋleːɡəleːɡə"),
    ("aaskrhenmnnchen", "askʁːənmŋxn̩"),
    ("aasvgel", "asfɡeːl"),
    ("aasvogel", "aːsfoːɡl̩"),
    ("ab", "ap"),
    ("abaelard", "abaɪlaːɐt"),
    ("abaelardus", "abaːɐduːs"),
    ("abakus", "abakʊs"),
    ("abakusrechnen", "apakʊsʁɛçnən"),
    ("abarbeiten", "apaʁbaɪtn̩"),
    ("abarbeitungs", "apaʁbaɪtʊŋs"),
    ("abart", "abaːɐt"),
    ("abartig", "aːbaːɐtɪk"),
    ("abartigkeit", "aːbaʁtɪkkaɪt"),
    ("abaxial", "abaksiaːl"),
    ("abbach", "apbax"),
    ("abbacher", "apbaxɐ"),
    ("abbaggern", "apbaɡɐn"),
    ("abband", "apbant"),
    ("abbaside", "apbaziːdə"),
    ("abbasidenherrschaft", "apbaziːdn̩haft"),
    ("abbasidenkalif", "abaːzidɛŋkaliː"),
    ("abbasidenkalifat", "abaziːdn̩kaliːfat"),
    ("abbasidenzeit", "aːbaziːdn̩t͡saɪt"),
    ("abbasidisch", "abaziːdɪʃ"),
    ("abbat", "apbaːt"),
    ("abbau", "apbaʊ"),
    ("abbaubarkeit", "apbaʊbaːɐkaɪt"),
    ("abbauen", "apbaʊən"),
    ("abbaufhig", "apbaʊfhɪk"),
    ("abbauratenkorrigiert", "apbaʊʁaːtn̩kɔʁiɡiːɐt"),
    ("abbauwrdig", "apbaʊvʁdɪk"),
    ("abbeien", "apbaɪən"),
    ("abbeizen", "apbaɪt͡sn̩"),
    ("abbeizmittel", "apbaɪt͡smɪtl̩"),
    ("abbekam", "apbəkaːm"),
    ("abbekm", "apbɛkm̩"),
    ("abbekommen", "apbəkɔmən"),
    ("abben", "abn̩"),
    ("abbenrode", "apbn̩ʁoːdə"),
    ("abberief", "apbəʁiːf"),
    ("abberufen", "apbəʁuːfn̩"),
    ("abbestellen", "apbəʃtɛlən"),
    ("abbetteln", "apbɛtl̩n"),
    ("abbezahlen", "apbət͡saːlən"),
    ("abbge", "apɡə"),
    ("abbgeln", "apɡɛln"),
    ("abbiegemglichkeit", "apbiːɡəmkaɪt"),
    ("abbiegen", "apbiːɡn̩"),
    ("abbieger", "apbiːɡɐ"),
    ("abbiegerspur", "apbiːɡɐspuːɐ"),
    ("abbiegespur", "apbiːɡəspuːɐ"),
    ("abbiegestreifen", "apbiːɡəstʁaɪfn̩"),
    ("abbiegeunfall", "apbiːɡɔɪnfal"),
    ("abbiegevorgang", "apbiːɡəfoːɐɡaŋ"),
    ("abbild", "apbɪlt"),
    ("abbildbeziehung", "apbɪltbət͡siːʊŋ"),
    ("abbilddatei", "abɪltdataɪ"),
    ("abbilden", "apbɪldn̩"),
    ("abbildgenau", "apbɪltɡənaʊ"),
    ("abbildtheoretisch", "abɪlteoʁeːtɪʃ"),
    ("abbildtheorie", "apbɪltteoʁiː"),
    ("abbildungs", "apbɪldʊŋs"),
    ("abbildverhltnis", "apbɪltfɛɐhltnɪs"),
    ("abbinde", "apbɪndə"),
    ("abbinden", "apbɪndn̩"),
    ("abbiss", "apbɪs"),
    ("abbitte", "apbɪtə"),
    ("abbitten", "apbɪtn̩"),
    ("abblas", "ablas"),
    ("abblasen", "apblaːzn̩"),
    ("abblassen", "apblasn̩"),
    ("abblend", "apblɛnt"),
    ("abblenden", "apblɛndn̩"),
    ("abblick", "apblɪk"),
    ("abblieb", "apbliːp"),
    ("abblies", "apbliːs"),
    ("abblitzen", "aplɪt͡sn̩"),
    ("abblocken", "apblɔkn̩"),
    ("abblst", "abl̩st"),
    ("abblttern", "apbltɐn"),
    ("abbog", "apboːk"),
    ("abbrach", "apbʁax"),
    ("abbrachte", "apbʁaxtə"),
    ("abbrand", "apbʁant"),
    ("abbrannte", "apbʁantə"),
    ("abbrausen", "apbʁaʊzn̩"),
    ("abbrche", "apbʁçə"),
    ("abbrchte", "apbʁçtə"),
    ("abbrckeln", "apbʁkl̩n"),
    ("abbrechen", "apbʁɛçn̩"),
    ("abbrecher", "apbʁɛçɐ"),
    ("abbrecherquote", "apbʁɛçɐkvoːtə"),
    ("abbrems", "apbʁɛms"),
    ("abbremsen", "apbʁɛmzn̩"),
    ("abbrenn", "apbʁɛn"),
    ("abbrennen", "apbʁɛnən"),
    ("abbreviatur", "abʁeviatuːɐ"),
    ("abbreviieren", "abʁeviiːʁən"),
    ("abbricht", "apbʁɪçt"),
    ("abbringen", "apbʁɪŋən"),
    ("abbrnde", "apbʁndə"),
    ("abbruch", "apbʁʊx"),
    ("abbruchreif", "apbʁʊxʁaɪf"),
    ("abbruchschaufel", "apbʁʊxʃaʊfl̩"),
    ("abbrummen", "apbʁʊmən"),
    ("abbte", "aptə"),
    ("abbuchen", "apbuːxn̩"),
    ("abbuchungs", "apbuːxʊŋs"),
    ("abbummeln", "apbʊml̩n"),
    ("abbände", "apbɛndə"),
    ("abbürsten", "apbʏʁstn̩"),
    ("abc", "aːp"),
    ("abchase", "aptaːzə"),
    ("abchasien", "aboziːən"),
    ("abchasienkrieg", "apxaːziənkʁiːk"),
    ("abchasisch", "abiʃ"),
    ("abchecken", "apt͡ʃɛkn̩"),
    ("abclich", "apt͡siːx"),
    ("abdachen", "apdaxn̩"),
    ("abdampf", "apdamp͡f"),
    ("abdampfen", "apdamp͡fn̩"),
    ("abdanken", "apdaŋkn̩"),
    ("abdankungsangebot", "apdaŋkʊŋsanɡəboːt"),
    ("abdankungsbrief", "apdaŋkʊŋsbʁiːf"),
    ("abdankungserklrung", "apdaŋsɛɐklʁʊŋ"),
    ("abdankungsurkunde", "apdaŋkʊŋsuːɐkʊŋdə"),
    ("abdeck", "apdɛk"),
    ("abdecken", "apdɛkn̩"),
    ("abdecker", "apdɛkɐ"),
    ("abdeckerei", "apdɛkəʁaɪ"),
    ("abdeckungsrate", "apdɛkʊŋsʁaːtə"),
    ("abdeichen", "apdaɪçn̩"),
    ("abdekantieren", "apdekantiːʁən"),
    ("abdestillieren", "apdɛstɪljiːʁən"),
    ("abdicht", "apdɪçt"),
    ("abdichten", "apdɪçtn̩"),
    ("abdichtungs", "apdɪçtʊŋs"),
    ("abdienen", "apdiːnən"),
    ("abdikation", "apdikat͡sioːn"),
    ("abdingbar", "apdɪŋbaːɐ"),
    ("abdingen", "apdɪŋən"),
    ("abdmmen", "aptmən"),
    ("abdocken", "apdɔkn̩"),
    ("abdomen", "apdoːmən"),
    ("abdomenende", "apdomeːnəndə"),
    ("abdomina", "apdomiːna"),
    ("abdominal", "apdominaːl"),
    ("abdominalgraviditt", "apdominaːlɡʁavidiːt"),
    ("abdrahten", "apdʁaːtn̩"),
    ("abdrcke", "apdʁɛkə"),
    ("abdrcken", "apdʁɛkn̩"),
    ("abdreh", "apdʁeː"),
    ("abdrehen", "apdʁeːən"),
    ("abdrift", "apdʁɪft"),
    ("abdriften", "apdʁɪftn̩"),
    ("abdrngen", "apdʁnɡn̩"),
    ("abdrosseln", "apdʁɔsl̩n"),
    ("abdruck", "apdʁʊk"),
    ("abdrucken", "apdʁʊkn̩"),
    ("abdsen", "apdsn̩"),
    ("abduktion", "abdʊkt͡sioːn"),
    ("abduktions", "abdʊkt͡sioːns"),
    ("abduktiv", "apdʊktiːf"),
    ("abduktor", "apdʊktoːɐ"),
    ("abduktoren", "apdʊktoːʁən"),
    ("abdullah", "apdʊllaː"),
    ("abdunkeln", "apdʊŋkl̩n"),
    ("abdunkelungs", "apdʊŋkəlʊŋs"),
    ("abduschen", "apduːʃn̩"),
    ("abduzens", "apdut͡seːəns"),
    ("abebben", "aːbn̩"),
    ("abece", "aːbətə"),
    ("abecebcher", "aːbət͡seːbçɐ"),
    ("abecebuch", "aːbət͡seːbuç"),
    ("abecelich", "aːbət͡seːlɪç"),
    ("abeceschtze", "aːbəʃt͡sə"),
    ("abel", "aːbl̩"),
    ("abelitz", "aːbəlɪt͡s"),
    ("abelsch", "aːbl̩ʃ"),
    ("abend", "aːbn̩t"),
    ("abendfllend", "aːbn̩tflɛnt"),
    ("abendhand", "aːbn̩thant"),
    ("abendkhle", "aːbn̩tkleː"),
    ("abendlich", "aːbn̩tlɪç"),
    ("abendlndisch", "aːbn̩tlɪʃ"),
    ("abendluten", "aːbn̩tluːtn̩"),
    ("abendmahl", "aːbn̩tmaːl"),
    ("abendmahls", "aːbn̩tmaːls"),
    ("abendmal", "aːbn̩tmaːl"),
    ("abendrot", "aːbn̩tʁoːt"),
    ("abendroth", "aːbn̩tʁoːt"),
    ("abendrte", "aːbn̩tʁ̩tə"),
    ("abends", "aːbn̩t͡s"),
    ("abendwand", "aːbn̩tvant"),
    ("abenteuer", "aːbn̩tɔɪɐ"),
    ("abenteuerhungrig", "aːbn̩tɔɪɐhʊŋʁɪk"),
    ("abenteuerlich", "aːbn̩tɔɪɐlɪç"),
    ("abenteuerlichkeit", "aːbn̩tɔɪɐlɪçkaɪt"),
    ("abenteuerlustig", "aːbn̩tɔɪɐlʊstɪk"),
    ("abenteuerreich", "aːbn̩tɔɪɐʁaɪç"),
    ("abenteurer", "aːbn̩tɔɪɐ"),
    ("abenteurertum", "aːbn̩tɔɪɐtuːm"),
    ("aber", "aːbɐ"),
    ("aberglaube", "aːbɐɡlaʊbə"),
    ("aberglauben", "aːbɐɡlaʊbn̩"),
    ("aberglubisch", "aːbɐɡluːbɪʃ"),
    ("aberhundert", "abɐhʊndɐt"),
    ("aberkannt", "aːbɐkant"),
    ("aberkennen", "aːbɐkɛnən"),
    ("abermalig", "aːbɐmaːlɪk"),
    ("abermals", "aːbɐmaːls"),
    ("abermilliarden", "abɛʁmijiaːɐdn̩"),
    ("abermillionen", "aːbɐmɪlioːnən"),
    ("abernten", "aːbɐntn̩"),
    ("aberrant", "aːbɐʁant"),
    ("aberration", "aːbɐʁat͡sioːn"),
    ("aberrationsfehler", "abɛʁat͡sioːnsfeːlɐ"),
    ("aberrationskorrektur", "abɛʁat͡sioːnskɔʁɛktuːɐ"),
    ("aberrationswinkel", "abɛʁat͡sioːnsvɪŋkl̩"),
    ("abertausend", "aːbɐtaʊzn̩t"),
    ("aberwitz", "aːbɐvɪt͡s"),
    ("aberwitzig", "aːbɐvɪt͡sɪk"),
    ("aberziehen", "aːbɐt͡siːən"),
    ("aberzogen", "aːbɐt͡soːɡn̩"),
    ("abessen", "aːbəsn̩"),
    ("abessinien", "aːbəsiːniən"),
    ("abessinienkrieg", "abɛsiːniənkʁiːk"),
    ("abessinier", "abɛziːniɐ"),
    ("abessinierkatze", "aːbəsinɐkat͡sə"),
    ("abessinisch", "abɛsiːnɪʃ"),
    ("abeter", "aːbətɐ"),
    ("abfackeln", "apfakl̩n"),
    ("abfahrbereit", "apfaːɐbəʁaɪt"),
];

/// Lookup German word in static exception table via binary search.
#[inline(always)]
pub fn lookup_de_override(word: &str) -> Option<&'static str> {
    DE_LEXICON
        .binary_search_by_key(&word, |(w, _)| w)
        .ok()
        .map(|idx| DE_LEXICON[idx].1)
}

/// Phonemize a single German word using compiled deterministic phonotactics.
pub fn phonemize_de_word(word: &str) -> String {
    let lower = word.to_lowercase();
    if let Some(override_ipa) = lookup_de_override(&lower) {
        return override_ipa.to_string();
    }

    let chars: Vec<char> = lower.chars().collect();
    let mut result = String::with_capacity(chars.len() * 2);
    let mut i = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u', 'ä', 'ö', 'ü'];
    // Standard German prefixes at word start
    if lower.starts_with("ge") && lower.len() > 3 && !vowels.contains(&chars[2]) {
        result.push_str("ɡə");
        i = 2;
    } else if lower.starts_with("be") && lower.len() > 3 && !vowels.contains(&chars[2]) {
        result.push_str("bə");
        i = 2;
    } else if lower.starts_with("ver") && lower.len() > 4 {
        result.push_str("fɛɐ");
        i = 3;
    } else if lower.starts_with("zer") && lower.len() > 4 {
        result.push_str("tsɛɐ");
        i = 3;
    } else if lower.starts_with("ent") && lower.len() > 4 {
        result.push_str("ɛnt");
        i = 3;
    }

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let next2 = chars.get(i + 2).copied();
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };

        // Trigraph sch -> [ʃ]
        if c == 's' && next == Some('c') && next2 == Some('h') {
            result.push('ʃ');
            i += 3;
            continue;
        }
        // Word-initial sp / st -> [ʃp] / [ʃt]
        if c == 's' && (i == 0 || prev == Some('-')) && (next == Some('p') || next == Some('t')) {
            result.push('ʃ');
            i += 1;
            continue;
        }
        // Digraph ch -> [x] after back vowels (a, o, u, au), else [ç]
        if c == 'c' && next == Some('h') {
            let is_back = matches!(prev, Some('a' | 'o' | 'u'))
                || (i >= 2 && chars[i - 2] == 'a' && prev == Some('u'));
            if is_back {
                result.push('x');
            } else {
                result.push('ç');
            }
            i += 2;
            continue;
        }
        // ck -> [k]
        if c == 'c' && next == Some('k') {
            result.push('k');
            i += 2;
            continue;
        }
        // tz -> [ts]
        if c == 't' && next == Some('z') {
            result.push_str("ts");
            i += 2;
            continue;
        }
        // qu -> [kv]
        if c == 'q' && next == Some('u') {
            result.push_str("kv");
            i += 2;
            continue;
        }
        // ph -> [f]
        if c == 'p' && next == Some('h') {
            result.push('f');
            i += 2;
            continue;
        }
        // th -> [t]
        if c == 't' && next == Some('h') {
            result.push('t');
            i += 2;
            continue;
        }
        // ng -> [ŋ]
        if c == 'n' && next == Some('g') {
            result.push('ŋ');
            i += 2;
            continue;
        }
        // nk -> [ŋk]
        if c == 'n' && next == Some('k') {
            result.push_str("ŋk");
            i += 2;
            continue;
        }
        // Diphthongs
        // ei / ai -> [aɪ]
        if (c == 'e' || c == 'a') && next == Some('i') {
            result.push_str("aɪ");
            i += 2;
            continue;
        }
        // eu / äu -> [ɔʏ]
        if (c == 'e' || c == 'ä') && next == Some('u') {
            result.push_str("ɔʏ");
            i += 2;
            continue;
        }
        // au -> [aʊ]
        if c == 'a' && next == Some('u') {
            result.push_str("aʊ");
            i += 2;
            continue;
        }
        // ie -> [iː]
        if c == 'i' && next == Some('e') {
            result.push_str("iː");
            i += 2;
            continue;
        }
        // Double vowels
        if c == 'a' && next == Some('a') {
            result.push_str("aː");
            i += 2;
            continue;
        }
        if c == 'e' && next == Some('e') {
            result.push_str("eː");
            i += 2;
            continue;
        }
        if c == 'o' && next == Some('o') {
            result.push_str("oː");
            i += 2;
            continue;
        }
        // Dehnungs-h: vowel + h -> long vowel
        let is_vowel = matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü');
        if is_vowel && next == Some('h') {
            match c {
                'ä' => result.push_str("ɛː"),
                'ö' => result.push_str("øː"),
                'ü' => result.push_str("yː"),
                _ => {
                    result.push(c);
                    result.push('ː');
                }
            }
            i += 2;
            continue;
        }
        // Word-final -ig -> [ɪç], or before endings -> [ɪɡ]
        if c == 'i' && next == Some('g') {
            if next2.is_none() {
                result.push_str("ɪç");
                i += 2;
                continue;
            } else if next2 == Some('e') {
                result.push_str("ɪɡ");
                i += 2;
                continue;
            }
        }
        // Unstressed endings with e: -er, -ern, -ers, and internal -er- before consonants -> ɐ
        if c == 'e' && next == Some('r') {
            let next3 = if i + 3 < chars.len() {
                Some(chars[i + 3])
            } else {
                None
            };
            if next2.is_none() {
                result.push('ɐ');
                i += 2;
                continue;
            } else if next2 == Some('n') && (next3.is_none() || !vowels.contains(&next3.unwrap())) {
                result.push_str("ɐn");
                i += 3;
                continue;
            } else if next2 == Some('s') && (next3.is_none() || !vowels.contains(&next3.unwrap())) {
                result.push_str("ɐs");
                i += 3;
                continue;
            } else if next2.is_some_and(|n| !vowels.contains(&n)) {
                result.push('ɐ');
                i += 2;
                continue;
            }
        }
        // Short vowels before double consonants / geminates
        let is_before_double = next.is_some_and(|n| {
            (next2 == Some(n) && !vowels.contains(&n))
                || (n == 'c' && next2 == Some('k'))
                || (n == 't' && next2 == Some('z'))
                || (n == 's' && next2 == Some('c') && chars.get(i + 3).copied() == Some('h'))
        });
        if is_before_double && vowels.contains(&c) {
            match c {
                'ä' | 'e' => {
                    result.push('ɛ');
                    i += 1;
                    continue;
                }
                'i' => {
                    result.push('ɪ');
                    i += 1;
                    continue;
                }
                'o' => {
                    result.push('ɔ');
                    i += 1;
                    continue;
                }
                'u' => {
                    result.push('ʊ');
                    i += 1;
                    continue;
                }
                _ => {}
            }
        }
        // Unstressed e -> schwa [ə] in endings (-e, -en, -es, -em, -el, -et)
        if c == 'e' && i > 0 {
            let is_schwa_end = next.is_none()
                || (next == Some('n') && (next2.is_none() || !vowels.contains(&next2.unwrap())))
                || (next == Some('s') && next2.is_none())
                || (next == Some('m') && next2.is_none())
                || (next == Some('l') && next2.is_none())
                || (next == Some('t') && next2.is_none() && i >= 3);
            if is_schwa_end {
                result.push('ə');
                i += 1;
                continue;
            }
        }
        // Vocalic r: r after vowel at end of word or before consonant -> [ɐ]
        if c == 'r'
            && matches!(prev, Some('a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü'))
            && (next.is_none()
                || !matches!(next, Some('a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü')))
        {
            result.push('ɐ');
            i += 1;
            continue;
        }
        // Degemination of double consonants (e.g. tt -> t, nn -> n)
        if next == Some(c) && !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü') {
            match c {
                's' => result.push('s'),
                'b' => result.push(if next2.is_none() { 'p' } else { 'b' }),
                'd' => result.push(if next2.is_none() { 't' } else { 'd' }),
                'g' => result.push(if next2.is_none() { 'k' } else { 'g' }),
                _ => result.push(c),
            }
            i += 2;
            continue;
        }
        // Final devoicing (Auslautverhärtung)
        let is_end_or_voiceless =
            next.is_none() || matches!(next, Some('s' | 't' | 'k' | 'p' | 'f' | 'h'));
        if is_end_or_voiceless {
            match c {
                'b' => {
                    result.push('p');
                    i += 1;
                    continue;
                }
                'd' => {
                    result.push('t');
                    i += 1;
                    continue;
                }
                'g' => {
                    result.push('k');
                    i += 1;
                    continue;
                }
                _ => {}
            }
        }
        // General character mapping
        match c {
            'z' => result.push_str("ts"),
            'w' => result.push('v'),
            'v' => result.push('f'),
            'ß' => result.push('s'),
            'x' => result.push_str("ks"),
            'ä' => result.push_str("ɛː"),
            'ö' => result.push_str("øː"),
            'ü' => result.push_str("yː"),
            'r' => result.push('ʁ'),
            's' => {
                let is_voiced = next
                    .is_some_and(|n| matches!(n, 'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü'));
                if is_voiced
                    && (i == 0
                        || matches!(
                            prev,
                            Some(
                                'a' | 'e'
                                    | 'i'
                                    | 'o'
                                    | 'u'
                                    | 'ä'
                                    | 'ö'
                                    | 'ü'
                                    | 'l'
                                    | 'm'
                                    | 'n'
                                    | 'r'
                            )
                        ))
                {
                    result.push('z');
                } else {
                    result.push('s');
                }
            }
            _ => result.push(c),
        }
        i += 1;
    }
    result
}
