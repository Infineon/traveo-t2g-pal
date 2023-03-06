#[doc = r"Register block"]
#[repr(C)]
pub struct DEC {
    #[doc = "0x00 - Image property settings."]
    pub imageproperty: IMAGEPROPERTY,
    #[doc = "0x04 - Software reset trigger."]
    pub swreset: SWRESET,
    #[doc = "0x08 - Decoding status"]
    pub decodingstatus: DECODINGSTATUS,
    #[doc = "0x0c - Quantization table number downloaded from the JPEG image."]
    pub quanttableno: QUANTTABLENO,
    #[doc = "0x10 - Huffman Table number downloaded from the JPEG image."]
    pub huffmantableno: HUFFMANTABLENO,
    #[doc = "0x14 - The DRI value downloaded from the JPEG image."]
    pub dri: DRI,
    #[doc = "0x18 - Y size downloaded from the JPEG image."]
    pub sizey: SIZEY,
    #[doc = "0x1c - X size downloaded from the JPEG image."]
    pub sizex: SIZEX,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - Decoding option settings."]
    pub decodingoption: DECODINGOPTION,
    #[doc = "0x28 - Interrupt Setting"]
    pub intr_dec_en: INTR_DEC_EN,
    #[doc = "0x2c - Interrupt status"]
    pub intr_dec: INTR_DEC,
    #[doc = "0x30 - Error code for INTR_DEC.ErrorMarker."]
    pub errorcode: ERRORCODE,
    #[doc = "0x34 - Correction Mode and Crop Decoding status. This register is initialized by the decoding start trigger (CMD.START)."]
    pub correctioncropstatus: CORRECTIONCROPSTATUS,
    #[doc = "0x38 - Suspend status. Decoding can be resumed from Suspended State by writing '1' to CMD.RESUME."]
    pub suspend: SUSPEND,
    #[doc = "0x3c - DNL value downloaded from the JPEG image. This register is initialized by the decoding start trigger (CMD.START)."]
    pub dnl: DNL,
    #[doc = "0x40 - Y crop start position"]
    pub cropstarty: CROPSTARTY,
    #[doc = "0x44 - X crop start position"]
    pub cropstartx: CROPSTARTX,
    #[doc = "0x48 - Crop size for Y direction"]
    pub cropsizey: CROPSIZEY,
    #[doc = "0x4c - Crop size for X direction"]
    pub cropsizex: CROPSIZEX,
    _reserved19: [u8; 0xb0],
    #[doc = "0x100..0x180 - Quantization Table number 0 downloaded from a JPEG image."]
    pub quanttable0: [QUANTTABLE0; 32],
    #[doc = "0x180..0x200 - Quantization Table number 1 downloaded from a JPEG image."]
    pub quanttable1: [QUANTTABLE1; 32],
    #[doc = "0x200..0x280 - Quantization Table number 2 downloaded from a JPEG image."]
    pub quanttable2: [QUANTTABLE2; 32],
    #[doc = "0x280..0x300 - Quantization Table number 3 downloaded from a JPEG image."]
    pub quanttable3: [QUANTTABLE3; 32],
    #[doc = "0x300..0x310 - Shows the code length distribution for DC Huffman Table 0. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
    pub huffmantabledc0codelength: [HUFFMANTABLEDC0CODELENGTH; 4],
    #[doc = "0x310..0x31c - Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 0."]
    pub huffmantabledc0groupno: [HUFFMANTABLEDC0GROUPNO; 3],
    _reserved25: [u8; 0x04],
    #[doc = "0x320..0x330 - Shows the code length distribution for AC Huffman Table 0. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
    pub huffmantableac0codelength: [HUFFMANTABLEAC0CODELENGTH; 4],
    #[doc = "0x330..0x3d4 - Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 0."]
    pub huffmantableac0groupno: [HUFFMANTABLEAC0GROUPNO; 41],
    _reserved27: [u8; 0x40],
    #[doc = "0x414..0x424 - Shows the code length distribution for DC Huffman Table 1. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
    pub huffmantabledc1codelength: [HUFFMANTABLEDC1CODELENGTH; 4],
    #[doc = "0x424..0x430 - Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 1."]
    pub huffmantabledc1groupno: [HUFFMANTABLEDC1GROUPNO; 3],
    _reserved29: [u8; 0x04],
    #[doc = "0x434..0x444 - Shows the code length distribution for AC Huffman Table 1. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
    pub huffmantableac1codelength: [HUFFMANTABLEAC1CODELENGTH; 4],
    #[doc = "0x444..0x4e8 - Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 1."]
    pub huffmantableac1groupno: [HUFFMANTABLEAC1GROUPNO; 41],
}
#[doc = "IMAGEPROPERTY (rw) register accessor: an alias for `Reg<IMAGEPROPERTY_SPEC>`"]
pub type IMAGEPROPERTY = crate::Reg<imageproperty::IMAGEPROPERTY_SPEC>;
#[doc = "Image property settings."]
pub mod imageproperty;
#[doc = "SWRESET (w) register accessor: an alias for `Reg<SWRESET_SPEC>`"]
pub type SWRESET = crate::Reg<swreset::SWRESET_SPEC>;
#[doc = "Software reset trigger."]
pub mod swreset;
#[doc = "DECODINGSTATUS (r) register accessor: an alias for `Reg<DECODINGSTATUS_SPEC>`"]
pub type DECODINGSTATUS = crate::Reg<decodingstatus::DECODINGSTATUS_SPEC>;
#[doc = "Decoding status"]
pub mod decodingstatus;
#[doc = "QUANTTABLENO (r) register accessor: an alias for `Reg<QUANTTABLENO_SPEC>`"]
pub type QUANTTABLENO = crate::Reg<quanttableno::QUANTTABLENO_SPEC>;
#[doc = "Quantization table number downloaded from the JPEG image."]
pub mod quanttableno;
#[doc = "HUFFMANTABLENO (r) register accessor: an alias for `Reg<HUFFMANTABLENO_SPEC>`"]
pub type HUFFMANTABLENO = crate::Reg<huffmantableno::HUFFMANTABLENO_SPEC>;
#[doc = "Huffman Table number downloaded from the JPEG image."]
pub mod huffmantableno;
#[doc = "DRI (r) register accessor: an alias for `Reg<DRI_SPEC>`"]
pub type DRI = crate::Reg<dri::DRI_SPEC>;
#[doc = "The DRI value downloaded from the JPEG image."]
pub mod dri;
#[doc = "SIZEY (r) register accessor: an alias for `Reg<SIZEY_SPEC>`"]
pub type SIZEY = crate::Reg<sizey::SIZEY_SPEC>;
#[doc = "Y size downloaded from the JPEG image."]
pub mod sizey;
#[doc = "SIZEX (r) register accessor: an alias for `Reg<SIZEX_SPEC>`"]
pub type SIZEX = crate::Reg<sizex::SIZEX_SPEC>;
#[doc = "X size downloaded from the JPEG image."]
pub mod sizex;
#[doc = "DECODINGOPTION (rw) register accessor: an alias for `Reg<DECODINGOPTION_SPEC>`"]
pub type DECODINGOPTION = crate::Reg<decodingoption::DECODINGOPTION_SPEC>;
#[doc = "Decoding option settings."]
pub mod decodingoption;
#[doc = "INTR_DEC_EN (rw) register accessor: an alias for `Reg<INTR_DEC_EN_SPEC>`"]
pub type INTR_DEC_EN = crate::Reg<intr_dec_en::INTR_DEC_EN_SPEC>;
#[doc = "Interrupt Setting"]
pub mod intr_dec_en;
#[doc = "INTR_DEC (rw) register accessor: an alias for `Reg<INTR_DEC_SPEC>`"]
pub type INTR_DEC = crate::Reg<intr_dec::INTR_DEC_SPEC>;
#[doc = "Interrupt status"]
pub mod intr_dec;
#[doc = "ERRORCODE (r) register accessor: an alias for `Reg<ERRORCODE_SPEC>`"]
pub type ERRORCODE = crate::Reg<errorcode::ERRORCODE_SPEC>;
#[doc = "Error code for INTR_DEC.ErrorMarker."]
pub mod errorcode;
#[doc = "CORRECTIONCROPSTATUS (r) register accessor: an alias for `Reg<CORRECTIONCROPSTATUS_SPEC>`"]
pub type CORRECTIONCROPSTATUS = crate::Reg<correctioncropstatus::CORRECTIONCROPSTATUS_SPEC>;
#[doc = "Correction Mode and Crop Decoding status. This register is initialized by the decoding start trigger (CMD.START)."]
pub mod correctioncropstatus;
#[doc = "SUSPEND (r) register accessor: an alias for `Reg<SUSPEND_SPEC>`"]
pub type SUSPEND = crate::Reg<suspend::SUSPEND_SPEC>;
#[doc = "Suspend status. Decoding can be resumed from Suspended State by writing '1' to CMD.RESUME."]
pub mod suspend;
#[doc = "DNL (r) register accessor: an alias for `Reg<DNL_SPEC>`"]
pub type DNL = crate::Reg<dnl::DNL_SPEC>;
#[doc = "DNL value downloaded from the JPEG image. This register is initialized by the decoding start trigger (CMD.START)."]
pub mod dnl;
#[doc = "CROPSTARTY (rw) register accessor: an alias for `Reg<CROPSTARTY_SPEC>`"]
pub type CROPSTARTY = crate::Reg<cropstarty::CROPSTARTY_SPEC>;
#[doc = "Y crop start position"]
pub mod cropstarty;
#[doc = "CROPSTARTX (rw) register accessor: an alias for `Reg<CROPSTARTX_SPEC>`"]
pub type CROPSTARTX = crate::Reg<cropstartx::CROPSTARTX_SPEC>;
#[doc = "X crop start position"]
pub mod cropstartx;
#[doc = "CROPSIZEY (rw) register accessor: an alias for `Reg<CROPSIZEY_SPEC>`"]
pub type CROPSIZEY = crate::Reg<cropsizey::CROPSIZEY_SPEC>;
#[doc = "Crop size for Y direction"]
pub mod cropsizey;
#[doc = "CROPSIZEX (rw) register accessor: an alias for `Reg<CROPSIZEX_SPEC>`"]
pub type CROPSIZEX = crate::Reg<cropsizex::CROPSIZEX_SPEC>;
#[doc = "Crop size for X direction"]
pub mod cropsizex;
#[doc = "QUANTTABLE0 (r) register accessor: an alias for `Reg<QUANTTABLE0_SPEC>`"]
pub type QUANTTABLE0 = crate::Reg<quanttable0::QUANTTABLE0_SPEC>;
#[doc = "Quantization Table number 0 downloaded from a JPEG image."]
pub mod quanttable0;
#[doc = "QUANTTABLE1 (r) register accessor: an alias for `Reg<QUANTTABLE1_SPEC>`"]
pub type QUANTTABLE1 = crate::Reg<quanttable1::QUANTTABLE1_SPEC>;
#[doc = "Quantization Table number 1 downloaded from a JPEG image."]
pub mod quanttable1;
#[doc = "QUANTTABLE2 (r) register accessor: an alias for `Reg<QUANTTABLE2_SPEC>`"]
pub type QUANTTABLE2 = crate::Reg<quanttable2::QUANTTABLE2_SPEC>;
#[doc = "Quantization Table number 2 downloaded from a JPEG image."]
pub mod quanttable2;
#[doc = "QUANTTABLE3 (r) register accessor: an alias for `Reg<QUANTTABLE3_SPEC>`"]
pub type QUANTTABLE3 = crate::Reg<quanttable3::QUANTTABLE3_SPEC>;
#[doc = "Quantization Table number 3 downloaded from a JPEG image."]
pub mod quanttable3;
#[doc = "HUFFMANTABLEDC0CODELENGTH (r) register accessor: an alias for `Reg<HUFFMANTABLEDC0CODELENGTH_SPEC>`"]
pub type HUFFMANTABLEDC0CODELENGTH =
    crate::Reg<huffmantabledc0codelength::HUFFMANTABLEDC0CODELENGTH_SPEC>;
