/// Bidirectional type.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct Type(pub u8);

impl Type {
    /// Right to left Arabic.
    pub const AL: Self = Self(0);
    /// Arabic number.
    pub const AN: Self = Self(1);
    /// Paragraph separator.
    pub const B: Self = Self(2);
    /// Boundary neutral.
    pub const BN: Self = Self(3);
    /// Common number separator.
    pub const CS: Self = Self(4);
    /// European number.
    pub const EN: Self = Self(5);
    /// European number separator.
    pub const ES: Self = Self(6);
    /// European number terminator.
    pub const ET: Self = Self(7);
    /// Control: First strong isolate.
    pub const FSI: Self = Self(8);
    /// Left to right.
    pub const L: Self = Self(9);
    /// Control: Left to right embedding.
    pub const LRE: Self = Self(10);
    /// Control: Left to right isolate.
    pub const LRI: Self = Self(11);
    /// Control: Left to right override.
    pub const LRO: Self = Self(12);
    /// Non-spacing mark.
    pub const NSM: Self = Self(13);
    /// Other neutral.
    pub const ON: Self = Self(14);
    /// Control: Pop directional format.
    pub const PDF: Self = Self(15);
    /// Control: Pop directional isolate.
    pub const PDI: Self = Self(16);
    /// Right to left.
    pub const R: Self = Self(17);
    /// Control: Right to left embedding.
    pub const RLE: Self = Self(18);
    /// Control: Right to left isolate.
    pub const RLI: Self = Self(19);
    /// Control: Right to left override.
    pub const RLO: Self = Self(20);
    /// Segment separator.
    pub const S: Self = Self(21);
    /// Whitespace.
    pub const WS: Self = Self(22);
}

impl Type {
    /// Returns the bidirectional type of the specified character.
    pub fn from_char(ch: char) -> Self {
        lookup_bidi_class(ch as u32)
    }

    /// Returns true if this type is ignored-- that is removed by rule X9.
    pub fn is_ignored(self) -> bool {
        super::is_removed_by_x9(self)
    }

    /// Returns true if this type should be reset to the base paragraph
    /// level when it occurs at the end of a line.
    pub fn is_resettable(self) -> bool {
        self.mask() & super::RESETTABLE_MASK != 0
    }

    pub(super) const fn mask(self) -> u32 {
        1 << self.0
    }
}

pub mod bidi_classes {
    use super::Type;
    pub const AL: Type = Type(0);
    pub const AN: Type = Type(1);
    pub const B: Type = Type(2);
    pub const BN: Type = Type(3);
    pub const CS: Type = Type(4);
    pub const EN: Type = Type(5);
    pub const ES: Type = Type(6);
    pub const ET: Type = Type(7);
    pub const FSI: Type = Type(8);
    pub const L: Type = Type(9);
    pub const LRE: Type = Type(10);
    pub const LRI: Type = Type(11);
    pub const LRO: Type = Type(12);
    pub const NSM: Type = Type(13);
    pub const ON: Type = Type(14);
    pub const PDF: Type = Type(15);
    pub const PDI: Type = Type(16);
    pub const R: Type = Type(17);
    pub const RLE: Type = Type(18);
    pub const RLI: Type = Type(19);
    pub const RLO: Type = Type(20);
    pub const S: Type = Type(21);
    pub const WS: Type = Type(22);
}

