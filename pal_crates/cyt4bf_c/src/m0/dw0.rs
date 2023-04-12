#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - Active descriptor control"]
    pub act_descr_ctl: ACT_DESCR_CTL,
    #[doc = "0x24 - Active descriptor source"]
    pub act_descr_src: ACT_DESCR_SRC,
    #[doc = "0x28 - Active descriptor destination"]
    pub act_descr_dst: ACT_DESCR_DST,
    _reserved5: [u8; 0x04],
    #[doc = "0x30 - Active descriptor X loop control"]
    pub act_descr_x_ctl: ACT_DESCR_X_CTL,
    #[doc = "0x34 - Active descriptor Y loop control"]
    pub act_descr_y_ctl: ACT_DESCR_Y_CTL,
    #[doc = "0x38 - Active descriptor next pointer"]
    pub act_descr_next_ptr: ACT_DESCR_NEXT_PTR,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - Active source"]
    pub act_src: ACT_SRC,
    #[doc = "0x44 - Active destination"]
    pub act_dst: ACT_DST,
    _reserved10: [u8; 0x38],
    #[doc = "0x80 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved11: [u8; 0x7c],
    #[doc = "0x100 - CRC control"]
    pub crc_ctl: CRC_CTL,
    _reserved12: [u8; 0x0c],
    #[doc = "0x110 - CRC data control"]
    pub crc_data_ctl: CRC_DATA_CTL,
    _reserved13: [u8; 0x0c],
    #[doc = "0x120 - CRC polynomial control"]
    pub crc_pol_ctl: CRC_POL_CTL,
    _reserved14: [u8; 0x0c],
    #[doc = "0x130 - CRC LFSR control"]
    pub crc_lfsr_ctl: CRC_LFSR_CTL,
    _reserved15: [u8; 0x0c],
    #[doc = "0x140 - CRC remainder control"]
    pub crc_rem_ctl: CRC_REM_CTL,
    _reserved16: [u8; 0x04],
    #[doc = "0x148 - CRC remainder result"]
    pub crc_rem_result: CRC_REM_RESULT,
    _reserved17: [u8; 0x7eb4],
    #[doc = "0x8000..0x802c - DW channel structure"]
    pub ch_struct0: CH_STRUCT,
    _reserved18: [u8; 0x14],
    #[doc = "0x8040..0x806c - DW channel structure"]
    pub ch_struct1: CH_STRUCT,
    _reserved19: [u8; 0x14],
    #[doc = "0x8080..0x80ac - DW channel structure"]
    pub ch_struct2: CH_STRUCT,
    _reserved20: [u8; 0x14],
    #[doc = "0x80c0..0x80ec - DW channel structure"]
    pub ch_struct3: CH_STRUCT,
    _reserved21: [u8; 0x14],
    #[doc = "0x8100..0x812c - DW channel structure"]
    pub ch_struct4: CH_STRUCT,
    _reserved22: [u8; 0x14],
    #[doc = "0x8140..0x816c - DW channel structure"]
    pub ch_struct5: CH_STRUCT,
    _reserved23: [u8; 0x14],
    #[doc = "0x8180..0x81ac - DW channel structure"]
    pub ch_struct6: CH_STRUCT,
    _reserved24: [u8; 0x14],
    #[doc = "0x81c0..0x81ec - DW channel structure"]
    pub ch_struct7: CH_STRUCT,
    _reserved25: [u8; 0x14],
    #[doc = "0x8200..0x822c - DW channel structure"]
    pub ch_struct8: CH_STRUCT,
    _reserved26: [u8; 0x14],
    #[doc = "0x8240..0x826c - DW channel structure"]
    pub ch_struct9: CH_STRUCT,
    _reserved27: [u8; 0x14],
    #[doc = "0x8280..0x82ac - DW channel structure"]
    pub ch_struct10: CH_STRUCT,
    _reserved28: [u8; 0x14],
    #[doc = "0x82c0..0x82ec - DW channel structure"]
    pub ch_struct11: CH_STRUCT,
    _reserved29: [u8; 0x14],
    #[doc = "0x8300..0x832c - DW channel structure"]
    pub ch_struct12: CH_STRUCT,
    _reserved30: [u8; 0x14],
    #[doc = "0x8340..0x836c - DW channel structure"]
    pub ch_struct13: CH_STRUCT,
    _reserved31: [u8; 0x14],
    #[doc = "0x8380..0x83ac - DW channel structure"]
    pub ch_struct14: CH_STRUCT,
    _reserved32: [u8; 0x14],
    #[doc = "0x83c0..0x83ec - DW channel structure"]
    pub ch_struct15: CH_STRUCT,
    _reserved33: [u8; 0x14],
    #[doc = "0x8400..0x842c - DW channel structure"]
    pub ch_struct16: CH_STRUCT,
    _reserved34: [u8; 0x14],
    #[doc = "0x8440..0x846c - DW channel structure"]
    pub ch_struct17: CH_STRUCT,
    _reserved35: [u8; 0x14],
    #[doc = "0x8480..0x84ac - DW channel structure"]
    pub ch_struct18: CH_STRUCT,
    _reserved36: [u8; 0x14],
    #[doc = "0x84c0..0x84ec - DW channel structure"]
    pub ch_struct19: CH_STRUCT,
    _reserved37: [u8; 0x14],
    #[doc = "0x8500..0x852c - DW channel structure"]
    pub ch_struct20: CH_STRUCT,
    _reserved38: [u8; 0x14],
    #[doc = "0x8540..0x856c - DW channel structure"]
    pub ch_struct21: CH_STRUCT,
    _reserved39: [u8; 0x14],
    #[doc = "0x8580..0x85ac - DW channel structure"]
    pub ch_struct22: CH_STRUCT,
    _reserved40: [u8; 0x14],
    #[doc = "0x85c0..0x85ec - DW channel structure"]
    pub ch_struct23: CH_STRUCT,
    _reserved41: [u8; 0x14],
    #[doc = "0x8600..0x862c - DW channel structure"]
    pub ch_struct24: CH_STRUCT,
    _reserved42: [u8; 0x14],
    #[doc = "0x8640..0x866c - DW channel structure"]
    pub ch_struct25: CH_STRUCT,
    _reserved43: [u8; 0x14],
    #[doc = "0x8680..0x86ac - DW channel structure"]
    pub ch_struct26: CH_STRUCT,
    _reserved44: [u8; 0x14],
    #[doc = "0x86c0..0x86ec - DW channel structure"]
    pub ch_struct27: CH_STRUCT,
    _reserved45: [u8; 0x14],
    #[doc = "0x8700..0x872c - DW channel structure"]
    pub ch_struct28: CH_STRUCT,
    _reserved46: [u8; 0x14],
    #[doc = "0x8740..0x876c - DW channel structure"]
    pub ch_struct29: CH_STRUCT,
    _reserved47: [u8; 0x14],
    #[doc = "0x8780..0x87ac - DW channel structure"]
    pub ch_struct30: CH_STRUCT,
    _reserved48: [u8; 0x14],
    #[doc = "0x87c0..0x87ec - DW channel structure"]
    pub ch_struct31: CH_STRUCT,
    _reserved49: [u8; 0x14],
    #[doc = "0x8800..0x882c - DW channel structure"]
    pub ch_struct32: CH_STRUCT,
    _reserved50: [u8; 0x14],
    #[doc = "0x8840..0x886c - DW channel structure"]
    pub ch_struct33: CH_STRUCT,
    _reserved51: [u8; 0x14],
    #[doc = "0x8880..0x88ac - DW channel structure"]
    pub ch_struct34: CH_STRUCT,
    _reserved52: [u8; 0x14],
    #[doc = "0x88c0..0x88ec - DW channel structure"]
    pub ch_struct35: CH_STRUCT,
    _reserved53: [u8; 0x14],
    #[doc = "0x8900..0x892c - DW channel structure"]
    pub ch_struct36: CH_STRUCT,
    _reserved54: [u8; 0x14],
    #[doc = "0x8940..0x896c - DW channel structure"]
    pub ch_struct37: CH_STRUCT,
    _reserved55: [u8; 0x14],
    #[doc = "0x8980..0x89ac - DW channel structure"]
    pub ch_struct38: CH_STRUCT,
    _reserved56: [u8; 0x14],
    #[doc = "0x89c0..0x89ec - DW channel structure"]
    pub ch_struct39: CH_STRUCT,
    _reserved57: [u8; 0x14],
    #[doc = "0x8a00..0x8a2c - DW channel structure"]
    pub ch_struct40: CH_STRUCT,
    _reserved58: [u8; 0x14],
    #[doc = "0x8a40..0x8a6c - DW channel structure"]
    pub ch_struct41: CH_STRUCT,
    _reserved59: [u8; 0x14],
    #[doc = "0x8a80..0x8aac - DW channel structure"]
    pub ch_struct42: CH_STRUCT,
    _reserved60: [u8; 0x14],
    #[doc = "0x8ac0..0x8aec - DW channel structure"]
    pub ch_struct43: CH_STRUCT,
    _reserved61: [u8; 0x14],
    #[doc = "0x8b00..0x8b2c - DW channel structure"]
    pub ch_struct44: CH_STRUCT,
    _reserved62: [u8; 0x14],
    #[doc = "0x8b40..0x8b6c - DW channel structure"]
    pub ch_struct45: CH_STRUCT,
    _reserved63: [u8; 0x14],
    #[doc = "0x8b80..0x8bac - DW channel structure"]
    pub ch_struct46: CH_STRUCT,
    _reserved64: [u8; 0x14],
    #[doc = "0x8bc0..0x8bec - DW channel structure"]
    pub ch_struct47: CH_STRUCT,
    _reserved65: [u8; 0x14],
    #[doc = "0x8c00..0x8c2c - DW channel structure"]
    pub ch_struct48: CH_STRUCT,
    _reserved66: [u8; 0x14],
    #[doc = "0x8c40..0x8c6c - DW channel structure"]
    pub ch_struct49: CH_STRUCT,
    _reserved67: [u8; 0x14],
    #[doc = "0x8c80..0x8cac - DW channel structure"]
    pub ch_struct50: CH_STRUCT,
    _reserved68: [u8; 0x14],
    #[doc = "0x8cc0..0x8cec - DW channel structure"]
    pub ch_struct51: CH_STRUCT,
    _reserved69: [u8; 0x14],
    #[doc = "0x8d00..0x8d2c - DW channel structure"]
    pub ch_struct52: CH_STRUCT,
    _reserved70: [u8; 0x14],
    #[doc = "0x8d40..0x8d6c - DW channel structure"]
    pub ch_struct53: CH_STRUCT,
    _reserved71: [u8; 0x14],
    #[doc = "0x8d80..0x8dac - DW channel structure"]
    pub ch_struct54: CH_STRUCT,
    _reserved72: [u8; 0x14],
    #[doc = "0x8dc0..0x8dec - DW channel structure"]
    pub ch_struct55: CH_STRUCT,
    _reserved73: [u8; 0x14],
    #[doc = "0x8e00..0x8e2c - DW channel structure"]
    pub ch_struct56: CH_STRUCT,
    _reserved74: [u8; 0x14],
    #[doc = "0x8e40..0x8e6c - DW channel structure"]
    pub ch_struct57: CH_STRUCT,
    _reserved75: [u8; 0x14],
    #[doc = "0x8e80..0x8eac - DW channel structure"]
    pub ch_struct58: CH_STRUCT,
    _reserved76: [u8; 0x14],
    #[doc = "0x8ec0..0x8eec - DW channel structure"]
    pub ch_struct59: CH_STRUCT,
    _reserved77: [u8; 0x14],
    #[doc = "0x8f00..0x8f2c - DW channel structure"]
    pub ch_struct60: CH_STRUCT,
    _reserved78: [u8; 0x14],
    #[doc = "0x8f40..0x8f6c - DW channel structure"]
    pub ch_struct61: CH_STRUCT,
    _reserved79: [u8; 0x14],
    #[doc = "0x8f80..0x8fac - DW channel structure"]
    pub ch_struct62: CH_STRUCT,
    _reserved80: [u8; 0x14],
    #[doc = "0x8fc0..0x8fec - DW channel structure"]
    pub ch_struct63: CH_STRUCT,
    _reserved81: [u8; 0x14],
    #[doc = "0x9000..0x902c - DW channel structure"]
    pub ch_struct64: CH_STRUCT,
    _reserved82: [u8; 0x14],
    #[doc = "0x9040..0x906c - DW channel structure"]
    pub ch_struct65: CH_STRUCT,
    _reserved83: [u8; 0x14],
    #[doc = "0x9080..0x90ac - DW channel structure"]
    pub ch_struct66: CH_STRUCT,
    _reserved84: [u8; 0x14],
    #[doc = "0x90c0..0x90ec - DW channel structure"]
    pub ch_struct67: CH_STRUCT,
    _reserved85: [u8; 0x14],
    #[doc = "0x9100..0x912c - DW channel structure"]
    pub ch_struct68: CH_STRUCT,
    _reserved86: [u8; 0x14],
    #[doc = "0x9140..0x916c - DW channel structure"]
    pub ch_struct69: CH_STRUCT,
    _reserved87: [u8; 0x14],
    #[doc = "0x9180..0x91ac - DW channel structure"]
    pub ch_struct70: CH_STRUCT,
    _reserved88: [u8; 0x14],
    #[doc = "0x91c0..0x91ec - DW channel structure"]
    pub ch_struct71: CH_STRUCT,
    _reserved89: [u8; 0x14],
    #[doc = "0x9200..0x922c - DW channel structure"]
    pub ch_struct72: CH_STRUCT,
    _reserved90: [u8; 0x14],
    #[doc = "0x9240..0x926c - DW channel structure"]
    pub ch_struct73: CH_STRUCT,
    _reserved91: [u8; 0x14],
    #[doc = "0x9280..0x92ac - DW channel structure"]
    pub ch_struct74: CH_STRUCT,
    _reserved92: [u8; 0x14],
    #[doc = "0x92c0..0x92ec - DW channel structure"]
    pub ch_struct75: CH_STRUCT,
    _reserved93: [u8; 0x14],
    #[doc = "0x9300..0x932c - DW channel structure"]
    pub ch_struct76: CH_STRUCT,
    _reserved94: [u8; 0x14],
    #[doc = "0x9340..0x936c - DW channel structure"]
    pub ch_struct77: CH_STRUCT,
    _reserved95: [u8; 0x14],
    #[doc = "0x9380..0x93ac - DW channel structure"]
    pub ch_struct78: CH_STRUCT,
    _reserved96: [u8; 0x14],
    #[doc = "0x93c0..0x93ec - DW channel structure"]
    pub ch_struct79: CH_STRUCT,
    _reserved97: [u8; 0x14],
    #[doc = "0x9400..0x942c - DW channel structure"]
    pub ch_struct80: CH_STRUCT,
    _reserved98: [u8; 0x14],
    #[doc = "0x9440..0x946c - DW channel structure"]
    pub ch_struct81: CH_STRUCT,
    _reserved99: [u8; 0x14],
    #[doc = "0x9480..0x94ac - DW channel structure"]
    pub ch_struct82: CH_STRUCT,
    _reserved100: [u8; 0x14],
    #[doc = "0x94c0..0x94ec - DW channel structure"]
    pub ch_struct83: CH_STRUCT,
    _reserved101: [u8; 0x14],
    #[doc = "0x9500..0x952c - DW channel structure"]
    pub ch_struct84: CH_STRUCT,
    _reserved102: [u8; 0x14],
    #[doc = "0x9540..0x956c - DW channel structure"]
    pub ch_struct85: CH_STRUCT,
    _reserved103: [u8; 0x14],
    #[doc = "0x9580..0x95ac - DW channel structure"]
    pub ch_struct86: CH_STRUCT,
    _reserved104: [u8; 0x14],
    #[doc = "0x95c0..0x95ec - DW channel structure"]
    pub ch_struct87: CH_STRUCT,
    _reserved105: [u8; 0x14],
    #[doc = "0x9600..0x962c - DW channel structure"]
    pub ch_struct88: CH_STRUCT,
    _reserved106: [u8; 0x14],
    #[doc = "0x9640..0x966c - DW channel structure"]
    pub ch_struct89: CH_STRUCT,
    _reserved107: [u8; 0x14],
    #[doc = "0x9680..0x96ac - DW channel structure"]
    pub ch_struct90: CH_STRUCT,
    _reserved108: [u8; 0x14],
    #[doc = "0x96c0..0x96ec - DW channel structure"]
    pub ch_struct91: CH_STRUCT,
    _reserved109: [u8; 0x14],
    #[doc = "0x9700..0x972c - DW channel structure"]
    pub ch_struct92: CH_STRUCT,
    _reserved110: [u8; 0x14],
    #[doc = "0x9740..0x976c - DW channel structure"]
    pub ch_struct93: CH_STRUCT,
    _reserved111: [u8; 0x14],
    #[doc = "0x9780..0x97ac - DW channel structure"]
    pub ch_struct94: CH_STRUCT,
    _reserved112: [u8; 0x14],
    #[doc = "0x97c0..0x97ec - DW channel structure"]
    pub ch_struct95: CH_STRUCT,
    _reserved113: [u8; 0x14],
    #[doc = "0x9800..0x982c - DW channel structure"]
    pub ch_struct96: CH_STRUCT,
    _reserved114: [u8; 0x14],
    #[doc = "0x9840..0x986c - DW channel structure"]
    pub ch_struct97: CH_STRUCT,
    _reserved115: [u8; 0x14],
    #[doc = "0x9880..0x98ac - DW channel structure"]
    pub ch_struct98: CH_STRUCT,
    _reserved116: [u8; 0x14],
    #[doc = "0x98c0..0x98ec - DW channel structure"]
    pub ch_struct99: CH_STRUCT,
    _reserved117: [u8; 0x14],
    #[doc = "0x9900..0x992c - DW channel structure"]
    pub ch_struct100: CH_STRUCT,
    _reserved118: [u8; 0x14],
    #[doc = "0x9940..0x996c - DW channel structure"]
    pub ch_struct101: CH_STRUCT,
    _reserved119: [u8; 0x14],
    #[doc = "0x9980..0x99ac - DW channel structure"]
    pub ch_struct102: CH_STRUCT,
    _reserved120: [u8; 0x14],
    #[doc = "0x99c0..0x99ec - DW channel structure"]
    pub ch_struct103: CH_STRUCT,
    _reserved121: [u8; 0x14],
    #[doc = "0x9a00..0x9a2c - DW channel structure"]
    pub ch_struct104: CH_STRUCT,
    _reserved122: [u8; 0x14],
    #[doc = "0x9a40..0x9a6c - DW channel structure"]
    pub ch_struct105: CH_STRUCT,
    _reserved123: [u8; 0x14],
    #[doc = "0x9a80..0x9aac - DW channel structure"]
    pub ch_struct106: CH_STRUCT,
    _reserved124: [u8; 0x14],
    #[doc = "0x9ac0..0x9aec - DW channel structure"]
    pub ch_struct107: CH_STRUCT,
    _reserved125: [u8; 0x14],
    #[doc = "0x9b00..0x9b2c - DW channel structure"]
    pub ch_struct108: CH_STRUCT,
    _reserved126: [u8; 0x14],
    #[doc = "0x9b40..0x9b6c - DW channel structure"]
    pub ch_struct109: CH_STRUCT,
    _reserved127: [u8; 0x14],
    #[doc = "0x9b80..0x9bac - DW channel structure"]
    pub ch_struct110: CH_STRUCT,
    _reserved128: [u8; 0x14],
    #[doc = "0x9bc0..0x9bec - DW channel structure"]
    pub ch_struct111: CH_STRUCT,
    _reserved129: [u8; 0x14],
    #[doc = "0x9c00..0x9c2c - DW channel structure"]
    pub ch_struct112: CH_STRUCT,
    _reserved130: [u8; 0x14],
    #[doc = "0x9c40..0x9c6c - DW channel structure"]
    pub ch_struct113: CH_STRUCT,
    _reserved131: [u8; 0x14],
    #[doc = "0x9c80..0x9cac - DW channel structure"]
    pub ch_struct114: CH_STRUCT,
    _reserved132: [u8; 0x14],
    #[doc = "0x9cc0..0x9cec - DW channel structure"]
    pub ch_struct115: CH_STRUCT,
    _reserved133: [u8; 0x14],
    #[doc = "0x9d00..0x9d2c - DW channel structure"]
    pub ch_struct116: CH_STRUCT,
    _reserved134: [u8; 0x14],
    #[doc = "0x9d40..0x9d6c - DW channel structure"]
    pub ch_struct117: CH_STRUCT,
    _reserved135: [u8; 0x14],
    #[doc = "0x9d80..0x9dac - DW channel structure"]
    pub ch_struct118: CH_STRUCT,
    _reserved136: [u8; 0x14],
    #[doc = "0x9dc0..0x9dec - DW channel structure"]
    pub ch_struct119: CH_STRUCT,
    _reserved137: [u8; 0x14],
    #[doc = "0x9e00..0x9e2c - DW channel structure"]
    pub ch_struct120: CH_STRUCT,
    _reserved138: [u8; 0x14],
    #[doc = "0x9e40..0x9e6c - DW channel structure"]
    pub ch_struct121: CH_STRUCT,
    _reserved139: [u8; 0x14],
    #[doc = "0x9e80..0x9eac - DW channel structure"]
    pub ch_struct122: CH_STRUCT,
    _reserved140: [u8; 0x14],
    #[doc = "0x9ec0..0x9eec - DW channel structure"]
    pub ch_struct123: CH_STRUCT,
    _reserved141: [u8; 0x14],
    #[doc = "0x9f00..0x9f2c - DW channel structure"]
    pub ch_struct124: CH_STRUCT,
    _reserved142: [u8; 0x14],
    #[doc = "0x9f40..0x9f6c - DW channel structure"]
    pub ch_struct125: CH_STRUCT,
    _reserved143: [u8; 0x14],
    #[doc = "0x9f80..0x9fac - DW channel structure"]
    pub ch_struct126: CH_STRUCT,
    _reserved144: [u8; 0x14],
    #[doc = "0x9fc0..0x9fec - DW channel structure"]
    pub ch_struct127: CH_STRUCT,
    _reserved145: [u8; 0x14],
    #[doc = "0xa000..0xa02c - DW channel structure"]
    pub ch_struct128: CH_STRUCT,
    _reserved146: [u8; 0x14],
    #[doc = "0xa040..0xa06c - DW channel structure"]
    pub ch_struct129: CH_STRUCT,
    _reserved147: [u8; 0x14],
    #[doc = "0xa080..0xa0ac - DW channel structure"]
    pub ch_struct130: CH_STRUCT,
    _reserved148: [u8; 0x14],
    #[doc = "0xa0c0..0xa0ec - DW channel structure"]
    pub ch_struct131: CH_STRUCT,
    _reserved149: [u8; 0x14],
    #[doc = "0xa100..0xa12c - DW channel structure"]
    pub ch_struct132: CH_STRUCT,
    _reserved150: [u8; 0x14],
    #[doc = "0xa140..0xa16c - DW channel structure"]
    pub ch_struct133: CH_STRUCT,
    _reserved151: [u8; 0x14],
    #[doc = "0xa180..0xa1ac - DW channel structure"]
    pub ch_struct134: CH_STRUCT,
    _reserved152: [u8; 0x14],
    #[doc = "0xa1c0..0xa1ec - DW channel structure"]
    pub ch_struct135: CH_STRUCT,
    _reserved153: [u8; 0x14],
    #[doc = "0xa200..0xa22c - DW channel structure"]
    pub ch_struct136: CH_STRUCT,
    _reserved154: [u8; 0x14],
    #[doc = "0xa240..0xa26c - DW channel structure"]
    pub ch_struct137: CH_STRUCT,
    _reserved155: [u8; 0x14],
    #[doc = "0xa280..0xa2ac - DW channel structure"]
    pub ch_struct138: CH_STRUCT,
    _reserved156: [u8; 0x14],
    #[doc = "0xa2c0..0xa2ec - DW channel structure"]
    pub ch_struct139: CH_STRUCT,
    _reserved157: [u8; 0x14],
    #[doc = "0xa300..0xa32c - DW channel structure"]
    pub ch_struct140: CH_STRUCT,
    _reserved158: [u8; 0x14],
    #[doc = "0xa340..0xa36c - DW channel structure"]
    pub ch_struct141: CH_STRUCT,
    _reserved159: [u8; 0x14],
    #[doc = "0xa380..0xa3ac - DW channel structure"]
    pub ch_struct142: CH_STRUCT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ACT_DESCR_CTL (r) register accessor: an alias for `Reg<ACT_DESCR_CTL_SPEC>`"]