#[doc = "Shows the code length distribution for DC Huffman Table 0. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
pub mod huffmantabledc0codelength;
#[doc = "HUFFMANTABLEDC0GROUPNO (r) register accessor: an alias for `Reg<HUFFMANTABLEDC0GROUPNO_SPEC>`"]
pub type HUFFMANTABLEDC0GROUPNO = crate::Reg<huffmantabledc0groupno::HUFFMANTABLEDC0GROUPNO_SPEC>;
#[doc = "Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 0."]
pub mod huffmantabledc0groupno;
#[doc = "HUFFMANTABLEAC0CODELENGTH (r) register accessor: an alias for `Reg<HUFFMANTABLEAC0CODELENGTH_SPEC>`"]
pub type HUFFMANTABLEAC0CODELENGTH =
    crate::Reg<huffmantableac0codelength::HUFFMANTABLEAC0CODELENGTH_SPEC>;
#[doc = "Shows the code length distribution for AC Huffman Table 0. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
pub mod huffmantableac0codelength;
#[doc = "HUFFMANTABLEAC0GROUPNO (r) register accessor: an alias for `Reg<HUFFMANTABLEAC0GROUPNO_SPEC>`"]
pub type HUFFMANTABLEAC0GROUPNO = crate::Reg<huffmantableac0groupno::HUFFMANTABLEAC0GROUPNO_SPEC>;
#[doc = "Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 0."]
pub mod huffmantableac0groupno;
#[doc = "HUFFMANTABLEDC1CODELENGTH (r) register accessor: an alias for `Reg<HUFFMANTABLEDC1CODELENGTH_SPEC>`"]
pub type HUFFMANTABLEDC1CODELENGTH =
    crate::Reg<huffmantabledc1codelength::HUFFMANTABLEDC1CODELENGTH_SPEC>;
