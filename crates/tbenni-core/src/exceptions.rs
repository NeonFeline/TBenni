//! Curated lexical exception tables for Anglicisms, loanwords, and stress exceptions.

use crate::phoneme::Phoneme;
use crate::strutil;

/// Distilled loanword and acronym static lexicon, sorted for O(log N) binary search.
static LOANWORD_LEXICON: &[(&str, &[Phoneme])] = &[
    ("abc", &[Phoneme::A, Phoneme::B, Phoneme::E, Phoneme::C, Phoneme::E]),
    ("abw", &[Phoneme::A, Phoneme::B, Phoneme::E, Phoneme::V, Phoneme::U]),
    ("afc", &[Phoneme::A, Phoneme::E, Phoneme::F, Phoneme::C, Phoneme::E]),
    ("afgańska", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::K, Phoneme::A]),
    ("afgańskich", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::I, Phoneme::Ch]),
    ("afgańskie", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::E]),
    ("afgańskiego", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::E, Phoneme::G, Phoneme::O]),
    ("afgańskiej", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::E, Phoneme::J]),
    ("afgańskiemu", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::E, Phoneme::M, Phoneme::U]),
    ("afgańskim", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::I, Phoneme::M]),
    ("afgańskimi", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::I, Phoneme::MPal, Phoneme::I]),
    ("afgańską", &[Phoneme::A, Phoneme::F, Phoneme::G, Phoneme::A, Phoneme::Ni, Phoneme::S, Phoneme::K, Phoneme::ONasal]),
    ("agd", &[Phoneme::A, Phoneme::GPal, Phoneme::E, Phoneme::D, Phoneme::E]),
    ("anion", &[Phoneme::A, Phoneme::Ni, Phoneme::J, Phoneme::O, Phoneme::N]),
    ("anons", &[Phoneme::A, Phoneme::N, Phoneme::ONasal, Phoneme::S]),
    ("apartheid", &[Phoneme::A, Phoneme::P, Phoneme::A, Phoneme::R, Phoneme::T, Phoneme::Ch, Phoneme::E, Phoneme::J, Phoneme::T]),
    ("apple", &[Phoneme::E, Phoneme::P, Phoneme::L]),
    ("atelier", &[Phoneme::A, Phoneme::T, Phoneme::E, Phoneme::L, Phoneme::J, Phoneme::E]),
    ("attache", &[Phoneme::A, Phoneme::T, Phoneme::A, Phoneme::Sz, Phoneme::E]),
    ("auckland", &[Phoneme::O, Phoneme::K, Phoneme::L, Phoneme::A, Phoneme::N, Phoneme::T]),
    ("barbecue", &[Phoneme::B, Phoneme::A, Phoneme::R, Phoneme::B, Phoneme::E, Phoneme::K, Phoneme::J, Phoneme::U]),
    ("baseball", &[Phoneme::B, Phoneme::E, Phoneme::J, Phoneme::Z, Phoneme::B, Phoneme::O, Phoneme::L]),
    ("beethoven", &[Phoneme::B, Phoneme::E, Phoneme::T, Phoneme::O, Phoneme::V, Phoneme::E, Phoneme::N]),
    ("bezie", &[Phoneme::B, Phoneme::E, Phoneme::Zi, Phoneme::E]),
    ("birmingham", &[Phoneme::BPal, Phoneme::I, Phoneme::R, Phoneme::MPal, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::A, Phoneme::M]),
    ("blues", &[Phoneme::B, Phoneme::L, Phoneme::U, Phoneme::S]),
    ("breakdance", &[Phoneme::B, Phoneme::R, Phoneme::E, Phoneme::J, Phoneme::K, Phoneme::D, Phoneme::E, Phoneme::N, Phoneme::S]),
    ("bridgetown", &[Phoneme::B, Phoneme::RPal, Phoneme::I, Phoneme::Cz, Phoneme::T, Phoneme::A, Phoneme::W, Phoneme::N]),
    ("briefing", &[Phoneme::B, Phoneme::RPal, Phoneme::I, Phoneme::FPal, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("bushido", &[Phoneme::B, Phoneme::U, Phoneme::Si, Phoneme::I, Phoneme::D, Phoneme::O]),
    ("caf", &[Phoneme::K, Phoneme::A, Phoneme::F]),
    ("camembert", &[Phoneme::K, Phoneme::A, Phoneme::M, Phoneme::E, Phoneme::M, Phoneme::B, Phoneme::E, Phoneme::R]),
    ("canberra", &[Phoneme::K, Phoneme::A, Phoneme::N, Phoneme::B, Phoneme::E, Phoneme::R, Phoneme::A]),
    ("cappuccino", &[Phoneme::K, Phoneme::A, Phoneme::P, Phoneme::U, Phoneme::Cz, Phoneme::I, Phoneme::N, Phoneme::O]),
    ("casting", &[Phoneme::K, Phoneme::A, Phoneme::S, Phoneme::T, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("castingowy", &[Phoneme::K, Phoneme::A, Phoneme::S, Phoneme::T, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::O, Phoneme::V, Phoneme::Y]),
    ("catering", &[Phoneme::K, Phoneme::A, Phoneme::T, Phoneme::E, Phoneme::RPal, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("cateringowy", &[Phoneme::K, Phoneme::A, Phoneme::T, Phoneme::E, Phoneme::RPal, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::O, Phoneme::V, Phoneme::Y]),
    ("cheerleaderka", &[Phoneme::Cz, Phoneme::I, Phoneme::R, Phoneme::LPal, Phoneme::I, Phoneme::D, Phoneme::E, Phoneme::R, Phoneme::K, Phoneme::A]),
    ("cheeseburger", &[Phoneme::Cz, Phoneme::I, Phoneme::S, Phoneme::B, Phoneme::U, Phoneme::R, Phoneme::G, Phoneme::E, Phoneme::R]),
    ("chihuahua", &[Phoneme::Cz, Phoneme::I, Phoneme::W, Phoneme::A, Phoneme::W, Phoneme::A]),
    ("chile", &[Phoneme::Cz, Phoneme::I, Phoneme::L, Phoneme::E]),
    ("chili", &[Phoneme::Cz, Phoneme::I, Phoneme::LPal, Phoneme::I]),
    ("chilijczyk", &[Phoneme::Cz, Phoneme::I, Phoneme::LPal, Phoneme::I, Phoneme::J, Phoneme::Cz, Phoneme::Y, Phoneme::K]),
    ("chilijka", &[Phoneme::Cz, Phoneme::I, Phoneme::LPal, Phoneme::I, Phoneme::J, Phoneme::K, Phoneme::A]),
    ("chilijski", &[Phoneme::Cz, Phoneme::I, Phoneme::LPal, Phoneme::I, Phoneme::J, Phoneme::S, Phoneme::KPal, Phoneme::I]),
    ("chilli", &[Phoneme::Cz, Phoneme::I, Phoneme::LPal, Phoneme::I]),
    ("chip", &[Phoneme::Cz, Phoneme::I, Phoneme::P]),
    ("chips", &[Phoneme::Cz, Phoneme::I, Phoneme::P, Phoneme::S]),
    ("cliché", &[Phoneme::K, Phoneme::LPal, Phoneme::I, Phoneme::Sz, Phoneme::E]),
    ("clown", &[Phoneme::K, Phoneme::L, Phoneme::A, Phoneme::W, Phoneme::N]),
    ("coach", &[Phoneme::K, Phoneme::O, Phoneme::W, Phoneme::Cz]),
    ("coleslaw", &[Phoneme::K, Phoneme::O, Phoneme::L, Phoneme::S, Phoneme::L, Phoneme::O, Phoneme::W]),
    ("cool", &[Phoneme::K, Phoneme::U, Phoneme::L]),
    ("copywriter", &[Phoneme::K, Phoneme::O, Phoneme::PPal, Phoneme::I, Phoneme::R, Phoneme::A, Phoneme::J, Phoneme::T, Phoneme::E, Phoneme::R]),
    ("country", &[Phoneme::K, Phoneme::A, Phoneme::N, Phoneme::T, Phoneme::R, Phoneme::Y]),
    ("coventry", &[Phoneme::K, Phoneme::O, Phoneme::V, Phoneme::E, Phoneme::N, Phoneme::T, Phoneme::R, Phoneme::I]),
    ("cover", &[Phoneme::K, Phoneme::A, Phoneme::V, Phoneme::E, Phoneme::R]),
    ("curling", &[Phoneme::K, Phoneme::E, Phoneme::R, Phoneme::LPal, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("curry", &[Phoneme::K, Phoneme::A, Phoneme::R, Phoneme::Y]),
    ("cv", &[Phoneme::S, Phoneme::I, Phoneme::VPal, Phoneme::I]),
    ("cyberpunk", &[Phoneme::C, Phoneme::Y, Phoneme::B, Phoneme::E, Phoneme::R, Phoneme::P, Phoneme::A, Phoneme::N, Phoneme::K]),
    ("deadline", &[Phoneme::D, Phoneme::E, Phoneme::D, Phoneme::L, Phoneme::A, Phoneme::J, Phoneme::N]),
    ("dealer", &[Phoneme::D, Phoneme::I, Phoneme::L, Phoneme::E, Phoneme::R]),
    ("demonstrować", &[Phoneme::D, Phoneme::E, Phoneme::M, Phoneme::ONasal, Phoneme::S, Phoneme::T, Phoneme::R, Phoneme::O, Phoneme::V, Phoneme::A, Phoneme::Ci]),
    ("design", &[Phoneme::D, Phoneme::I, Phoneme::Z, Phoneme::A, Phoneme::J, Phoneme::N]),
    ("designer", &[Phoneme::D, Phoneme::I, Phoneme::Z, Phoneme::A, Phoneme::J, Phoneme::N, Phoneme::E, Phoneme::R]),
    ("diesel", &[Phoneme::D, Phoneme::I, Phoneme::Z, Phoneme::E, Phoneme::L]),
    ("disco", &[Phoneme::D, Phoneme::I, Phoneme::S, Phoneme::K, Phoneme::O]),
    ("dj", &[Phoneme::D, Phoneme::I, Phoneme::DzRetro, Phoneme::E, Phoneme::J]),
    ("eugeniusz", &[Phoneme::E, Phoneme::W, Phoneme::G, Phoneme::E, Phoneme::Ni, Phoneme::J, Phoneme::U, Phoneme::Sz]),
    ("expose", &[Phoneme::E, Phoneme::K, Phoneme::S, Phoneme::P, Phoneme::O, Phoneme::Z, Phoneme::E]),
    ("ezt", &[Phoneme::E, Phoneme::Z, Phoneme::E, Phoneme::T]),
    ("facebook", &[Phoneme::F, Phoneme::E, Phoneme::J, Phoneme::S, Phoneme::B, Phoneme::U, Phoneme::K]),
    ("fair", &[Phoneme::F, Phoneme::E, Phoneme::R]),
    ("flevoland", &[Phoneme::F, Phoneme::L, Phoneme::E, Phoneme::J, Phoneme::V, Phoneme::O, Phoneme::L, Phoneme::A, Phoneme::N, Phoneme::T]),
    ("foyer", &[Phoneme::F, Phoneme::W, Phoneme::A, Phoneme::J, Phoneme::E]),
    ("freelancer", &[Phoneme::F, Phoneme::RPal, Phoneme::I, Phoneme::L, Phoneme::E, Phoneme::N, Phoneme::S, Phoneme::E, Phoneme::R]),
    ("frisbee", &[Phoneme::F, Phoneme::RPal, Phoneme::I, Phoneme::Z, Phoneme::BPal, Phoneme::I]),
    ("gender", &[Phoneme::DzRetro, Phoneme::E, Phoneme::N, Phoneme::D, Phoneme::E, Phoneme::R]),
    ("gensior", &[Phoneme::G, Phoneme::ENasal, Phoneme::Si, Phoneme::O, Phoneme::R]),
    ("ghana", &[Phoneme::G, Phoneme::A, Phoneme::N, Phoneme::A]),
    ("gnocchi", &[Phoneme::Ni, Phoneme::O, Phoneme::K, Phoneme::KPal, Phoneme::I]),
    ("gonsior", &[Phoneme::G, Phoneme::ONasal, Phoneme::Si, Phoneme::O, Phoneme::R]),
    ("google", &[Phoneme::G, Phoneme::U, Phoneme::G, Phoneme::L]),
    ("googlować", &[Phoneme::G, Phoneme::U, Phoneme::G, Phoneme::L, Phoneme::O, Phoneme::V, Phoneme::A, Phoneme::Ci]),
    ("gouda", &[Phoneme::G, Phoneme::O, Phoneme::W, Phoneme::D, Phoneme::A]),
    ("groningen", &[Phoneme::Ch, Phoneme::R, Phoneme::O, Phoneme::W, Phoneme::Ni, Phoneme::I, Phoneme::Ng, Phoneme::G, Phoneme::E, Phoneme::N]),
    ("groningski", &[Phoneme::Ch, Phoneme::R, Phoneme::O, Phoneme::Ni, Phoneme::I, Phoneme::Ni, Phoneme::S, Phoneme::KPal, Phoneme::I]),
    ("hainaut", &[Phoneme::E, Phoneme::N, Phoneme::O]),
    ("hardware", &[Phoneme::Ch, Phoneme::A, Phoneme::R, Phoneme::D, Phoneme::V, Phoneme::E, Phoneme::R]),
    ("headshot", &[Phoneme::Ch, Phoneme::E, Phoneme::T, Phoneme::Sz, Phoneme::O, Phoneme::T]),
    ("helsinki", &[Phoneme::Ch, Phoneme::E, Phoneme::L, Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::KPal, Phoneme::I]),
    ("house", &[Phoneme::Ch, Phoneme::A, Phoneme::W, Phoneme::S]),
    ("ibook", &[Phoneme::A, Phoneme::J, Phoneme::B, Phoneme::U, Phoneme::K]),
    ("idaho", &[Phoneme::A, Phoneme::J, Phoneme::D, Phoneme::E, Phoneme::Ch, Phoneme::O, Phoneme::W]),
    ("igloo", &[Phoneme::I, Phoneme::G, Phoneme::L, Phoneme::U]),
    ("imac", &[Phoneme::A, Phoneme::J, Phoneme::M, Phoneme::A, Phoneme::K]),
    ("impeachment", &[Phoneme::I, Phoneme::M, Phoneme::PPal, Phoneme::I, Phoneme::Cz, Phoneme::M, Phoneme::E, Phoneme::N, Phoneme::T]),
    ("iowa", &[Phoneme::A, Phoneme::J, Phoneme::O, Phoneme::W, Phoneme::A]),
    ("ipa", &[Phoneme::A, Phoneme::J, Phoneme::PPal, Phoneme::I, Phoneme::E, Phoneme::J]),
    ("iphone", &[Phoneme::A, Phoneme::J, Phoneme::F, Phoneme::O, Phoneme::N]),
    ("ipod", &[Phoneme::A, Phoneme::J, Phoneme::P, Phoneme::O, Phoneme::T]),
    ("iq", &[Phoneme::A, Phoneme::J, Phoneme::K, Phoneme::J, Phoneme::U]),
    ("ironie", &[Phoneme::I, Phoneme::R, Phoneme::O, Phoneme::Ni, Phoneme::J, Phoneme::E]),
    ("jacuzzi", &[Phoneme::DzRetro, Phoneme::A, Phoneme::K, Phoneme::U, Phoneme::Z, Phoneme::I]),
    ("jazz", &[Phoneme::DzRetro, Phoneme::E, Phoneme::S]),
    ("jeep", &[Phoneme::DzRetro, Phoneme::I, Phoneme::P]),
    ("jogging", &[Phoneme::DzRetro, Phoneme::O, Phoneme::GPal, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("joker", &[Phoneme::DzRetro, Phoneme::O, Phoneme::K, Phoneme::E, Phoneme::R]),
    ("joypad", &[Phoneme::DzRetro, Phoneme::O, Phoneme::J, Phoneme::P, Phoneme::A, Phoneme::T]),
    ("jury", &[Phoneme::ZRetro, Phoneme::Y, Phoneme::RPal, Phoneme::I]),
    ("kanion", &[Phoneme::K, Phoneme::A, Phoneme::Ni, Phoneme::J, Phoneme::O, Phoneme::N]),
    ("kanji", &[Phoneme::K, Phoneme::A, Phoneme::N, Phoneme::Dzi, Phoneme::I]),
    ("kentucky", &[Phoneme::K, Phoneme::E, Phoneme::N, Phoneme::T, Phoneme::A, Phoneme::KPal, Phoneme::I]),
    ("ketchup", &[Phoneme::K, Phoneme::E, Phoneme::Cz, Phoneme::A, Phoneme::P]),
    ("kingstown", &[Phoneme::KPal, Phoneme::I, Phoneme::N, Phoneme::K, Phoneme::S, Phoneme::T, Phoneme::A, Phoneme::W, Phoneme::N]),
    ("klown", &[Phoneme::K, Phoneme::L, Phoneme::A, Phoneme::W, Phoneme::N]),
    ("kondensator", &[Phoneme::K, Phoneme::O, Phoneme::N, Phoneme::D, Phoneme::ENasal, Phoneme::S, Phoneme::A, Phoneme::T, Phoneme::O, Phoneme::R]),
    ("konsolidator", &[Phoneme::K, Phoneme::ONasal, Phoneme::S, Phoneme::O, Phoneme::LPal, Phoneme::I, Phoneme::D, Phoneme::A, Phoneme::T, Phoneme::O, Phoneme::R]),
    ("label", &[Phoneme::L, Phoneme::E, Phoneme::J, Phoneme::B, Phoneme::E, Phoneme::L]),
    ("lead", &[Phoneme::LPal, Phoneme::I, Phoneme::T]),
    ("leasing", &[Phoneme::LPal, Phoneme::I, Phoneme::Z, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("leasingowy", &[Phoneme::LPal, Phoneme::I, Phoneme::Z, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::O, Phoneme::V, Phoneme::Y]),
    ("liechtenstein", &[Phoneme::LPal, Phoneme::I, Phoneme::Ch, Phoneme::T, Phoneme::E, Phoneme::N, Phoneme::Sz, Phoneme::T, Phoneme::A, Phoneme::J, Phoneme::N]),
    ("liniowy", &[Phoneme::LPal, Phoneme::I, Phoneme::Ni, Phoneme::J, Phoneme::O, Phoneme::V, Phoneme::Y]),
    ("macao", &[Phoneme::M, Phoneme::A, Phoneme::K, Phoneme::A, Phoneme::O]),
    ("macbook", &[Phoneme::M, Phoneme::A, Phoneme::G, Phoneme::B, Phoneme::U, Phoneme::K]),
    ("macho", &[Phoneme::M, Phoneme::A, Phoneme::Cz, Phoneme::O]),
    ("macintosh", &[Phoneme::M, Phoneme::A, Phoneme::KPal, Phoneme::I, Phoneme::N, Phoneme::T, Phoneme::O, Phoneme::Sz]),
    ("maf", &[Phoneme::E, Phoneme::M, Phoneme::A, Phoneme::E, Phoneme::F]),
    ("maine", &[Phoneme::M, Phoneme::E, Phoneme::J, Phoneme::N]),
    ("maksimum", &[Phoneme::M, Phoneme::A, Phoneme::K, Phoneme::S, Phoneme::I, Phoneme::M, Phoneme::U, Phoneme::M]),
    ("management", &[Phoneme::M, Phoneme::E, Phoneme::N, Phoneme::E, Phoneme::DzRetro, Phoneme::M, Phoneme::E, Phoneme::N, Phoneme::T]),
    ("manchester", &[Phoneme::M, Phoneme::A, Phoneme::N, Phoneme::Cz, Phoneme::E, Phoneme::S, Phoneme::T, Phoneme::E, Phoneme::R]),
    ("massachusetts", &[Phoneme::M, Phoneme::A, Phoneme::S, Phoneme::A, Phoneme::Cz, Phoneme::J, Phoneme::U, Phoneme::S, Phoneme::E, Phoneme::T, Phoneme::S]),
    ("meitner", &[Phoneme::M, Phoneme::A, Phoneme::J, Phoneme::T, Phoneme::N, Phoneme::E, Phoneme::R]),
    ("melbourne", &[Phoneme::M, Phoneme::E, Phoneme::L, Phoneme::B, Phoneme::E, Phoneme::R, Phoneme::N]),
    ("menu", &[Phoneme::M, Phoneme::E, Phoneme::Ni, Phoneme::I]),
    ("michigan", &[Phoneme::MPal, Phoneme::I, Phoneme::Sz, Phoneme::I, Phoneme::G, Phoneme::A, Phoneme::N]),
    ("missouri", &[Phoneme::MPal, Phoneme::I, Phoneme::Z, Phoneme::U, Phoneme::RPal, Phoneme::I]),
    ("monsior", &[Phoneme::M, Phoneme::ONasal, Phoneme::Si, Phoneme::O, Phoneme::R]),
    ("mozzarella", &[Phoneme::M, Phoneme::O, Phoneme::C, Phoneme::A, Phoneme::R, Phoneme::E, Phoneme::L, Phoneme::L, Phoneme::A]),
    ("musical", &[Phoneme::M, Phoneme::J, Phoneme::U, Phoneme::Z, Phoneme::I, Phoneme::K, Phoneme::A, Phoneme::L]),
    ("naypyidaw", &[Phoneme::N, Phoneme::E, Phoneme::J, Phoneme::PPal, Phoneme::I, Phoneme::D, Phoneme::O]),
    ("newcastle", &[Phoneme::Ni, Phoneme::U, Phoneme::K, Phoneme::A, Phoneme::S, Phoneme::L]),
    ("news", &[Phoneme::Ni, Phoneme::U, Phoneme::S]),
    ("newsowy", &[Phoneme::Ni, Phoneme::U, Phoneme::S, Phoneme::O, Phoneme::V, Phoneme::Y]),
    ("nick", &[Phoneme::Ni, Phoneme::I, Phoneme::K]),
    ("orwellowski", &[Phoneme::O, Phoneme::R, Phoneme::W, Phoneme::E, Phoneme::L, Phoneme::O, Phoneme::F, Phoneme::S, Phoneme::KPal, Phoneme::I]),
    ("outsourcing", &[Phoneme::A, Phoneme::W, Phoneme::T, Phoneme::S, Phoneme::O, Phoneme::R, Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("overijssel", &[Phoneme::O, Phoneme::V, Phoneme::E, Phoneme::R, Phoneme::E, Phoneme::J, Phoneme::S, Phoneme::E, Phoneme::L]),
    ("peeling", &[Phoneme::PPal, Phoneme::I, Phoneme::LPal, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("pendrive", &[Phoneme::P, Phoneme::E, Phoneme::N, Phoneme::D, Phoneme::R, Phoneme::A, Phoneme::J, Phoneme::F]),
    ("pepsi", &[Phoneme::P, Phoneme::E, Phoneme::P, Phoneme::S, Phoneme::I]),
    ("phenian", &[Phoneme::F, Phoneme::E, Phoneme::Ni, Phoneme::J, Phoneme::A, Phoneme::N]),
    ("piercing", &[Phoneme::PPal, Phoneme::I, Phoneme::R, Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("pizza", &[Phoneme::PPal, Phoneme::I, Phoneme::C, Phoneme::C, Phoneme::A]),
    ("pizzeria", &[Phoneme::PPal, Phoneme::I, Phoneme::C, Phoneme::C, Phoneme::E, Phoneme::RPal, Phoneme::J, Phoneme::A]),
    ("playlista", &[Phoneme::P, Phoneme::L, Phoneme::E, Phoneme::J, Phoneme::LPal, Phoneme::I, Phoneme::S, Phoneme::T, Phoneme::A]),
    ("podcasting", &[Phoneme::P, Phoneme::O, Phoneme::T, Phoneme::K, Phoneme::A, Phoneme::S, Phoneme::T, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("point", &[Phoneme::P, Phoneme::W, Phoneme::E, Phoneme::N, Phoneme::T]),
    ("poncho", &[Phoneme::P, Phoneme::O, Phoneme::N, Phoneme::Cz, Phoneme::O]),
    ("pr", &[Phoneme::PPal, Phoneme::I, Phoneme::A, Phoneme::R]),
    ("prequel", &[Phoneme::P, Phoneme::RPal, Phoneme::I, Phoneme::K, Phoneme::W, Phoneme::E, Phoneme::L]),
    ("prr", &[Phoneme::P, Phoneme::R]),
    ("quad", &[Phoneme::K, Phoneme::W, Phoneme::O, Phoneme::T]),
    ("rancho", &[Phoneme::R, Phoneme::A, Phoneme::N, Phoneme::Cz, Phoneme::O]),
    ("rating", &[Phoneme::R, Phoneme::E, Phoneme::J, Phoneme::T, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("reykjavik", &[Phoneme::R, Phoneme::E, Phoneme::J, Phoneme::KPal, Phoneme::J, Phoneme::A, Phoneme::VPal, Phoneme::I, Phoneme::K]),
    ("reykjavícki", &[Phoneme::R, Phoneme::E, Phoneme::J, Phoneme::KPal, Phoneme::J, Phoneme::A, Phoneme::VPal, Phoneme::I, Phoneme::C, Phoneme::KPal, Phoneme::I]),
    ("reykjavík", &[Phoneme::R, Phoneme::E, Phoneme::J, Phoneme::KPal, Phoneme::J, Phoneme::A, Phoneme::VPal, Phoneme::I, Phoneme::K]),
    ("rock", &[Phoneme::R, Phoneme::O, Phoneme::K]),
    ("rockowy", &[Phoneme::R, Phoneme::O, Phoneme::K, Phoneme::O, Phoneme::V, Phoneme::Y]),
    ("roseau", &[Phoneme::R, Phoneme::O, Phoneme::Z, Phoneme::O]),
    ("sashimi", &[Phoneme::S, Phoneme::A, Phoneme::Si, Phoneme::I, Phoneme::MPal, Phoneme::I]),
    ("schadenfreude", &[Phoneme::Sz, Phoneme::A, Phoneme::D, Phoneme::E, Phoneme::N, Phoneme::F, Phoneme::R, Phoneme::O, Phoneme::J, Phoneme::D, Phoneme::E]),
    ("show", &[Phoneme::Sz, Phoneme::O, Phoneme::W]),
    ("silos", &[Phoneme::S, Phoneme::I, Phoneme::L, Phoneme::O, Phoneme::S]),
    ("singapur", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::A, Phoneme::P, Phoneme::U, Phoneme::R]),
    ("singapurczyk", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::A, Phoneme::P, Phoneme::U, Phoneme::R, Phoneme::Cz, Phoneme::Y, Phoneme::K]),
    ("singapurka", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::A, Phoneme::P, Phoneme::U, Phoneme::R, Phoneme::K, Phoneme::A]),
    ("singapurski", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::A, Phoneme::P, Phoneme::U, Phoneme::R, Phoneme::S, Phoneme::KPal, Phoneme::I]),
    ("singel", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::G, Phoneme::E, Phoneme::L]),
    ("singiel", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::GPal, Phoneme::E, Phoneme::L]),
    ("sinologia", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::O, Phoneme::L, Phoneme::O, Phoneme::GPal, Phoneme::J, Phoneme::A]),
    ("sinus", &[Phoneme::S, Phoneme::I, Phoneme::N, Phoneme::U, Phoneme::S]),
    ("software", &[Phoneme::S, Phoneme::O, Phoneme::F, Phoneme::T, Phoneme::V, Phoneme::E, Phoneme::R]),
    ("spray", &[Phoneme::S, Phoneme::P, Phoneme::R, Phoneme::E, Phoneme::J]),
    ("squash", &[Phoneme::S, Phoneme::K, Phoneme::W, Phoneme::O, Phoneme::Sz]),
    ("squat", &[Phoneme::S, Phoneme::K, Phoneme::W, Phoneme::O, Phoneme::T]),
    ("staccato", &[Phoneme::S, Phoneme::T, Phoneme::A, Phoneme::K, Phoneme::K, Phoneme::A, Phoneme::T, Phoneme::O]),
    ("steward", &[Phoneme::S, Phoneme::T, Phoneme::J, Phoneme::U, Phoneme::A, Phoneme::R, Phoneme::T]),
    ("stewardesa", &[Phoneme::S, Phoneme::T, Phoneme::J, Phoneme::U, Phoneme::A, Phoneme::R, Phoneme::D, Phoneme::E, Phoneme::S, Phoneme::A]),
    ("stewardessa", &[Phoneme::S, Phoneme::T, Phoneme::J, Phoneme::U, Phoneme::A, Phoneme::R, Phoneme::D, Phoneme::E, Phoneme::S, Phoneme::S, Phoneme::A]),
    ("sydney", &[Phoneme::S, Phoneme::Y, Phoneme::D, Phoneme::N, Phoneme::I]),
    ("sześćset", &[Phoneme::Sz, Phoneme::E, Phoneme::J, Phoneme::S, Phoneme::E, Phoneme::T]),
    ("tbilisi", &[Phoneme::T, Phoneme::BPal, Phoneme::I, Phoneme::LPal, Phoneme::I, Phoneme::S, Phoneme::I]),
    ("tennessee", &[Phoneme::T, Phoneme::E, Phoneme::N, Phoneme::E, Phoneme::S, Phoneme::I]),
    ("thimphu", &[Phoneme::T, Phoneme::I, Phoneme::M, Phoneme::F, Phoneme::U]),
    ("thorshavn", &[Phoneme::T, Phoneme::O, Phoneme::R, Phoneme::S, Phoneme::Ch, Phoneme::A, Phoneme::W, Phoneme::N]),
    ("tournée", &[Phoneme::T, Phoneme::U, Phoneme::R, Phoneme::N, Phoneme::E]),
    ("tweet", &[Phoneme::T, Phoneme::W, Phoneme::I, Phoneme::T]),
    ("tweetować", &[Phoneme::T, Phoneme::W, Phoneme::I, Phoneme::T, Phoneme::O, Phoneme::V, Phoneme::A, Phoneme::Ci]),
    ("twitter", &[Phoneme::T, Phoneme::W, Phoneme::I, Phoneme::T, Phoneme::E, Phoneme::R]),
    ("tzatziki", &[Phoneme::C, Phoneme::A, Phoneme::C, Phoneme::I, Phoneme::KPal, Phoneme::I]),
    ("utah", &[Phoneme::J, Phoneme::U, Phoneme::T, Phoneme::A]),
    ("victoria", &[Phoneme::VPal, Phoneme::I, Phoneme::K, Phoneme::T, Phoneme::O, Phoneme::RPal, Phoneme::J, Phoneme::A]),
    ("vintage", &[Phoneme::VPal, Phoneme::I, Phoneme::N, Phoneme::T, Phoneme::E, Phoneme::DzRetro]),
    ("voilà", &[Phoneme::V, Phoneme::W, Phoneme::A, Phoneme::L, Phoneme::A]),
    ("volapük", &[Phoneme::V, Phoneme::O, Phoneme::L, Phoneme::A, Phoneme::PPal, Phoneme::I, Phoneme::K]),
    ("voodoo", &[Phoneme::V, Phoneme::U, Phoneme::D, Phoneme::U]),
    ("weekendy", &[Phoneme::VPal, Phoneme::I, Phoneme::K, Phoneme::E, Phoneme::N, Phoneme::D, Phoneme::Y]),
    ("wellington", &[Phoneme::W, Phoneme::E, Phoneme::LPal, Phoneme::I, Phoneme::N, Phoneme::K, Phoneme::T, Phoneme::O, Phoneme::N]),
    ("wenzel", &[Phoneme::V, Phoneme::ENasal, Phoneme::Z, Phoneme::E, Phoneme::L]),
    ("wg", &[Phoneme::V, Phoneme::E, Phoneme::D, Phoneme::W, Phoneme::U, Phoneme::K]),
    ("wiench", &[Phoneme::VPal, Phoneme::ENasal, Phoneme::Ch]),
    ("windsurfing", &[Phoneme::W, Phoneme::I, Phoneme::N, Phoneme::T, Phoneme::S, Phoneme::E, Phoneme::R, Phoneme::FPal, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("wku", &[Phoneme::V, Phoneme::U, Phoneme::K, Phoneme::A, Phoneme::U]),
    ("wons", &[Phoneme::V, Phoneme::ONasal, Phoneme::S]),
    ("wow", &[Phoneme::W, Phoneme::A, Phoneme::W]),
    ("wołonsewicz", &[Phoneme::V, Phoneme::O, Phoneme::W, Phoneme::ONasal, Phoneme::S, Phoneme::E, Phoneme::VPal, Phoneme::I, Phoneme::Cz]),
    ("wyoming", &[Phoneme::W, Phoneme::A, Phoneme::J, Phoneme::O, Phoneme::MPal, Phoneme::I, Phoneme::N, Phoneme::K]),
    ("xbox", &[Phoneme::E, Phoneme::K, Phoneme::S, Phoneme::B, Phoneme::O, Phoneme::K, Phoneme::S]),
    ("youtube", &[Phoneme::J, Phoneme::U, Phoneme::T, Phoneme::J, Phoneme::U, Phoneme::P]),
    ("zademonstrować", &[Phoneme::Z, Phoneme::A, Phoneme::D, Phoneme::E, Phoneme::M, Phoneme::ONasal, Phoneme::S, Phoneme::T, Phoneme::R, Phoneme::O, Phoneme::V, Phoneme::A, Phoneme::Ci]),
    ("zimbabwe", &[Phoneme::Z, Phoneme::I, Phoneme::M, Phoneme::B, Phoneme::A, Phoneme::B, Phoneme::V, Phoneme::E]),
    ("ziszczać", &[Phoneme::Z, Phoneme::I, Phoneme::Sz, Phoneme::Cz, Phoneme::A, Phoneme::Ci]),
    ("ziścić", &[Phoneme::Z, Phoneme::I, Phoneme::Si, Phoneme::Ci, Phoneme::I, Phoneme::Ci]),
    ("zoolatria", &[Phoneme::Z, Phoneme::O, Phoneme::GlottalStop, Phoneme::O, Phoneme::L, Phoneme::A, Phoneme::T, Phoneme::RPal, Phoneme::J, Phoneme::A]),
    ("zoolog", &[Phoneme::Z, Phoneme::O, Phoneme::GlottalStop, Phoneme::O, Phoneme::L, Phoneme::O, Phoneme::K]),
    ("zoomorfizm", &[Phoneme::Z, Phoneme::O, Phoneme::GlottalStop, Phoneme::O, Phoneme::M, Phoneme::O, Phoneme::R, Phoneme::FPal, Phoneme::I, Phoneme::Z, Phoneme::M]),
    ("zurych", &[Phoneme::C, Phoneme::U, Phoneme::R, Phoneme::Y, Phoneme::Ch]),
];

/// Check if a lowercase word has a curated phonetic exception override.
#[inline]
pub fn lookup_loanword_override(word: &str) -> Option<&'static [Phoneme]> {
    if let Ok(idx) = LOANWORD_LEXICON.binary_search_by(|(w, _)| strutil::cmp(w, word)) {
        return Some(LOANWORD_LEXICON[idx].1);
    }
    None
}

pub fn is_latin_nia_glide(word: &str) -> bool {
    matches!(
        word,
        "albania"
            | "aragonia"
            | "armenia"
            | "bolonia"
            | "bretania"
            | "estonia"
            | "hiszpania"
            | "japonia"
            | "jordania"
            | "kaledonia"
            | "katalonia"
            | "kenia"
            | "kolonia"
            | "laponia"
            | "macedonia"
            | "mauretania"
            | "oceania"
            | "patagonia"
            | "rumunia"
            | "saksonia"
            | "sardynia"
            | "szampania"
            | "słowenia"
            | "tanzania"
            | "toskania"
            | "agonia"
            | "apolonia"
            | "aronia"
            | "begonia"
            | "cukinia"
            | "dania"
            | "eugenia"
            | "gardenia"
            | "harmonia"
            | "hegemonia"
            | "ironia"
            | "linia"
            | "litania"
            | "opinia"
            | "peonia"
            | "robinia"
            | "symfonia"
            | "unia"
    )
}

/// Check if word is known to have antepenultimate stress in prescriptive Polish.
pub fn is_antepenultimate_stress_word(word: &str) -> bool {
    if word.chars().any(char::is_uppercase) {
        return is_antepenultimate_stress_lowercase(&word.to_lowercase());
    }
    is_antepenultimate_stress_lowercase(word)
}

/// [`is_antepenultimate_stress_word`] for an already lowercased word (the engine's hot path).
pub(crate) fn is_antepenultimate_stress_lowercase(w: &str) -> bool {
    // Borrowings in -yka / -ika (fizyka, Ameryka), except native -atyka (bijatyka, pijatyka)
    if (strutil::has_suffix(w, "yka") || strutil::has_suffix(w, "ika"))
        && !strutil::has_suffix(w, "bijatyka")
        && !strutil::has_suffix(w, "pijatyka")
    {
        return true;
    }
    if !matches!(w.as_bytes().last(), Some(b'a' | b't')) {
        return false;
    }

    matches!(
        w,
        "fizyka"
            | "muzyka"
            | "matematyka"
            | "polityka"
            | "botanika"
            | "logika"
            | "gramatyka"
            | "retoryka"
            | "optyka"
            | "pedagogika"
            | "elektronika"
            | "informatyka"
            | "statystyka"
            | "technika"
            | "praktyka"
            | "krytyka"
            | "akademika"
            | "czterysta"
            | "siedemset"
            | "osiemset"
            | "dziewięćset"
            | "opera"
            | "uniwersytet"
            | "rzeczpospolita"
    )
}