pub type ACT_DESCR_CTL = crate::Reg<act_descr_ctl::ACT_DESCR_CTL_SPEC>;
#[doc = "Active descriptor control"]
pub mod act_descr_ctl;
#[doc = "ACT_DESCR_SRC (r) register accessor: an alias for `Reg<ACT_DESCR_SRC_SPEC>`"]
pub type ACT_DESCR_SRC = crate::Reg<act_descr_src::ACT_DESCR_SRC_SPEC>;
#[doc = "Active descriptor source"]
pub mod act_descr_src;
#[doc = "ACT_DESCR_DST (r) register accessor: an alias for `Reg<ACT_DESCR_DST_SPEC>`"]
pub type ACT_DESCR_DST = crate::Reg<act_descr_dst::ACT_DESCR_DST_SPEC>;
#[doc = "Active descriptor destination"]
pub mod act_descr_dst;
#[doc = "ACT_DESCR_X_CTL (r) register accessor: an alias for `Reg<ACT_DESCR_X_CTL_SPEC>`"]
pub type ACT_DESCR_X_CTL = crate::Reg<act_descr_x_ctl::ACT_DESCR_X_CTL_SPEC>;
#[doc = "Active descriptor X loop control"]
pub mod act_descr_x_ctl;
#[doc = "ACT_DESCR_Y_CTL (r) register accessor: an alias for `Reg<ACT_DESCR_Y_CTL_SPEC>`"]
pub type ACT_DESCR_Y_CTL = crate::Reg<act_descr_y_ctl::ACT_DESCR_Y_CTL_SPEC>;
#[doc = "Active descriptor Y loop control"]
pub mod act_descr_y_ctl;
#[doc = "ACT_DESCR_NEXT_PTR (r) register accessor: an alias for `Reg<ACT_DESCR_NEXT_PTR_SPEC>`"]
pub type ACT_DESCR_NEXT_PTR = crate::Reg<act_descr_next_ptr::ACT_DESCR_NEXT_PTR_SPEC>;
#[doc = "Active descriptor next pointer"]
pub mod act_descr_next_ptr;
#[doc = "ACT_SRC (r) register accessor: an alias for `Reg<ACT_SRC_SPEC>`"]
pub type ACT_SRC = crate::Reg<act_src::ACT_SRC_SPEC>;
#[doc = "Active source"]
pub mod act_src;
#[doc = "ACT_DST (r) register accessor: an alias for `Reg<ACT_DST_SPEC>`"]
pub type ACT_DST = crate::Reg<act_dst::ACT_DST_SPEC>;
#[doc = "Active destination"]
pub mod act_dst;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "CRC_CTL (rw) register accessor: an alias for `Reg<CRC_CTL_SPEC>`"]
pub type CRC_CTL = crate::Reg<crc_ctl::CRC_CTL_SPEC>;
#[doc = "CRC control"]
pub mod crc_ctl;
#[doc = "CRC_DATA_CTL (rw) register accessor: an alias for `Reg<CRC_DATA_CTL_SPEC>`"]
pub type CRC_DATA_CTL = crate::Reg<crc_data_ctl::CRC_DATA_CTL_SPEC>;
#[doc = "CRC data control"]
pub mod crc_data_ctl;
#[doc = "CRC_POL_CTL (rw) register accessor: an alias for `Reg<CRC_POL_CTL_SPEC>`"]
pub type CRC_POL_CTL = crate::Reg<crc_pol_ctl::CRC_POL_CTL_SPEC>;
#[doc = "CRC polynomial control"]
pub mod crc_pol_ctl;
#[doc = "CRC_LFSR_CTL (rw) register accessor: an alias for `Reg<CRC_LFSR_CTL_SPEC>`"]
pub type CRC_LFSR_CTL = crate::Reg<crc_lfsr_ctl::CRC_LFSR_CTL_SPEC>;
#[doc = "CRC LFSR control"]
pub mod crc_lfsr_ctl;
#[doc = "CRC_REM_CTL (rw) register accessor: an alias for `Reg<CRC_REM_CTL_SPEC>`"]
pub type CRC_REM_CTL = crate::Reg<crc_rem_ctl::CRC_REM_CTL_SPEC>;
#[doc = "CRC remainder control"]
pub mod crc_rem_ctl;
#[doc = "CRC_REM_RESULT (r) register accessor: an alias for `Reg<CRC_REM_RESULT_SPEC>`"]
pub type CRC_REM_RESULT = crate::Reg<crc_rem_result::CRC_REM_RESULT_SPEC>;
#[doc = "CRC remainder result"]
pub mod crc_rem_result;
#[doc = "DW channel structure"]
pub use self::ch_struct::CH_STRUCT;
#[doc = r"Cluster"]
#[doc = "DW channel structure"]
pub mod ch_struct;