#[rustfmt::skip]
const BIDI_CLASS_RANGES: &[(u32, u32)] = &[
    (0,9),(9,10),(10,11),(11,12),(12,13),(13,14),(14,28),(28,31),(31,32),(32,33),(33,35),(35,38),(38,43),(43,44),(44,45),(45,46),
    (46,48),(48,58),(58,59),(59,65),(91,97),(123,127),(127,133),(133,134),(134,160),(160,161),(161,162),(162,166),(166,170),(171,173),(173,174),(174,176),
    (176,178),(178,180),(180,181),(182,185),(185,186),(187,192),(215,216),(247,248),(697,699),(706,720),(722,736),(741,750),(751,768),(768,880),(884,886),(894,895),
    (900,902),(903,904),(1014,1015),(1155,1162),(1418,1419),(1421,1423),(1423,1424),(1425,1470),(1470,1471),(1471,1472),(1472,1473),(1473,1475),(1475,1476),(1476,1478),(1478,1479),(1479,1480),
    (1488,1515),(1519,1525),(1536,1542),(1542,1544),(1544,1545),(1545,1547),(1547,1548),(1548,1549),(1549,1550),(1550,1552),(1552,1563),(1563,1565),(1566,1611),(1611,1632),(1632,1642),(1642,1643),
    (1643,1645),(1645,1648),(1648,1649),(1649,1750),(1750,1757),(1757,1758),(1758,1759),(1759,1765),(1765,1767),(1767,1769),(1769,1770),(1770,1774),(1774,1776),(1776,1786),(1786,1806),(1807,1809),
    (1809,1810),(1810,1840),(1840,1867),(1869,1958),(1958,1969),(1969,1970),(1984,2027),(2027,2036),(2036,2038),(2038,2042),(2042,2043),(2045,2046),(2046,2070),(2070,2074),(2074,2075),(2075,2084),
    (2084,2085),(2085,2088),(2088,2089),(2089,2094),(2096,2111),(2112,2137),(2137,2140),(2142,2143),(2144,2155),(2208,2229),(2230,2248),(2259,2274),(2274,2275),(2275,2307),(2362,2363),(2364,2365),
    (2369,2377),(2381,2382),(2385,2392),(2402,2404),(2433,2434),(2492,2493),(2497,2501),(2509,2510),(2530,2532),(2546,2548),(2555,2556),(2558,2559),(2561,2563),(2620,2621),(2625,2627),(2631,2633),
    (2635,2638),(2641,2642),(2672,2674),(2677,2678),(2689,2691),(2748,2749),(2753,2758),(2759,2761),(2765,2766),(2786,2788),(2801,2802),(2810,2816),(2817,2818),(2876,2877),(2879,2880),(2881,2885),
    (2893,2894),(2901,2903),(2914,2916),(2946,2947),(3008,3009),(3021,3022),(3059,3065),(3065,3066),(3066,3067),(3072,3073),(3076,3077),(3134,3137),(3142,3145),(3146,3150),(3157,3159),(3170,3172),
    (3192,3199),(3201,3202),(3260,3261),(3276,3278),(3298,3300),(3328,3330),(3387,3389),(3393,3397),(3405,3406),(3426,3428),(3457,3458),(3530,3531),(3538,3541),(3542,3543),(3633,3634),(3636,3643),
    (3647,3648),(3655,3663),(3761,3762),(3764,3773),(3784,3790),(3864,3866),(3893,3894),(3895,3896),(3897,3898),(3898,3902),(3953,3967),(3968,3973),(3974,3976),(3981,3992),(3993,4029),(4038,4039),
    (4141,4145),(4146,4152),(4153,4155),(4157,4159),(4184,4186),(4190,4193),(4209,4213),(4226,4227),(4229,4231),(4237,4238),(4253,4254),(4957,4960),(5008,5018),(5120,5121),(5760,5761),(5787,5789),
    (5906,5909),(5938,5941),(5970,5972),(6002,6004),(6068,6070),(6071,6078),(6086,6087),(6089,6100),(6107,6108),(6109,6110),(6128,6138),(6144,6155),(6155,6158),(6158,6159),(6277,6279),(6313,6314),
    (6432,6435),(6439,6441),(6450,6451),(6457,6460),(6464,6465),(6468,6470),(6622,6656),(6679,6681),(6683,6684),(6742,6743),(6744,6751),(6752,6753),(6754,6755),(6757,6765),(6771,6781),(6783,6784),
    (6832,6849),(6912,6916),(6964,6965),(6966,6971),(6972,6973),(6978,6979),(7019,7028),(7040,7042),(7074,7078),(7080,7082),(7083,7086),(7142,7143),(7144,7146),(7149,7150),(7151,7154),(7212,7220),
    (7222,7224),(7376,7379),(7380,7393),(7394,7401),(7405,7406),(7412,7413),(7416,7418),(7616,7674),(7675,7680),(8125,8126),(8127,8130),(8141,8144),(8157,8160),(8173,8176),(8189,8191),(8192,8203),
    (8203,8206),(8207,8208),(8208,8232),(8232,8233),(8233,8234),(8234,8235),(8235,8236),(8236,8237),(8237,8238),(8238,8239),(8239,8240),(8240,8245),(8245,8260),(8260,8261),(8261,8287),(8287,8288),
    (8288,8293),(8294,8295),(8295,8296),(8296,8297),(8297,8298),(8298,8304),(8304,8305),(8308,8314),(8314,8316),(8316,8319),(8320,8330),(8330,8332),(8332,8335),(8352,8384),(8400,8433),(8448,8450),
    (8451,8455),(8456,8458),(8468,8469),(8470,8473),(8478,8484),(8485,8486),(8487,8488),(8489,8490),(8494,8495),(8506,8508),(8512,8517),(8522,8526),(8528,8544),(8585,8588),(8592,8722),(8722,8723),
    (8723,8724),(8724,9014),(9083,9109),(9110,9255),(9280,9291),(9312,9352),(9352,9372),(9450,9900),(9901,10240),(10496,11124),(11126,11158),(11159,11264),(11493,11499),(11503,11506),(11513,11520),(11647,11648),
    (11744,11776),(11776,11859),(11904,11930),(11931,12020),(12032,12246),(12272,12284),(12288,12289),(12289,12293),(12296,12321),(12330,12334),(12336,12337),(12342,12344),(12349,12352),(12441,12443),(12443,12445),(12448,12449),
    (12539,12540),(12736,12772),(12829,12831),(12880,12896),(12924,12927),(12977,12992),(13004,13008),(13175,13179),(13278,13280),(13311,13312),(19904,19968),(42128,42183),(42509,42512),(42607,42611),(42611,42612),(42612,42622),
    (42622,42624),(42654,42656),(42736,42738),(42752,42786),(42888,42889),(43010,43011),(43014,43015),(43019,43020),(43045,43047),(43048,43052),(43052,43053),(43064,43066),(43124,43128),(43204,43206),(43232,43250),(43263,43264),
    (43302,43310),(43335,43346),(43392,43395),(43443,43444),(43446,43450),(43452,43454),(43493,43494),(43561,43567),(43569,43571),(43573,43575),(43587,43588),(43596,43597),(43644,43645),(43696,43697),(43698,43701),(43703,43705),
    (43710,43712),(43713,43714),(43756,43758),(43766,43767),(43882,43884),(44005,44006),(44008,44009),(44013,44014),(64285,64286),(64286,64287),(64287,64297),(64297,64298),(64298,64311),(64312,64317),(64318,64319),(64320,64322),
    (64323,64325),(64326,64336),(64336,64450),(64467,64830),(64830,64832),(64848,64912),(64914,64968),(65008,65021),(65021,65022),(65024,65040),(65040,65050),(65056,65072),(65072,65104),(65104,65105),(65105,65106),(65106,65107),
    (65108,65109),(65109,65110),(65110,65119),(65119,65120),(65120,65122),(65122,65124),(65124,65127),(65128,65129),(65129,65131),(65131,65132),(65136,65141),(65142,65277),(65279,65280),(65281,65283),(65283,65286),(65286,65291),
    (65291,65292),(65292,65293),(65293,65294),(65294,65296),(65296,65306),(65306,65307),(65307,65313),(65339,65345),(65371,65382),(65504,65506),(65506,65509),(65509,65511),(65512,65519),(65529,65534),(65793,65794),(65856,65933),
    (65936,65949),(65952,65953),(66045,66046),(66272,66273),(66273,66300),(66422,66427),(67584,67590),(67592,67593),(67594,67638),(67639,67641),(67644,67645),(67647,67670),(67671,67743),(67751,67760),(67808,67827),(67828,67830),
    (67835,67868),(67871,67872),(67872,67898),(67903,67904),(67968,68024),(68028,68048),(68050,68097),(68097,68100),(68101,68103),(68108,68112),(68112,68116),(68117,68120),(68121,68150),(68152,68155),(68159,68160),(68160,68169),
    (68176,68185),(68192,68256),(68288,68325),(68325,68327),(68331,68343),(68352,68406),(68409,68416),(68416,68438),(68440,68467),(68472,68498),(68505,68509),(68521,68528),(68608,68681),(68736,68787),(68800,68851),(68858,68864),
    (68864,68900),(68900,68904),(68912,68922),(69216,69247),(69248,69290),(69291,69293),(69293,69294),(69296,69298),(69376,69416),(69424,69446),(69446,69457),(69457,69466),(69552,69580),(69600,69623),(69633,69634),(69688,69703),
    (69714,69734),(69759,69762),(69811,69815),(69817,69819),(69888,69891),(69927,69932),(69933,69941),(70003,70004),(70016,70018),(70070,70079),(70089,70093),(70095,70096),(70191,70194),(70196,70197),(70198,70200),(70206,70207),
    (70367,70368),(70371,70379),(70400,70402),(70459,70461),(70464,70465),(70502,70509),(70512,70517),(70712,70720),(70722,70725),(70726,70727),(70750,70751),(70835,70841),(70842,70843),(70847,70849),(70850,70852),(71090,71094),
    (71100,71102),(71103,71105),(71132,71134),(71219,71227),(71229,71230),(71231,71233),(71264,71277),(71339,71340),(71341,71342),(71344,71350),(71351,71352),(71453,71456),(71458,71462),(71463,71468),(71727,71736),(71737,71739),
    (71995,71997),(71998,71999),(72003,72004),(72148,72152),(72154,72156),(72160,72161),(72193,72199),(72201,72203),(72243,72249),(72251,72255),(72263,72264),(72273,72279),(72281,72284),(72330,72343),(72344,72346),(72752,72759),
    (72760,72766),(72850,72872),(72874,72881),(72882,72884),(72885,72887),(73009,73015),(73018,73019),(73020,73022),(73023,73030),(73031,73032),(73104,73106),(73109,73110),(73111,73112),(73459,73461),(73685,73693),(73693,73697),
    (73697,73714),(92912,92917),(92976,92983),(94031,94032),(94095,94099),(94178,94179),(94180,94181),(113821,113823),(113824,113828),(119143,119146),(119155,119163),(119163,119171),(119173,119180),(119210,119214),(119296,119362),(119362,119365),
    (119365,119366),(119552,119639),(120539,120540),(120597,120598),(120655,120656),(120713,120714),(120771,120772),(120782,120832),(121344,121399),(121403,121453),(121461,121462),(121476,121477),(121499,121504),(121505,121520),(122880,122887),(122888,122905),
    (122907,122914),(122915,122917),(122918,122923),(123184,123191),(123628,123632),(123647,123648),(124928,125125),(125127,125136),(125136,125143),(125184,125252),(125252,125259),(125259,125260),(125264,125274),(125278,125280),(126065,126133),(126209,126270),
    (126464,126468),(126469,126496),(126497,126499),(126500,126501),(126503,126504),(126505,126515),(126516,126520),(126521,126522),(126523,126524),(126530,126531),(126535,126536),(126537,126538),(126539,126540),(126541,126544),(126545,126547),(126548,126549),
    (126551,126552),(126553,126554),(126555,126556),(126557,126558),(126559,126560),(126561,126563),(126564,126565),(126567,126571),(126572,126579),(126580,126584),(126585,126589),(126590,126591),(126592,126602),(126603,126620),(126625,126628),(126629,126634),
    (126635,126652),(126704,126706),(126976,127020),(127024,127124),(127136,127151),(127153,127168),(127169,127184),(127185,127222),(127232,127243),(127243,127248),(127279,127280),(127338,127344),(127405,127406),(127584,127590),(127744,128728),(128736,128749),
    (128752,128765),(128768,128884),(128896,128985),(128992,129004),(129024,129036),(129040,129096),(129104,129114),(129120,129160),(129168,129198),(129200,129202),(129280,129401),(129402,129484),(129485,129620),(129632,129646),(129648,129653),(129656,129659),
    (129664,129671),(129680,129705),(129712,129719),(129728,129731),(129744,129751),(129792,129939),(129940,129995),(130032,130042),(917505,917506),(917536,917632),(917760,918000),
];