#[doc = "Shows the code length distribution for DC Huffman Table 1. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
pub mod huffmantabledc1codelength;
#[doc = "HUFFMANTABLEDC1GROUPNO (r) register accessor: an alias for `Reg<HUFFMANTABLEDC1GROUPNO_SPEC>`"]
pub type HUFFMANTABLEDC1GROUPNO = crate::Reg<huffmantabledc1groupno::HUFFMANTABLEDC1GROUPNO_SPEC>;
#[doc = "Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 1."]
pub mod huffmantabledc1groupno;
#[doc = "HUFFMANTABLEAC1CODELENGTH (r) register accessor: an alias for `Reg<HUFFMANTABLEAC1CODELENGTH_SPEC>`"]
pub type HUFFMANTABLEAC1CODELENGTH =
    crate::Reg<huffmantableac1codelength::HUFFMANTABLEAC1CODELENGTH_SPEC>;
#[doc = "Shows the code length distribution for AC Huffman Table 1. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used."]
pub mod huffmantableac1codelength;
#[doc = "HUFFMANTABLEAC1GROUPNO (r) register accessor: an alias for `Reg<HUFFMANTABLEAC1GROUPNO_SPEC>`"]
pub type HUFFMANTABLEAC1GROUPNO = crate::Reg<huffmantableac1groupno::HUFFMANTABLEAC1GROUPNO_SPEC>;
#[doc = "Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 1."]
pub mod huffmantableac1groupno;
