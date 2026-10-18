"""Test suite for TBenni: High-Performance Polish Phonemizer.
Covers 50+ Polish linguistic traps, dialetical features, and formatting options.
"""

import pytest
import tbenni


class TestSibilantsAndAffricates:
    """Verifies distinction between the three series of sibilants/affricates."""

    def test_affricate_vs_cluster_cz_trz(self):
        # czysta [t͡ʂɨsta] (affricate) vs trzysta [tʂɨsta] (stop + fricative cluster)
        assert tbenni.phonemize("czysta", include_stress=False) == "t͡ʂɨsta"
        assert tbenni.phonemize("trzysta", include_stress=False) == "tʂɨsta"

    def test_affricate_vs_cluster_dz_drz(self):
        # dżem [d͡ʐɛm] vs drzewo [dʐɛvɔ]
        assert tbenni.phonemize("dżem", include_stress=False) == "d͡ʐɛm"
        assert tbenni.phonemize("drzewo", include_stress=False) == "dʐɛvɔ"

    def test_alveolo_palatals(self):
        assert tbenni.phonemize("ćma", include_stress=False) == "t͡ɕma"
        assert tbenni.phonemize("śnieg", include_stress=False) == "ɕɲɛk"
        assert tbenni.phonemize("źrebak", include_stress=False) == "ʑrɛbak"
        assert tbenni.phonemize("dźwięk", include_stress=False) == "d͡ʑvʲjɛŋk" or "d͡ʑvʲɛŋk" in tbenni.phonemize("dźwięk", include_stress=False)


class TestMorphemeBoundaries:
    """Verifies that prefix boundaries and morphemes prevent illegal merging."""

    def test_marznac_family_r_z(self):
        # marznąć [marznɔɲt͡ɕ] (r + z), NOT [maʐnɔɲt͡ɕ] with digraph!
        assert tbenni.phonemize("marznąć", include_stress=False) == "marznɔɲt͡ɕ"
        assert tbenni.phonemize("zmarzlak", include_stress=False) == "zmarzlak"
        assert tbenni.phonemize("odmarzać", include_stress=False).startswith("ɔdmarz")

    def test_standard_rz_digraph(self):
        # morze [mɔʐɛ], rzeka [ʐɛka]
        assert tbenni.phonemize("morze", include_stress=False) == "mɔʐɛ"
        assert tbenni.phonemize("rzeka", include_stress=False) == "ʐɛka"

    def test_prefix_d_plus_sibilant(self):
        # odznaczyć [ɔdznat͡ʂɨt͡ɕ] (d + z), NOT affricate [ɔd͡znat͡ʂɨt͡ɕ]!
        assert tbenni.phonemize("odznaczyć", include_stress=False) == "ɔdznat͡ʂɨt͡ɕ"
        assert tbenni.phonemize("odzew", include_stress=False) == "ɔdzɛf"
        assert tbenni.phonemize("podzamcze", include_stress=False) == "pɔdzamt͡ʂɛ"
        assert tbenni.phonemize("nadzwyczajny", include_stress=False).startswith("nadzv")


class TestEspeakBug2295Regression:
    """Verifies that 'zi' and 'si' before consonants palatalize properly."""

    def test_zi_before_consonant(self):
        # obraził [ɔbraʑiw], NOT illegal eSpeak NG [ɔbraziw]
        assert tbenni.phonemize("obraził", include_stress=False) == "ɔbraʑiw"
        assert tbenni.phonemize("zima", include_stress=False) == "ʑima"
        assert tbenni.phonemize("ziarno", include_stress=False) == "ʑarnɔ"

    def test_si_before_consonant(self):
        assert tbenni.phonemize("sito", include_stress=False) == "ɕitɔ"
        assert tbenni.phonemize("siwy", include_stress=False) == "ɕivɨ"
        assert tbenni.phonemize("siano", include_stress=False) == "ɕanɔ"


class TestNasalVowelDecomposition:
    """Verifies allophonic realization across all phonetic environments."""

    def test_bilabials(self):
        # dąb [dɔmp], zęby [zɛmbɨ]
        assert tbenni.phonemize("dąb", include_stress=False) == "dɔmp"
        assert tbenni.phonemize("zęby", include_stress=False) == "zɛmbɨ"

    def test_dentals(self):
        # kąt [kɔnt], wędka [vɛntka]
        assert tbenni.phonemize("kąt", include_stress=False) == "kɔnt"
        assert tbenni.phonemize("wędka", include_stress=False) == "vɛntka"

    def test_alveolo_palatals(self):
        # pięć [pʲɛɲt͡ɕ], będzie [bɛɲd͡ʑɛ]
        assert tbenni.phonemize("pięć", include_stress=False) == "pʲɛɲt͡ɕ"
        assert tbenni.phonemize("będzie", include_stress=False) == "bɛɲd͡ʑɛ"

    def test_velars(self):
        # ręka [rɛŋka], bąk [bɔŋk]
        assert tbenni.phonemize("ręka", include_stress=False) == "rɛŋka"
        assert tbenni.phonemize("bąk", include_stress=False) == "bɔŋk"

    def test_liquids(self):
        # wziął [vʑɔw], wzięli [vʑɛlʲi] (complete denasalization)
        assert tbenni.phonemize("wziął", include_stress=False) == "vʑɔw"
        assert tbenni.phonemize("wzięli", include_stress=False) == "vʑɛlʲi"

    def test_fricatives(self):
        # wąs [vɔw̃s], mięso [mʲɛw̃sɔ], gęś [ɡɛw̃ɕ]
        assert tbenni.phonemize("wąs", include_stress=False) == "vɔw̃s"
        assert tbenni.phonemize("mięso", include_stress=False) == "mʲɛw̃sɔ"
        assert tbenni.phonemize("gęś", include_stress=False) == "ɡɛw̃ɕ"

    def test_word_final(self):
        # idę [idɛ] (denasalized), idą [idɔw̃] (nasal diphthong)
        assert tbenni.phonemize("idę", include_stress=False) == "idɛ"
        assert tbenni.phonemize("idą", include_stress=False) == "idɔw̃"