#[rustfmt::skip]
const BIDI_CLASS_VALUES: &[Type] = &[
    Type::BN,Type::S,Type::B,Type::S,Type::WS,Type::B,Type::BN,Type::B,Type::S,Type::WS,Type::ON,Type::ET,Type::ON,Type::ES,Type::CS,Type::ES,
    Type::CS,Type::EN,Type::CS,Type::ON,Type::ON,Type::ON,Type::BN,Type::B,Type::BN,Type::CS,Type::ON,Type::ET,Type::ON,Type::ON,Type::BN,Type::ON,
    Type::ET,Type::EN,Type::ON,Type::ON,Type::EN,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::NSM,Type::ON,Type::ON,
    Type::ON,Type::ON,Type::ON,Type::NSM,Type::ON,Type::ON,Type::ET,Type::NSM,Type::R,Type::NSM,Type::R,Type::NSM,Type::R,Type::NSM,Type::R,Type::NSM,
    Type::R,Type::R,Type::AN,Type::ON,Type::AL,Type::ET,Type::AL,Type::CS,Type::AL,Type::ON,Type::NSM,Type::AL,Type::AL,Type::NSM,Type::AN,Type::ET,
    Type::AN,Type::AL,Type::NSM,Type::AL,Type::NSM,Type::AN,Type::ON,Type::NSM,Type::AL,Type::NSM,Type::ON,Type::NSM,Type::AL,Type::EN,Type::AL,Type::AL,
    Type::NSM,Type::AL,Type::NSM,Type::AL,Type::NSM,Type::AL,Type::R,Type::NSM,Type::R,Type::ON,Type::R,Type::NSM,Type::R,Type::NSM,Type::R,Type::NSM,
    Type::R,Type::NSM,Type::R,Type::NSM,Type::R,Type::R,Type::NSM,Type::R,Type::AL,Type::AL,Type::AL,Type::NSM,Type::AN,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ET,Type::ET,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ET,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::ET,Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::ET,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::ON,Type::WS,Type::ON,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ET,Type::NSM,Type::ON,Type::ON,Type::NSM,Type::BN,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::ON,Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::WS,
    Type::BN,Type::R,Type::ON,Type::WS,Type::B,Type::LRE,Type::RLE,Type::PDF,Type::LRO,Type::RLO,Type::CS,Type::ET,Type::ON,Type::CS,Type::ON,Type::WS,
    Type::BN,Type::LRI,Type::RLI,Type::FSI,Type::PDI,Type::BN,Type::EN,Type::EN,Type::ES,Type::ON,Type::EN,Type::ES,Type::ON,Type::ET,Type::NSM,Type::ON,
    Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ET,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ES,
    Type::ET,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::EN,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::NSM,Type::ON,Type::NSM,
    Type::NSM,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::WS,Type::ON,Type::ON,Type::NSM,Type::ON,Type::ON,Type::ON,Type::NSM,Type::ON,Type::ON,
    Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::NSM,Type::ON,Type::NSM,
    Type::ON,Type::NSM,Type::NSM,Type::ON,Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::NSM,Type::ET,Type::ON,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::R,Type::NSM,Type::R,Type::ES,Type::R,Type::R,Type::R,Type::R,
    Type::R,Type::R,Type::AL,Type::AL,Type::ON,Type::AL,Type::AL,Type::AL,Type::ON,Type::NSM,Type::ON,Type::NSM,Type::ON,Type::CS,Type::ON,Type::CS,
    Type::ON,Type::CS,Type::ON,Type::ET,Type::ON,Type::ES,Type::ON,Type::ON,Type::ET,Type::ON,Type::AL,Type::AL,Type::BN,Type::ON,Type::ET,Type::ON,
    Type::ES,Type::CS,Type::ES,Type::CS,Type::EN,Type::CS,Type::ON,Type::ON,Type::ON,Type::ET,Type::ON,Type::ET,Type::ON,Type::ON,Type::ON,Type::ON,
    Type::ON,Type::ON,Type::NSM,Type::NSM,Type::EN,Type::NSM,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,
    Type::R,Type::ON,Type::R,Type::R,Type::R,Type::R,Type::R,Type::NSM,Type::NSM,Type::NSM,Type::R,Type::R,Type::R,Type::NSM,Type::NSM,Type::R,
    Type::R,Type::R,Type::R,Type::NSM,Type::R,Type::R,Type::ON,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,Type::R,
    Type::AL,Type::NSM,Type::AN,Type::AN,Type::R,Type::NSM,Type::R,Type::R,Type::R,Type::AL,Type::NSM,Type::AL,Type::R,Type::R,Type::NSM,Type::NSM,
    Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::ET,
    Type::ON,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::NSM,Type::NSM,Type::BN,Type::NSM,Type::BN,Type::NSM,Type::NSM,Type::NSM,Type::ON,Type::NSM,
    Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::EN,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,
    Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::NSM,Type::ET,Type::R,Type::R,Type::NSM,Type::R,Type::NSM,Type::R,Type::R,Type::R,Type::AL,Type::AL,
    Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,
    Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,Type::AL,
    Type::AL,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::EN,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,
    Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,
    Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::ON,Type::EN,Type::BN,Type::BN,Type::NSM,
];