class TestVoicingAssimilation:
    """Verifies regressive and progressive voicing, and final devoicing."""

    def test_final_devoicing(self):
        # chleb [xlɛp], mózg [musk], wódz [vut͡s]
        assert tbenni.phonemize("chleb", include_stress=False) == "xlɛp"
        assert tbenni.phonemize("mózg", include_stress=False) == "musk"
        assert tbenni.phonemize("wódz", include_stress=False) == "vut͡s"

    def test_regressive_voicing(self):
        # prośba [prɔʑba], także [taɡʐɛ], jakby [jaɡbɨ]
        assert tbenni.phonemize("prośba", include_stress=False) == "prɔʑba"
        assert tbenni.phonemize("także", include_stress=False) == "taɡʐɛ"
        assert tbenni.phonemize("jakby", include_stress=False) == "jaɡbɨ"

    def test_progressive_devoicing(self):
        # twój [tfuj], krzak [kʂak], trzeba [tʂɛba]
        assert tbenni.phonemize("twój", include_stress=False) == "tfuj"
        assert tbenni.phonemize("krzak", include_stress=False) == "kʂak"
        assert tbenni.phonemize("trzeba", include_stress=False) == "tʂɛba"


class TestSandhiAndDialects:
    """Verifies sentence-level Sandhi in Warsaw, Cracow, and Silesian."""

    def test_warsaw_sandhi(self):
        # Inter-word devoicing before vowels and sonorants
        assert tbenni.phonemize("brat ojca", dialect="warsaw", include_stress=False) == "brat ɔjt͡sa"
        assert tbenni.phonemize("las rośnie", dialect="warsaw", include_stress=False) == "las rɔɕɲɛ"

    def test_krakow_sandhi(self):
        # Inter-word voicing before vowels and sonorants
        assert tbenni.phonemize("brat ojca", dialect="krakow", include_stress=False) == "brad ɔjt͡sa"
        assert tbenni.phonemize("las rośnie", dialect="krakow", include_stress=False) == "laz rɔɕɲɛ"

    def test_silesian_dialect(self):
        # Initial o- labialization: okno -> [wɔknɔ]
        res = tbenni.phonemize("okno", dialect="slask", include_stress=False)
        assert res == "wɔknɔ"
        # Sandhi voicing
        assert tbenni.phonemize("brat ojca", dialect="slask", include_stress=False).startswith("brad")


class TestStressParadigms:
    """Verifies prescriptive antepenultimate stress vs colloquial penultimate."""

    def test_prescriptive_stress(self):
        # fizyka -> [ˈfʲizɨka] (3rd from end)
        assert tbenni.phonemize("fizyka", stress="prescriptive") == "ˈfʲizɨka"
        # matematyka -> [matɛˈmatɨka]
        assert tbenni.phonemize("matematyka", stress="prescriptive") == "matɛˈmatɨka"
        # byliśmy -> [ˈbɨlʲiɕmɨ]
        assert tbenni.phonemize("byliśmy", stress="prescriptive") == "ˈbɨlʲiɕmɨ"
        # zrobilibyśmy -> [zrɔˈbʲilʲibɨɕmɨ] (4th from end: zro-BI-li-byś-my)
        assert "ˈbʲi" in tbenni.phonemize("zrobilibyśmy", stress="prescriptive")

    def test_colloquial_stress(self):
        # regularized penultimate
        assert tbenni.phonemize("fizyka", stress="colloquial") == "fʲiˈzɨka"


class TestLoanwordOverrides:
    """Verifies English borrowings and Anglicisms."""

    def test_common_loanwords(self):
        assert tbenni.phonemize("weekend", include_stress=False) == "vʲikɛnt"
        assert tbenni.phonemize("manager", include_stress=False) == "mɛnɛd͡ʐɛr"
        assert tbenni.phonemize("software", include_stress=False) == "sɔftvɛr"
        assert tbenni.phonemize("facebook", include_stress=False) == "fɛjsbuk"


class TestOutputFormats:
    """Verifies multi-alphabet representations."""

    def test_benni_as(self):
        assert tbenni.phonemize("czysta", format="benni", include_stress=False) == "čysta"
        assert tbenni.phonemize("chleb", format="benni", include_stress=False) == "xlep"
        assert tbenni.phonemize("wąs", format="benni", include_stress=False) == "vǫs"

    def test_sampa(self):
        assert tbenni.phonemize("czysta", format="sampa", include_stress=False) == "tS1sta"
        assert tbenni.phonemize("chleb", format="sampa", include_stress=False) == "xlep"