fn lookup_bidi_class(codepoint: u32) -> Type {
    let mut lo = 0;
    let mut hi = 747;
    while lo < hi {
        let mid = (lo + hi) / 2;
        let range = &BIDI_CLASS_RANGES[mid];
        if codepoint < range.0 {
            hi = mid;
        } else if codepoint >= range.1 {
            lo = mid + 1;
        } else {
            return BIDI_CLASS_VALUES[mid];
        }
    }
    Type::L
}

#[rustfmt::skip]
const BRACKET_PAIRS: &[(u32, u32)] = &[
    (40, 41), (91, 93), (123, 125), (3898, 3899), (3900, 3901), (5787, 5788), (8261, 8262), (8317, 8318), (8333, 8334), (8968, 8969), (8970, 8971), (9001, 9002), (10088, 10089), (10090, 10091), (10092, 10093), (10094, 10095), 
    (10096, 10097), (10098, 10099), (10100, 10101), (10181, 10182), (10214, 10215), (10216, 10217), (10218, 10219), (10220, 10221), (10222, 10223), (10627, 10628), (10629, 10630), (10631, 10632), (10633, 10634), (10635, 10636), (10637, 10640), (10639, 10638), 
    (10641, 10642), (10643, 10644), (10645, 10646), (10647, 10648), (10712, 10713), (10714, 10715), (10748, 10749), (11810, 11811), (11812, 11813), (11814, 11815), (11816, 11817), (12296, 12297), (12298, 12299), (12300, 12301), (12302, 12303), (12304, 12305), 
    (12308, 12309), (12310, 12311), (12312, 12313), (12314, 12315), (65113, 65114), (65115, 65116), (65117, 65118), (65288, 65289), (65339, 65341), (65371, 65373), (65375, 65376), (65378, 65379), 
];

fn opening_bracket(close: char) -> Option<char> {
    let c = close as u32;
    if let Ok(idx) = BRACKET_PAIRS.binary_search_by(|x| x.1.cmp(&c)) {
        return Some(unsafe { core::char::from_u32_unchecked(BRACKET_PAIRS[idx].0) });
    }
    None
}

fn closing_bracket(open: char) -> Option<char> {
    let c = open as u32;
    if let Ok(idx) = BRACKET_PAIRS.binary_search_by(|x| x.0.cmp(&c)) {
        return Some(unsafe { core::char::from_u32_unchecked(BRACKET_PAIRS[idx].1) });
    }
    None
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BracketType {
    None,
    Open(char),
    Close(char),
}

impl BracketType {
    pub fn from_char(ch: char) -> Self {
        if let Some(close) = closing_bracket(ch) {
            Self::Open(close)
        } else if let Some(open) = opening_bracket(ch) {
            Self::Close(open)
        } else {
            Self::None
        }
    }
}
