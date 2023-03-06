#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX` reader - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
pub type IDX_R = crate::FieldReader<u8, IDX_A>;
#[doc = "The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX_A {
    #[doc = "0: Bus master 0 MPU/SMPU. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31\\]: '0' MPU violation; '1': SMPU violation."]
    MPU_0 = 0,
    #[doc = "1: Bus master 1 MPU. See MPU_0 description."]
    MPU_1 = 1,
    #[doc = "2: Bus master 2 MPU. See MPU_0 description."]
    MPU_2 = 2,
    #[doc = "3: Bus master 3 MPU. See MPU_0 description."]
    MPU_3 = 3,
    #[doc = "4: Bus master 4 MPU. See MPU_0 description."]
    MPU_4 = 4,
    #[doc = "5: Bus master 5 MPU. See MPU_0 description."]
    MPU_5 = 5,
    #[doc = "6: Bus master 6 MPU. See MPU_0 description."]
    MPU_6 = 6,
    #[doc = "7: Bus master 7 MPU. See MPU_0 description."]
    MPU_7 = 7,
    #[doc = "8: Bus master 8 MPU. See MPU_0 description."]
    MPU_8 = 8,
    #[doc = "9: Bus master 9 MPU. See MPU_0 description."]
    MPU_9 = 9,
    #[doc = "10: Bus master 10 MPU. See MPU_0 description."]
    MPU_10 = 10,
    #[doc = "11: Bus master 11 MPU. See MPU_0 description."]
    MPU_11 = 11,
    #[doc = "12: Bus master 12 MPU. See MPU_0 description."]
    MPU_12 = 12,
    #[doc = "13: Bus master 13 MPU. See MPU_0 description."]
    MPU_13 = 13,
    #[doc = "14: Bus master 14 MPU. See MPU_0 description."]
    MPU_14 = 14,
    #[doc = "15: Bus master 15 MPU. See MPU_0 description."]
    MPU_15 = 15,
    #[doc = "16: Correctable ECC error in CM7_1 TCM memory. See CM7_0_TCM_C_ECC description."]
    CM7_1_TCM_C_ECC = 16,
    #[doc = "17: Non Correctable ECC error in CM7_1 TCM memory. See CM7_0_TCM_C_ECC description."]
    CM7_1_TCM_NC_ECC = 17,
    #[doc = "18: Correctable ECC error in CM7_0 Cache memories DATA0\\[16:2\\]: location information: Tag/Data SRAM, Way, Index and line Offset, see CM7 UGRM IEBR0/DEBR0 description for details. DATA0\\[31\\]: 0=Instruction cache, 1= Data cache"]
    CM7_0_CACHE_C_ECC = 18,
    #[doc = "19: Non Correctable ECC error in CM7_0 Cache memories. See CM7_0_CACHE_C_ECC description"]
    CM7_0_CACHE_NC_ECC = 19,
    #[doc = "20: Correctable ECC error in CM7_1 Cache memories. See CM7_0_CACHE_C_ECC description."]
    CM7_1_CACHE_C_ECC = 20,
    #[doc = "21: Non Correctable ECC error in CM7_1 Cache memories. See CM7_0_CACHE_C_ECC description."]
    CM7_1_CACHE_NC_ECC = 21,
    #[doc = "25: Peripheral interconnect, master interface 4 PPU. See MS_PPU_0 description."]
    MS_PPU_4 = 25,
    #[doc = "26: Peripheral interconnect, protection structures SRAM, correctable ECC error: DATA0\\[10:0\\]: Violating address. DATA1\\[7:0\\]: Syndrome of SRAM word."]
    PERI_ECC = 26,
    #[doc = "27: Peripheral interconnect, protection structures SRAM, non-correctable ECC error."]
    PERI_NC_ECC = 27,
    #[doc = "28: Peripheral interconnect, master interface 0 PPU. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31:28\\]: '0': master interface, PPU violation, '1': timeout detected, '2': bus error, other: undefined."]
    MS_PPU_0 = 28,
    #[doc = "29: Peripheral interconnect, master interface 1 PPU. See MS_PPU_0 description."]
    MS_PPU_1 = 29,
    #[doc = "30: Peripheral interconnect, master interface 2 PPU. See MS_PPU_0 description."]
    MS_PPU_2 = 30,
    #[doc = "31: Peripheral interconnect, master interface 3 PPU. See MS_PPU_0 description."]
    MS_PPU_3 = 31,
    #[doc = "32: Peripheral group 0 fault detection. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31:28\\]: '0': decoder or peripheral bus error, other: undefined."]
    GROUP_FAULT_0 = 32,
    #[doc = "33: Peripheral group 1 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_1 = 33,
    #[doc = "34: Peripheral group 2 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_2 = 34,
    #[doc = "35: Peripheral group 3 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_3 = 35,
    #[doc = "36: Peripheral group 4 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_4 = 36,
    #[doc = "37: Peripheral group 5 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_5 = 37,
    #[doc = "38: Peripheral group 6 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_6 = 38,
    #[doc = "39: Peripheral group 7 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_7 = 39,
    #[doc = "40: Peripheral group 8 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_8 = 40,
    #[doc = "41: Peripheral group 9 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_9 = 41,
    #[doc = "42: Peripheral group 10 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_10 = 42,
    #[doc = "43: Peripheral group 11 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_11 = 43,
    #[doc = "44: Peripheral group 12 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_12 = 44,
    #[doc = "45: Peripheral group 13 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_13 = 45,
    #[doc = "46: Peripheral group 14 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_14 = 46,
    #[doc = "47: Peripheral group 15 fault detection. See GROUP_FAULT_0 description."]
    GROUP_FAULT_15 = 47,
    #[doc = "48: Flash controller, main interface, bus error: FAULT_DATA0\\[26:0\\]: Violating address. Append 5'b00010 as most significant bits to derive 32-bit system address. FAULT_DATA1\\[11:8\\]: Master identifier."]
    FLASHC_MAIN_BUS_ERROR = 48,
    #[doc = "49: Flash controller, main interface, correctable ECC error: DATA\\[26:0\\]: Violating address. Append 5'b00010 as most significant bits to derive 32-bit system address. DATA1\\[7:0\\]: Syndrome of 64-bit word (at address offset 0x00). DATA1\\[15:8\\]: Syndrome of 64-bit word (at address offset 0x08). DATA1\\[23:16\\]: Syndrome of 64-bit word (at address offset 0x10). DATA1\\[31:24\\]: Syndrome of 64-bit word (at address offset 0x18)."]
    FLASHC_MAIN_C_ECC = 49,
    #[doc = "50: Flash controller, main interface, non-correctable ECC error. See FLASHC_MAIN_C_ECC description."]
    FLASHC_MAIN_NC_ECC = 50,
    #[doc = "51: Flash controller, work interface, bus error. See FLASHC_MAIN_BUS_ERROR description."]
    FLASHC_WORK_BUS_ERROR = 51,
    #[doc = "52: Flash controller, work interface, correctable ECC error: DATA0\\[26:0\\]: Violating address. Append 5'b00010 as most significant bits to derive 32-bit system address. DATA1\\[6:0\\]: Syndrome of 32-bit word."]
    FLASHC_WORK_C_ECC = 52,
    #[doc = "53: Flash controller, work interface, non-correctable ECC error. See FLASHC_WORK_C_ECC description."]
    FLASHC_WORK_NC_ECC = 53,
    #[doc = "54: Flash controller, CM0+ cache, correctable ECC error: DATA0\\[26:0\\]: Violating address. DATA1\\[6:0\\]: Syndrome of 32-bit SRAM word (at address offset 0x0). DATA1\\[14:8\\]: Syndrome of 32-bit SRAM word (at address offset 0x4). DATA1\\[22:16\\]: Syndrome of 32-bit SRAM word (at address offset 0x8). DATA1\\[30:24\\]: Syndrome of 32-bit SRAM word (at address offset 0xc)."]
    FLASHC_CM0_CA_C_ECC = 54,
    #[doc = "55: Flash controller, CM0+ cache, non-correctable ECC error. See FLASHC_CM0_CA_C_ECC description."]
    FLASHC_CM0_CA_NC_ECC = 55,
    #[doc = "56: Correctable ECC error in CM7_0 TCM memory DATA0\\[23:2\\]: Violating address. DATA1\\[7:0\\]: Syndrome of code word (at address offset 0x0). DATA1\\[31:30\\]: 0= ITCM, 2=D0TCM, 3=D1TCM"]
    CM7_0_TCM_C_ECC = 56,
    #[doc = "57: Non Correctable ECC error in CM7_0 TCM memory. See CM7_0_TCM_C_ECC description."]
    CM7_0_TCM_NC_ECC = 57,
    #[doc = "58: System SRAM 0 correctable ECC error: DATA0\\[31:0\\]: Violating address. DATA1\\[6:0\\]: Syndrome of 32-bit SRAM code word."]
    RAMC0_C_ECC = 58,
    #[doc = "59: System SRAM 0 non-correctable ECC error. See RAMC0_C_ECC description."]
    RAMC0_NC_ECC = 59,
    #[doc = "60: System SRAM 1 correctable ECC error. See RAMC0_C_ECC description."]
    RAMC1_C_ECC = 60,
    #[doc = "61: System SRAM 1 non-correctable ECC error. See RAMC0_C_ECC description."]
    RAMC1_NC_ECC = 61,
    #[doc = "62: System SRAM 2 correctable ECC error. See RAMC0_C_ECC description."]
    RAMC2_C_ECC = 62,
    #[doc = "63: System SRAM 2 non-correctable ECC error. See RAMC0_C_ECC description."]
    RAMC2_NC_ECC = 63,
    #[doc = "64: Cryptography SRAM correctable ECC error. DATA0\\[31:0\\]: Violating address. DATA1\\[6:0\\]: Syndrome of Least Significant 32-bit SRAM. DATA1\\[14:8\\]: Syndrome of Most Significant 32-bit SRAM."]
    CRYPTO_C_ECC = 64,
    #[doc = "65: Cryptography SRAM non-correctable ECC error. See CRYPTO_C_ECC description."]
    CRYPTO_NC_ECC = 65,
    #[doc = "70: DataWire 0 SRAM 1 correctable ECC error: DATA0\\[11:0\\]: Violating DW SRAM address (word address, assuming byte addressable). DATA1\\[6:0\\]: Syndrome of 32-bit SRAM code word."]
    DW0_C_ECC = 70,
    #[doc = "71: DataWire 0 SRAM 1 non-correctable ECC error. See DW0_C_ECC description."]
    DW0_NC_ECC = 71,
    #[doc = "72: DataWire 1 SRAM 1 correctable ECC error. See DW0_C_ECC description."]
    DW1_C_ECC = 72,
    #[doc = "73: DataWire 1 SRAM 1 non-correctable ECC error. See DW0_C_ECC description."]
    DW1_NC_ECC = 73,
    #[doc = "74: eCT Flash SRAM (for embedded operations) correctable ECC error: DATA0\\[15:0\\]: Address location in the eCT Flash SRAM. DATA1\\[6:0\\]: Syndrome of 32-bit SRAM word."]
    FM_SRAM_C_ECC = 74,
    #[doc = "75: eCT Flash SRAM non-correctable ECC error: See FM_SRAM_C_ECC description."]
    FM_SRAM_NC_ECC = 75,
    #[doc = "80: CAN controller 0 MRAM correctable ECC error: DATA0\\[15:0\\]: Violating address. DATA0\\[22:16\\]: ECC violating data\\[38:32\\]
from MRAM. DATA0\\[27:24\\]: Master ID: 0-7 = CAN channel ID within mxttcanfd cluster, 8 = AHB I/F DATA1\\[31:0\\]: ECC violating data\\[31:0\\]
from MRAM."]
    CAN0_C_ECC = 80,
    #[doc = "81: CAN controller 0 MRAM non-correctable ECC error: DATA0\\[15:0\\]: Violating address. DATA0\\[22:16\\]: ECC violating data\\[38:32\\]
from MRAM (not for Address Error). DATA0\\[27:24\\]: Master ID: 0-7 = CAN channel ID within mxttcanfd cluster, 8 = AHB I/F DATA0\\[30\\]: Write access, only possible for Address Error DATA0\\[31\\]: Address Error: a CAN channel did an MRAM access above MRAM_SIZE DATA1\\[31:0\\]: ECC violating data\\[31:0\\]
from MRAM (not for Address Error)."]
    CAN0_NC_ECC = 81,
    #[doc = "82: CAN controller 1 MRAM correctable ECC error. See CAN0_C_ECC description."]
    CAN1_C_ECC = 82,
    #[doc = "83: CAN controller 1 MRAM non-correctable ECC error. See CAN0_NC_ECC description."]
    CAN1_NC_ECC = 83,
    #[doc = "84: Video Ram Protection Unit 0 fault detection. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31:28\\]: '0': decoder or peripheral bus error, other: undefined."]
    VIDEOSS_VRPU0 = 84,
    #[doc = "85: Video Ram Protection Unit 1 fault detection. See VIDEOSS_VRPU0 description."]
    VIDEOSS_VRPU1 = 85,
    #[doc = "86: Video Ram Protection Unit 2 fault detection. See VIDEOSS_VRPU0 description."]
    VIDEOSS_VRPU2 = 86,
    #[doc = "87: Video Ram Protection Unit 3 fault detection. See VIDEOSS_VRPU0 description."]
    VIDEOSS_VRPU3 = 87,
    #[doc = "88: Video Ram Protection Unit 4 fault detection. See VIDEOSS_VRPU0 description."]
    VIDEOSS_VRPU4 = 88,
    #[doc = "89: Video Ram Protection Unit 5 fault detection. See VIDEOSS_VRPU0 description."]
    VIDEOSS_VRPU5 = 89,
    #[doc = "90: SRSS Clock SuperVisor (CSV) violation detected. Multiple CSV can detect a violation at the same time. DATA0\\[15:0\\]: clk_hf* root CSV violation flags. DATA0\\[24\\]: clk_ref CSV violation flag (reference clock for clk_hf CSVs) DATA0\\[25\\]: clk_lf CSV violation flag DATA0\\[26\\]: clk_hvilo CSV violation flag"]
    SRSS_CSV = 90,
    #[doc = "91: SRSS Clock SuperVisor (CSV) violation detected. Multiple CSV can detect a violation at the same time. DATA0\\[0\\]: BOD on VDDA DATA\\[1\\]: OVD on VDDA DATA\\[16\\]: LVD/HVD #1 DATA0\\[17\\]: LVD/HVD #2"]
    SRSS_SSV = 91,
    #[doc = "92: SRSS Multi-Counter Watch Dog Timer (MCWDT) #0 violation detected. Multiple counters can detect a violation at the same time. DATA0\\[0\\]: MCWDT subcounter 0 LOWER_LIMIT DATA0\\[1\\]: MCWDT subcounter 0 UPPER_LIMIT DATA0\\[2\\]: MCWDT subcounter 1 LOWER_LIMIT DATA0\\[3\\]: MCWDT subcounter 1 UPPER_LIMIT"]
    SRSS_MCWDT0 = 92,
    #[doc = "93: SRSS Multi-Counter Watch Dog Timer (MCWDT) #1 violation detected. See SRSS_MCWDT0 description."]
    SRSS_MCWDT1 = 93,
    #[doc = "94: SRSS Multi-Counter Watch Dog Timer (MCWDT) #2 violation detected. See SRSS_MCWDT0 description."]
    SRSS_MCWDT2 = 94,
    #[doc = "95: SRSS Multi-Counter Watch Dog Timer (MCWDT) #3 violation detected. See SRSS_MCWDT0 description."]
    SRSS_MCWDT3 = 95,
}
impl From<IDX_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX_A) -> Self {
        variant as _
    }
}
impl IDX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX_A> {
        match self.bits {
            0 => Some(IDX_A::MPU_0),
            1 => Some(IDX_A::MPU_1),
            2 => Some(IDX_A::MPU_2),
            3 => Some(IDX_A::MPU_3),
            4 => Some(IDX_A::MPU_4),
            5 => Some(IDX_A::MPU_5),
            6 => Some(IDX_A::MPU_6),
            7 => Some(IDX_A::MPU_7),
            8 => Some(IDX_A::MPU_8),
            9 => Some(IDX_A::MPU_9),
            10 => Some(IDX_A::MPU_10),
            11 => Some(IDX_A::MPU_11),
            12 => Some(IDX_A::MPU_12),
            13 => Some(IDX_A::MPU_13),
            14 => Some(IDX_A::MPU_14),
            15 => Some(IDX_A::MPU_15),
            16 => Some(IDX_A::CM7_1_TCM_C_ECC),
            17 => Some(IDX_A::CM7_1_TCM_NC_ECC),
            18 => Some(IDX_A::CM7_0_CACHE_C_ECC),
            19 => Some(IDX_A::CM7_0_CACHE_NC_ECC),
            20 => Some(IDX_A::CM7_1_CACHE_C_ECC),
            21 => Some(IDX_A::CM7_1_CACHE_NC_ECC),
            25 => Some(IDX_A::MS_PPU_4),
            26 => Some(IDX_A::PERI_ECC),
            27 => Some(IDX_A::PERI_NC_ECC),
            28 => Some(IDX_A::MS_PPU_0),
            29 => Some(IDX_A::MS_PPU_1),
            30 => Some(IDX_A::MS_PPU_2),
            31 => Some(IDX_A::MS_PPU_3),
            32 => Some(IDX_A::GROUP_FAULT_0),
            33 => Some(IDX_A::GROUP_FAULT_1),
            34 => Some(IDX_A::GROUP_FAULT_2),
            35 => Some(IDX_A::GROUP_FAULT_3),
            36 => Some(IDX_A::GROUP_FAULT_4),
            37 => Some(IDX_A::GROUP_FAULT_5),
            38 => Some(IDX_A::GROUP_FAULT_6),
            39 => Some(IDX_A::GROUP_FAULT_7),
            40 => Some(IDX_A::GROUP_FAULT_8),
            41 => Some(IDX_A::GROUP_FAULT_9),
            42 => Some(IDX_A::GROUP_FAULT_10),
            43 => Some(IDX_A::GROUP_FAULT_11),
            44 => Some(IDX_A::GROUP_FAULT_12),
            45 => Some(IDX_A::GROUP_FAULT_13),
            46 => Some(IDX_A::GROUP_FAULT_14),
            47 => Some(IDX_A::GROUP_FAULT_15),
            48 => Some(IDX_A::FLASHC_MAIN_BUS_ERROR),
            49 => Some(IDX_A::FLASHC_MAIN_C_ECC),
            50 => Some(IDX_A::FLASHC_MAIN_NC_ECC),
            51 => Some(IDX_A::FLASHC_WORK_BUS_ERROR),
            52 => Some(IDX_A::FLASHC_WORK_C_ECC),
            53 => Some(IDX_A::FLASHC_WORK_NC_ECC),
            54 => Some(IDX_A::FLASHC_CM0_CA_C_ECC),
            55 => Some(IDX_A::FLASHC_CM0_CA_NC_ECC),
            56 => Some(IDX_A::CM7_0_TCM_C_ECC),
            57 => Some(IDX_A::CM7_0_TCM_NC_ECC),
            58 => Some(IDX_A::RAMC0_C_ECC),
            59 => Some(IDX_A::RAMC0_NC_ECC),
            60 => Some(IDX_A::RAMC1_C_ECC),
            61 => Some(IDX_A::RAMC1_NC_ECC),
            62 => Some(IDX_A::RAMC2_C_ECC),
            63 => Some(IDX_A::RAMC2_NC_ECC),
            64 => Some(IDX_A::CRYPTO_C_ECC),
            65 => Some(IDX_A::CRYPTO_NC_ECC),
            70 => Some(IDX_A::DW0_C_ECC),
            71 => Some(IDX_A::DW0_NC_ECC),
            72 => Some(IDX_A::DW1_C_ECC),
            73 => Some(IDX_A::DW1_NC_ECC),
            74 => Some(IDX_A::FM_SRAM_C_ECC),
            75 => Some(IDX_A::FM_SRAM_NC_ECC),
            80 => Some(IDX_A::CAN0_C_ECC),
            81 => Some(IDX_A::CAN0_NC_ECC),
            82 => Some(IDX_A::CAN1_C_ECC),
            83 => Some(IDX_A::CAN1_NC_ECC),
            84 => Some(IDX_A::VIDEOSS_VRPU0),
            85 => Some(IDX_A::VIDEOSS_VRPU1),
            86 => Some(IDX_A::VIDEOSS_VRPU2),
            87 => Some(IDX_A::VIDEOSS_VRPU3),
            88 => Some(IDX_A::VIDEOSS_VRPU4),
            89 => Some(IDX_A::VIDEOSS_VRPU5),
            90 => Some(IDX_A::SRSS_CSV),
            91 => Some(IDX_A::SRSS_SSV),
            92 => Some(IDX_A::SRSS_MCWDT0),
            93 => Some(IDX_A::SRSS_MCWDT1),
            94 => Some(IDX_A::SRSS_MCWDT2),
            95 => Some(IDX_A::SRSS_MCWDT3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MPU_0`"]
    #[inline(always)]
    pub fn is_mpu_0(&self) -> bool {
        *self == IDX_A::MPU_0
    }
    #[doc = "Checks if the value of the field is `MPU_1`"]
    #[inline(always)]
    pub fn is_mpu_1(&self) -> bool {
        *self == IDX_A::MPU_1
    }
    #[doc = "Checks if the value of the field is `MPU_2`"]
    #[inline(always)]
    pub fn is_mpu_2(&self) -> bool {
        *self == IDX_A::MPU_2
    }
    #[doc = "Checks if the value of the field is `MPU_3`"]
    #[inline(always)]
    pub fn is_mpu_3(&self) -> bool {
        *self == IDX_A::MPU_3
    }
    #[doc = "Checks if the value of the field is `MPU_4`"]
    #[inline(always)]
    pub fn is_mpu_4(&self) -> bool {
        *self == IDX_A::MPU_4
    }
    #[doc = "Checks if the value of the field is `MPU_5`"]
    #[inline(always)]
    pub fn is_mpu_5(&self) -> bool {
        *self == IDX_A::MPU_5
    }
    #[doc = "Checks if the value of the field is `MPU_6`"]
    #[inline(always)]
    pub fn is_mpu_6(&self) -> bool {
        *self == IDX_A::MPU_6
    }
    #[doc = "Checks if the value of the field is `MPU_7`"]
    #[inline(always)]
    pub fn is_mpu_7(&self) -> bool {
        *self == IDX_A::MPU_7
    }
    #[doc = "Checks if the value of the field is `MPU_8`"]
    #[inline(always)]
    pub fn is_mpu_8(&self) -> bool {
        *self == IDX_A::MPU_8
    }
    #[doc = "Checks if the value of the field is `MPU_9`"]
    #[inline(always)]
    pub fn is_mpu_9(&self) -> bool {
        *self == IDX_A::MPU_9
    }
    #[doc = "Checks if the value of the field is `MPU_10`"]
    #[inline(always)]
    pub fn is_mpu_10(&self) -> bool {
        *self == IDX_A::MPU_10
    }
    #[doc = "Checks if the value of the field is `MPU_11`"]
    #[inline(always)]
    pub fn is_mpu_11(&self) -> bool {
        *self == IDX_A::MPU_11
    }
    #[doc = "Checks if the value of the field is `MPU_12`"]
    #[inline(always)]
    pub fn is_mpu_12(&self) -> bool {
        *self == IDX_A::MPU_12
    }
    #[doc = "Checks if the value of the field is `MPU_13`"]
    #[inline(always)]
    pub fn is_mpu_13(&self) -> bool {
        *self == IDX_A::MPU_13
    }
    #[doc = "Checks if the value of the field is `MPU_14`"]
    #[inline(always)]
    pub fn is_mpu_14(&self) -> bool {
        *self == IDX_A::MPU_14
    }
    #[doc = "Checks if the value of the field is `MPU_15`"]
    #[inline(always)]
    pub fn is_mpu_15(&self) -> bool {
        *self == IDX_A::MPU_15
    }
    #[doc = "Checks if the value of the field is `CM7_1_TCM_C_ECC`"]
    #[inline(always)]
    pub fn is_cm7_1_tcm_c_ecc(&self) -> bool {
        *self == IDX_A::CM7_1_TCM_C_ECC
    }
    #[doc = "Checks if the value of the field is `CM7_1_TCM_NC_ECC`"]
    #[inline(always)]
    pub fn is_cm7_1_tcm_nc_ecc(&self) -> bool {
        *self == IDX_A::CM7_1_TCM_NC_ECC
    }
    #[doc = "Checks if the value of the field is `CM7_0_CACHE_C_ECC`"]
    #[inline(always)]
    pub fn is_cm7_0_cache_c_ecc(&self) -> bool {
        *self == IDX_A::CM7_0_CACHE_C_ECC
    }
    #[doc = "Checks if the value of the field is `CM7_0_CACHE_NC_ECC`"]
    #[inline(always)]
    pub fn is_cm7_0_cache_nc_ecc(&self) -> bool {
        *self == IDX_A::CM7_0_CACHE_NC_ECC
    }
    #[doc = "Checks if the value of the field is `CM7_1_CACHE_C_ECC`"]
    #[inline(always)]
    pub fn is_cm7_1_cache_c_ecc(&self) -> bool {
        *self == IDX_A::CM7_1_CACHE_C_ECC
    }
    #[doc = "Checks if the value of the field is `CM7_1_CACHE_NC_ECC`"]
    #[inline(always)]
    pub fn is_cm7_1_cache_nc_ecc(&self) -> bool {
        *self == IDX_A::CM7_1_CACHE_NC_ECC
    }
    #[doc = "Checks if the value of the field is `MS_PPU_4`"]
    #[inline(always)]
    pub fn is_ms_ppu_4(&self) -> bool {
        *self == IDX_A::MS_PPU_4
    }
    #[doc = "Checks if the value of the field is `PERI_ECC`"]
    #[inline(always)]
    pub fn is_peri_ecc(&self) -> bool {
        *self == IDX_A::PERI_ECC
    }
    #[doc = "Checks if the value of the field is `PERI_NC_ECC`"]
    #[inline(always)]
    pub fn is_peri_nc_ecc(&self) -> bool {
        *self == IDX_A::PERI_NC_ECC
    }
    #[doc = "Checks if the value of the field is `MS_PPU_0`"]
    #[inline(always)]
    pub fn is_ms_ppu_0(&self) -> bool {
        *self == IDX_A::MS_PPU_0
    }
    #[doc = "Checks if the value of the field is `MS_PPU_1`"]
    #[inline(always)]
    pub fn is_ms_ppu_1(&self) -> bool {
        *self == IDX_A::MS_PPU_1
    }
    #[doc = "Checks if the value of the field is `MS_PPU_2`"]
    #[inline(always)]
    pub fn is_ms_ppu_2(&self) -> bool {
        *self == IDX_A::MS_PPU_2
    }
    #[doc = "Checks if the value of the field is `MS_PPU_3`"]
    #[inline(always)]
    pub fn is_ms_ppu_3(&self) -> bool {
        *self == IDX_A::MS_PPU_3
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_0`"]
    #[inline(always)]
    pub fn is_group_fault_0(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_0
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_1`"]
    #[inline(always)]
    pub fn is_group_fault_1(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_1
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_2`"]
    #[inline(always)]
    pub fn is_group_fault_2(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_2
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_3`"]
    #[inline(always)]
    pub fn is_group_fault_3(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_3
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_4`"]
    #[inline(always)]
    pub fn is_group_fault_4(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_4
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_5`"]
    #[inline(always)]
    pub fn is_group_fault_5(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_5
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_6`"]
    #[inline(always)]
    pub fn is_group_fault_6(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_6
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_7`"]
    #[inline(always)]
    pub fn is_group_fault_7(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_7
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_8`"]
    #[inline(always)]
    pub fn is_group_fault_8(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_8
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_9`"]
    #[inline(always)]
    pub fn is_group_fault_9(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_9
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_10`"]
    #[inline(always)]
    pub fn is_group_fault_10(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_10
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_11`"]
    #[inline(always)]
    pub fn is_group_fault_11(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_11
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_12`"]
    #[inline(always)]
    pub fn is_group_fault_12(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_12
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_13`"]
    #[inline(always)]
    pub fn is_group_fault_13(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_13
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_14`"]
    #[inline(always)]
    pub fn is_group_fault_14(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_14
    }
    #[doc = "Checks if the value of the field is `GROUP_FAULT_15`"]
    #[inline(always)]
    pub fn is_group_fault_15(&self) -> bool {
        *self == IDX_A::GROUP_FAULT_15
    }
    #[doc = "Checks if the value of the field is `FLASHC_MAIN_BUS_ERROR`"]
    #[inline(always)]
    pub fn is_flashc_main_bus_error(&self) -> bool {
        *self == IDX_A::FLASHC_MAIN_BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `FLASHC_MAIN_C_ECC`"]
    #[inline(always)]
    pub fn is_flashc_main_c_ecc(&self) -> bool {
        *self == IDX_A::FLASHC_MAIN_C_ECC
    }
    #[doc = "Checks if the value of the field is `FLASHC_MAIN_NC_ECC`"]
    #[inline(always)]
    pub fn is_flashc_main_nc_ecc(&self) -> bool {
        *self == IDX_A::FLASHC_MAIN_NC_ECC
    }
    #[doc = "Checks if the value of the field is `FLASHC_WORK_BUS_ERROR`"]
    #[inline(always)]
    pub fn is_flashc_work_bus_error(&self) -> bool {
        *self == IDX_A::FLASHC_WORK_BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `FLASHC_WORK_C_ECC`"]
    #[inline(always)]
    pub fn is_flashc_work_c_ecc(&self) -> bool {
        *self == IDX_A::FLASHC_WORK_C_ECC
    }
    #[doc = "Checks if the value of the field is `FLASHC_WORK_NC_ECC`"]
    #[inline(always)]
    pub fn is_flashc_work_nc_ecc(&self) -> bool {
        *self == IDX_A::FLASHC_WORK_NC_ECC
    }
    #[doc = "Checks if the value of the field is `FLASHC_CM0_CA_C_ECC`"]
    #[inline(always)]
    pub fn is_flashc_cm0_ca_c_ecc(&self) -> bool {
        *self == IDX_A::FLASHC_CM0_CA_C_ECC
    }
    #[doc = "Checks if the value of the field is `FLASHC_CM0_CA_NC_ECC`"]
    #[inline(always)]
    pub fn is_flashc_cm0_ca_nc_ecc(&self) -> bool {
        *self == IDX_A::FLASHC_CM0_CA_NC_ECC
    }
    #[doc = "Checks if the value of the field is `CM7_0_TCM_C_ECC`"]
    #[inline(always)]
    pub fn is_cm7_0_tcm_c_ecc(&self) -> bool {
        *self == IDX_A::CM7_0_TCM_C_ECC
    }
    #[doc = "Checks if the value of the field is `CM7_0_TCM_NC_ECC`"]
    #[inline(always)]
    pub fn is_cm7_0_tcm_nc_ecc(&self) -> bool {
        *self == IDX_A::CM7_0_TCM_NC_ECC
    }
    #[doc = "Checks if the value of the field is `RAMC0_C_ECC`"]
    #[inline(always)]
    pub fn is_ramc0_c_ecc(&self) -> bool {
        *self == IDX_A::RAMC0_C_ECC
    }
    #[doc = "Checks if the value of the field is `RAMC0_NC_ECC`"]
    #[inline(always)]
    pub fn is_ramc0_nc_ecc(&self) -> bool {
        *self == IDX_A::RAMC0_NC_ECC
    }
    #[doc = "Checks if the value of the field is `RAMC1_C_ECC`"]
    #[inline(always)]
    pub fn is_ramc1_c_ecc(&self) -> bool {
        *self == IDX_A::RAMC1_C_ECC
    }
    #[doc = "Checks if the value of the field is `RAMC1_NC_ECC`"]
    #[inline(always)]
    pub fn is_ramc1_nc_ecc(&self) -> bool {
        *self == IDX_A::RAMC1_NC_ECC
    }
    #[doc = "Checks if the value of the field is `RAMC2_C_ECC`"]
    #[inline(always)]
    pub fn is_ramc2_c_ecc(&self) -> bool {
        *self == IDX_A::RAMC2_C_ECC
    }
    #[doc = "Checks if the value of the field is `RAMC2_NC_ECC`"]
    #[inline(always)]
    pub fn is_ramc2_nc_ecc(&self) -> bool {
        *self == IDX_A::RAMC2_NC_ECC
    }
    #[doc = "Checks if the value of the field is `CRYPTO_C_ECC`"]
    #[inline(always)]
    pub fn is_crypto_c_ecc(&self) -> bool {
        *self == IDX_A::CRYPTO_C_ECC
    }
    #[doc = "Checks if the value of the field is `CRYPTO_NC_ECC`"]
    #[inline(always)]
    pub fn is_crypto_nc_ecc(&self) -> bool {
        *self == IDX_A::CRYPTO_NC_ECC
    }
    #[doc = "Checks if the value of the field is `DW0_C_ECC`"]
    #[inline(always)]
    pub fn is_dw0_c_ecc(&self) -> bool {
        *self == IDX_A::DW0_C_ECC
    }
    #[doc = "Checks if the value of the field is `DW0_NC_ECC`"]
    #[inline(always)]
    pub fn is_dw0_nc_ecc(&self) -> bool {
        *self == IDX_A::DW0_NC_ECC
    }
    #[doc = "Checks if the value of the field is `DW1_C_ECC`"]
    #[inline(always)]
    pub fn is_dw1_c_ecc(&self) -> bool {
        *self == IDX_A::DW1_C_ECC
    }
    #[doc = "Checks if the value of the field is `DW1_NC_ECC`"]
    #[inline(always)]
    pub fn is_dw1_nc_ecc(&self) -> bool {
        *self == IDX_A::DW1_NC_ECC
    }
    #[doc = "Checks if the value of the field is `FM_SRAM_C_ECC`"]
    #[inline(always)]
    pub fn is_fm_sram_c_ecc(&self) -> bool {
        *self == IDX_A::FM_SRAM_C_ECC
    }
    #[doc = "Checks if the value of the field is `FM_SRAM_NC_ECC`"]
    #[inline(always)]
    pub fn is_fm_sram_nc_ecc(&self) -> bool {
        *self == IDX_A::FM_SRAM_NC_ECC
    }
    #[doc = "Checks if the value of the field is `CAN0_C_ECC`"]
    #[inline(always)]
    pub fn is_can0_c_ecc(&self) -> bool {
        *self == IDX_A::CAN0_C_ECC
    }
    #[doc = "Checks if the value of the field is `CAN0_NC_ECC`"]
    #[inline(always)]
    pub fn is_can0_nc_ecc(&self) -> bool {
        *self == IDX_A::CAN0_NC_ECC
    }
    #[doc = "Checks if the value of the field is `CAN1_C_ECC`"]
    #[inline(always)]
    pub fn is_can1_c_ecc(&self) -> bool {
        *self == IDX_A::CAN1_C_ECC
    }
    #[doc = "Checks if the value of the field is `CAN1_NC_ECC`"]
    #[inline(always)]
    pub fn is_can1_nc_ecc(&self) -> bool {
        *self == IDX_A::CAN1_NC_ECC
    }
    #[doc = "Checks if the value of the field is `VIDEOSS_VRPU0`"]
    #[inline(always)]
    pub fn is_videoss_vrpu0(&self) -> bool {
        *self == IDX_A::VIDEOSS_VRPU0
    }
    #[doc = "Checks if the value of the field is `VIDEOSS_VRPU1`"]
    #[inline(always)]
    pub fn is_videoss_vrpu1(&self) -> bool {
        *self == IDX_A::VIDEOSS_VRPU1
    }
    #[doc = "Checks if the value of the field is `VIDEOSS_VRPU2`"]
    #[inline(always)]
    pub fn is_videoss_vrpu2(&self) -> bool {
        *self == IDX_A::VIDEOSS_VRPU2
    }
    #[doc = "Checks if the value of the field is `VIDEOSS_VRPU3`"]
    #[inline(always)]
    pub fn is_videoss_vrpu3(&self) -> bool {
        *self == IDX_A::VIDEOSS_VRPU3
    }
    #[doc = "Checks if the value of the field is `VIDEOSS_VRPU4`"]
    #[inline(always)]
    pub fn is_videoss_vrpu4(&self) -> bool {
        *self == IDX_A::VIDEOSS_VRPU4
    }
    #[doc = "Checks if the value of the field is `VIDEOSS_VRPU5`"]
    #[inline(always)]
    pub fn is_videoss_vrpu5(&self) -> bool {
        *self == IDX_A::VIDEOSS_VRPU5
    }
    #[doc = "Checks if the value of the field is `SRSS_CSV`"]
    #[inline(always)]
    pub fn is_srss_csv(&self) -> bool {
        *self == IDX_A::SRSS_CSV
    }
    #[doc = "Checks if the value of the field is `SRSS_SSV`"]
    #[inline(always)]
    pub fn is_srss_ssv(&self) -> bool {
        *self == IDX_A::SRSS_SSV
    }
    #[doc = "Checks if the value of the field is `SRSS_MCWDT0`"]
    #[inline(always)]
    pub fn is_srss_mcwdt0(&self) -> bool {
        *self == IDX_A::SRSS_MCWDT0
    }
    #[doc = "Checks if the value of the field is `SRSS_MCWDT1`"]
    #[inline(always)]
    pub fn is_srss_mcwdt1(&self) -> bool {
        *self == IDX_A::SRSS_MCWDT1
    }
    #[doc = "Checks if the value of the field is `SRSS_MCWDT2`"]
    #[inline(always)]
    pub fn is_srss_mcwdt2(&self) -> bool {
        *self == IDX_A::SRSS_MCWDT2
    }
    #[doc = "Checks if the value of the field is `SRSS_MCWDT3`"]
    #[inline(always)]
    pub fn is_srss_mcwdt3(&self) -> bool {
        *self == IDX_A::SRSS_MCWDT3
    }
}
#[doc = "Field `IDX` writer - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
pub type IDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, IDX_A, 7, O>;
impl<'a, const O: u8> IDX_W<'a, O> {
    #[doc = "Bus master 0 MPU/SMPU. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31\\]: '0' MPU violation; '1': SMPU violation."]
    #[inline(always)]
    pub fn mpu_0(self) -> &'a mut W {
        self.variant(IDX_A::MPU_0)
    }
    #[doc = "Bus master 1 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_1(self) -> &'a mut W {
        self.variant(IDX_A::MPU_1)
    }
    #[doc = "Bus master 2 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_2(self) -> &'a mut W {
        self.variant(IDX_A::MPU_2)
    }
    #[doc = "Bus master 3 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_3(self) -> &'a mut W {
        self.variant(IDX_A::MPU_3)
    }
    #[doc = "Bus master 4 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_4(self) -> &'a mut W {
        self.variant(IDX_A::MPU_4)
    }
    #[doc = "Bus master 5 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_5(self) -> &'a mut W {
        self.variant(IDX_A::MPU_5)
    }
    #[doc = "Bus master 6 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_6(self) -> &'a mut W {
        self.variant(IDX_A::MPU_6)
    }
    #[doc = "Bus master 7 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_7(self) -> &'a mut W {
        self.variant(IDX_A::MPU_7)
    }
    #[doc = "Bus master 8 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_8(self) -> &'a mut W {
        self.variant(IDX_A::MPU_8)
    }
    #[doc = "Bus master 9 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_9(self) -> &'a mut W {
        self.variant(IDX_A::MPU_9)
    }
    #[doc = "Bus master 10 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_10(self) -> &'a mut W {
        self.variant(IDX_A::MPU_10)
    }
    #[doc = "Bus master 11 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_11(self) -> &'a mut W {
        self.variant(IDX_A::MPU_11)
    }
    #[doc = "Bus master 12 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_12(self) -> &'a mut W {
        self.variant(IDX_A::MPU_12)
    }
    #[doc = "Bus master 13 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_13(self) -> &'a mut W {
        self.variant(IDX_A::MPU_13)
    }
    #[doc = "Bus master 14 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_14(self) -> &'a mut W {
        self.variant(IDX_A::MPU_14)
    }
    #[doc = "Bus master 15 MPU. See MPU_0 description."]
    #[inline(always)]
    pub fn mpu_15(self) -> &'a mut W {
        self.variant(IDX_A::MPU_15)
    }
    #[doc = "Correctable ECC error in CM7_1 TCM memory. See CM7_0_TCM_C_ECC description."]
    #[inline(always)]
    pub fn cm7_1_tcm_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_1_TCM_C_ECC)
    }
    #[doc = "Non Correctable ECC error in CM7_1 TCM memory. See CM7_0_TCM_C_ECC description."]
    #[inline(always)]
    pub fn cm7_1_tcm_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_1_TCM_NC_ECC)
    }
    #[doc = "Correctable ECC error in CM7_0 Cache memories DATA0\\[16:2\\]: location information: Tag/Data SRAM, Way, Index and line Offset, see CM7 UGRM IEBR0/DEBR0 description for details. DATA0\\[31\\]: 0=Instruction cache, 1= Data cache"]
    #[inline(always)]
    pub fn cm7_0_cache_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_0_CACHE_C_ECC)
    }
    #[doc = "Non Correctable ECC error in CM7_0 Cache memories. See CM7_0_CACHE_C_ECC description"]
    #[inline(always)]
    pub fn cm7_0_cache_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_0_CACHE_NC_ECC)
    }
    #[doc = "Correctable ECC error in CM7_1 Cache memories. See CM7_0_CACHE_C_ECC description."]
    #[inline(always)]
    pub fn cm7_1_cache_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_1_CACHE_C_ECC)
    }
    #[doc = "Non Correctable ECC error in CM7_1 Cache memories. See CM7_0_CACHE_C_ECC description."]
    #[inline(always)]
    pub fn cm7_1_cache_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_1_CACHE_NC_ECC)
    }
    #[doc = "Peripheral interconnect, master interface 4 PPU. See MS_PPU_0 description."]
    #[inline(always)]
    pub fn ms_ppu_4(self) -> &'a mut W {
        self.variant(IDX_A::MS_PPU_4)
    }
    #[doc = "Peripheral interconnect, protection structures SRAM, correctable ECC error: DATA0\\[10:0\\]: Violating address. DATA1\\[7:0\\]: Syndrome of SRAM word."]
    #[inline(always)]
    pub fn peri_ecc(self) -> &'a mut W {
        self.variant(IDX_A::PERI_ECC)
    }
    #[doc = "Peripheral interconnect, protection structures SRAM, non-correctable ECC error."]
    #[inline(always)]
    pub fn peri_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::PERI_NC_ECC)
    }
    #[doc = "Peripheral interconnect, master interface 0 PPU. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31:28\\]: '0': master interface, PPU violation, '1': timeout detected, '2': bus error, other: undefined."]
    #[inline(always)]
    pub fn ms_ppu_0(self) -> &'a mut W {
        self.variant(IDX_A::MS_PPU_0)
    }
    #[doc = "Peripheral interconnect, master interface 1 PPU. See MS_PPU_0 description."]
    #[inline(always)]
    pub fn ms_ppu_1(self) -> &'a mut W {
        self.variant(IDX_A::MS_PPU_1)
    }
    #[doc = "Peripheral interconnect, master interface 2 PPU. See MS_PPU_0 description."]
    #[inline(always)]
    pub fn ms_ppu_2(self) -> &'a mut W {
        self.variant(IDX_A::MS_PPU_2)
    }
    #[doc = "Peripheral interconnect, master interface 3 PPU. See MS_PPU_0 description."]
    #[inline(always)]
    pub fn ms_ppu_3(self) -> &'a mut W {
        self.variant(IDX_A::MS_PPU_3)
    }
    #[doc = "Peripheral group 0 fault detection. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31:28\\]: '0': decoder or peripheral bus error, other: undefined."]
    #[inline(always)]
    pub fn group_fault_0(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_0)
    }
    #[doc = "Peripheral group 1 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_1(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_1)
    }
    #[doc = "Peripheral group 2 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_2(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_2)
    }
    #[doc = "Peripheral group 3 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_3(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_3)
    }
    #[doc = "Peripheral group 4 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_4(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_4)
    }
    #[doc = "Peripheral group 5 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_5(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_5)
    }
    #[doc = "Peripheral group 6 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_6(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_6)
    }
    #[doc = "Peripheral group 7 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_7(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_7)
    }
    #[doc = "Peripheral group 8 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_8(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_8)
    }
    #[doc = "Peripheral group 9 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_9(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_9)
    }
    #[doc = "Peripheral group 10 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_10(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_10)
    }
    #[doc = "Peripheral group 11 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_11(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_11)
    }
    #[doc = "Peripheral group 12 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_12(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_12)
    }
    #[doc = "Peripheral group 13 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_13(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_13)
    }
    #[doc = "Peripheral group 14 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_14(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_14)
    }
    #[doc = "Peripheral group 15 fault detection. See GROUP_FAULT_0 description."]
    #[inline(always)]
    pub fn group_fault_15(self) -> &'a mut W {
        self.variant(IDX_A::GROUP_FAULT_15)
    }
    #[doc = "Flash controller, main interface, bus error: FAULT_DATA0\\[26:0\\]: Violating address. Append 5'b00010 as most significant bits to derive 32-bit system address. FAULT_DATA1\\[11:8\\]: Master identifier."]
    #[inline(always)]
    pub fn flashc_main_bus_error(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_MAIN_BUS_ERROR)
    }
    #[doc = "Flash controller, main interface, correctable ECC error: DATA\\[26:0\\]: Violating address. Append 5'b00010 as most significant bits to derive 32-bit system address. DATA1\\[7:0\\]: Syndrome of 64-bit word (at address offset 0x00). DATA1\\[15:8\\]: Syndrome of 64-bit word (at address offset 0x08). DATA1\\[23:16\\]: Syndrome of 64-bit word (at address offset 0x10). DATA1\\[31:24\\]: Syndrome of 64-bit word (at address offset 0x18)."]
    #[inline(always)]
    pub fn flashc_main_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_MAIN_C_ECC)
    }
    #[doc = "Flash controller, main interface, non-correctable ECC error. See FLASHC_MAIN_C_ECC description."]
    #[inline(always)]
    pub fn flashc_main_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_MAIN_NC_ECC)
    }
    #[doc = "Flash controller, work interface, bus error. See FLASHC_MAIN_BUS_ERROR description."]
    #[inline(always)]
    pub fn flashc_work_bus_error(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_WORK_BUS_ERROR)
    }
    #[doc = "Flash controller, work interface, correctable ECC error: DATA0\\[26:0\\]: Violating address. Append 5'b00010 as most significant bits to derive 32-bit system address. DATA1\\[6:0\\]: Syndrome of 32-bit word."]
    #[inline(always)]
    pub fn flashc_work_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_WORK_C_ECC)
    }
    #[doc = "Flash controller, work interface, non-correctable ECC error. See FLASHC_WORK_C_ECC description."]
    #[inline(always)]
    pub fn flashc_work_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_WORK_NC_ECC)
    }
    #[doc = "Flash controller, CM0+ cache, correctable ECC error: DATA0\\[26:0\\]: Violating address. DATA1\\[6:0\\]: Syndrome of 32-bit SRAM word (at address offset 0x0). DATA1\\[14:8\\]: Syndrome of 32-bit SRAM word (at address offset 0x4). DATA1\\[22:16\\]: Syndrome of 32-bit SRAM word (at address offset 0x8). DATA1\\[30:24\\]: Syndrome of 32-bit SRAM word (at address offset 0xc)."]
    #[inline(always)]
    pub fn flashc_cm0_ca_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_CM0_CA_C_ECC)
    }
    #[doc = "Flash controller, CM0+ cache, non-correctable ECC error. See FLASHC_CM0_CA_C_ECC description."]
    #[inline(always)]
    pub fn flashc_cm0_ca_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FLASHC_CM0_CA_NC_ECC)
    }
    #[doc = "Correctable ECC error in CM7_0 TCM memory DATA0\\[23:2\\]: Violating address. DATA1\\[7:0\\]: Syndrome of code word (at address offset 0x0). DATA1\\[31:30\\]: 0= ITCM, 2=D0TCM, 3=D1TCM"]
    #[inline(always)]
    pub fn cm7_0_tcm_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_0_TCM_C_ECC)
    }
    #[doc = "Non Correctable ECC error in CM7_0 TCM memory. See CM7_0_TCM_C_ECC description."]
    #[inline(always)]
    pub fn cm7_0_tcm_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CM7_0_TCM_NC_ECC)
    }
    #[doc = "System SRAM 0 correctable ECC error: DATA0\\[31:0\\]: Violating address. DATA1\\[6:0\\]: Syndrome of 32-bit SRAM code word."]
    #[inline(always)]
    pub fn ramc0_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::RAMC0_C_ECC)
    }
    #[doc = "System SRAM 0 non-correctable ECC error. See RAMC0_C_ECC description."]
    #[inline(always)]
    pub fn ramc0_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::RAMC0_NC_ECC)
    }
    #[doc = "System SRAM 1 correctable ECC error. See RAMC0_C_ECC description."]
    #[inline(always)]
    pub fn ramc1_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::RAMC1_C_ECC)
    }
    #[doc = "System SRAM 1 non-correctable ECC error. See RAMC0_C_ECC description."]
    #[inline(always)]
    pub fn ramc1_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::RAMC1_NC_ECC)
    }
    #[doc = "System SRAM 2 correctable ECC error. See RAMC0_C_ECC description."]
    #[inline(always)]
    pub fn ramc2_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::RAMC2_C_ECC)
    }
    #[doc = "System SRAM 2 non-correctable ECC error. See RAMC0_C_ECC description."]
    #[inline(always)]
    pub fn ramc2_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::RAMC2_NC_ECC)
    }
    #[doc = "Cryptography SRAM correctable ECC error. DATA0\\[31:0\\]: Violating address. DATA1\\[6:0\\]: Syndrome of Least Significant 32-bit SRAM. DATA1\\[14:8\\]: Syndrome of Most Significant 32-bit SRAM."]
    #[inline(always)]
    pub fn crypto_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CRYPTO_C_ECC)
    }
    #[doc = "Cryptography SRAM non-correctable ECC error. See CRYPTO_C_ECC description."]
    #[inline(always)]
    pub fn crypto_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CRYPTO_NC_ECC)
    }
    #[doc = "DataWire 0 SRAM 1 correctable ECC error: DATA0\\[11:0\\]: Violating DW SRAM address (word address, assuming byte addressable). DATA1\\[6:0\\]: Syndrome of 32-bit SRAM code word."]
    #[inline(always)]
    pub fn dw0_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::DW0_C_ECC)
    }
    #[doc = "DataWire 0 SRAM 1 non-correctable ECC error. See DW0_C_ECC description."]
    #[inline(always)]
    pub fn dw0_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::DW0_NC_ECC)
    }
    #[doc = "DataWire 1 SRAM 1 correctable ECC error. See DW0_C_ECC description."]
    #[inline(always)]
    pub fn dw1_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::DW1_C_ECC)
    }
    #[doc = "DataWire 1 SRAM 1 non-correctable ECC error. See DW0_C_ECC description."]
    #[inline(always)]
    pub fn dw1_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::DW1_NC_ECC)
    }
    #[doc = "eCT Flash SRAM (for embedded operations) correctable ECC error: DATA0\\[15:0\\]: Address location in the eCT Flash SRAM. DATA1\\[6:0\\]: Syndrome of 32-bit SRAM word."]
    #[inline(always)]
    pub fn fm_sram_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FM_SRAM_C_ECC)
    }
    #[doc = "eCT Flash SRAM non-correctable ECC error: See FM_SRAM_C_ECC description."]
    #[inline(always)]
    pub fn fm_sram_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::FM_SRAM_NC_ECC)
    }
    #[doc = "CAN controller 0 MRAM correctable ECC error: DATA0\\[15:0\\]: Violating address. DATA0\\[22:16\\]: ECC violating data\\[38:32\\]
from MRAM. DATA0\\[27:24\\]: Master ID: 0-7 = CAN channel ID within mxttcanfd cluster, 8 = AHB I/F DATA1\\[31:0\\]: ECC violating data\\[31:0\\]
from MRAM."]
    #[inline(always)]
    pub fn can0_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CAN0_C_ECC)
    }
    #[doc = "CAN controller 0 MRAM non-correctable ECC error: DATA0\\[15:0\\]: Violating address. DATA0\\[22:16\\]: ECC violating data\\[38:32\\]
from MRAM (not for Address Error). DATA0\\[27:24\\]: Master ID: 0-7 = CAN channel ID within mxttcanfd cluster, 8 = AHB I/F DATA0\\[30\\]: Write access, only possible for Address Error DATA0\\[31\\]: Address Error: a CAN channel did an MRAM access above MRAM_SIZE DATA1\\[31:0\\]: ECC violating data\\[31:0\\]
from MRAM (not for Address Error)."]
    #[inline(always)]
    pub fn can0_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CAN0_NC_ECC)
    }
    #[doc = "CAN controller 1 MRAM correctable ECC error. See CAN0_C_ECC description."]
    #[inline(always)]
    pub fn can1_c_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CAN1_C_ECC)
    }
    #[doc = "CAN controller 1 MRAM non-correctable ECC error. See CAN0_NC_ECC description."]
    #[inline(always)]
    pub fn can1_nc_ecc(self) -> &'a mut W {
        self.variant(IDX_A::CAN1_NC_ECC)
    }
    #[doc = "Video Ram Protection Unit 0 fault detection. DATA0\\[31:0\\]: Violating address. DATA1\\[0\\]: User read. DATA1\\[1\\]: User write. DATA1\\[2\\]: User execute. DATA1\\[3\\]: Privileged read. DATA1\\[4\\]: Privileged write. DATA1\\[5\\]: Privileged execute. DATA1\\[6\\]: Non-secure. DATA1\\[11:8\\]: Master identifier. DATA1\\[15:12\\]: Protection context identifier. DATA1\\[31:28\\]: '0': decoder or peripheral bus error, other: undefined."]
    #[inline(always)]
    pub fn videoss_vrpu0(self) -> &'a mut W {
        self.variant(IDX_A::VIDEOSS_VRPU0)
    }
    #[doc = "Video Ram Protection Unit 1 fault detection. See VIDEOSS_VRPU0 description."]
    #[inline(always)]
    pub fn videoss_vrpu1(self) -> &'a mut W {
        self.variant(IDX_A::VIDEOSS_VRPU1)
    }
    #[doc = "Video Ram Protection Unit 2 fault detection. See VIDEOSS_VRPU0 description."]
    #[inline(always)]
    pub fn videoss_vrpu2(self) -> &'a mut W {
        self.variant(IDX_A::VIDEOSS_VRPU2)
    }
    #[doc = "Video Ram Protection Unit 3 fault detection. See VIDEOSS_VRPU0 description."]
    #[inline(always)]
    pub fn videoss_vrpu3(self) -> &'a mut W {
        self.variant(IDX_A::VIDEOSS_VRPU3)
    }
    #[doc = "Video Ram Protection Unit 4 fault detection. See VIDEOSS_VRPU0 description."]
    #[inline(always)]
    pub fn videoss_vrpu4(self) -> &'a mut W {
        self.variant(IDX_A::VIDEOSS_VRPU4)
    }
    #[doc = "Video Ram Protection Unit 5 fault detection. See VIDEOSS_VRPU0 description."]
    #[inline(always)]
    pub fn videoss_vrpu5(self) -> &'a mut W {
        self.variant(IDX_A::VIDEOSS_VRPU5)
    }
    #[doc = "SRSS Clock SuperVisor (CSV) violation detected. Multiple CSV can detect a violation at the same time. DATA0\\[15:0\\]: clk_hf* root CSV violation flags. DATA0\\[24\\]: clk_ref CSV violation flag (reference clock for clk_hf CSVs) DATA0\\[25\\]: clk_lf CSV violation flag DATA0\\[26\\]: clk_hvilo CSV violation flag"]
    #[inline(always)]
    pub fn srss_csv(self) -> &'a mut W {
        self.variant(IDX_A::SRSS_CSV)
    }
    #[doc = "SRSS Clock SuperVisor (CSV) violation detected. Multiple CSV can detect a violation at the same time. DATA0\\[0\\]: BOD on VDDA DATA\\[1\\]: OVD on VDDA DATA\\[16\\]: LVD/HVD #1 DATA0\\[17\\]: LVD/HVD #2"]
    #[inline(always)]
    pub fn srss_ssv(self) -> &'a mut W {
        self.variant(IDX_A::SRSS_SSV)
    }
    #[doc = "SRSS Multi-Counter Watch Dog Timer (MCWDT) #0 violation detected. Multiple counters can detect a violation at the same time. DATA0\\[0\\]: MCWDT subcounter 0 LOWER_LIMIT DATA0\\[1\\]: MCWDT subcounter 0 UPPER_LIMIT DATA0\\[2\\]: MCWDT subcounter 1 LOWER_LIMIT DATA0\\[3\\]: MCWDT subcounter 1 UPPER_LIMIT"]
    #[inline(always)]
    pub fn srss_mcwdt0(self) -> &'a mut W {
        self.variant(IDX_A::SRSS_MCWDT0)
    }
    #[doc = "SRSS Multi-Counter Watch Dog Timer (MCWDT) #1 violation detected. See SRSS_MCWDT0 description."]
    #[inline(always)]
    pub fn srss_mcwdt1(self) -> &'a mut W {
        self.variant(IDX_A::SRSS_MCWDT1)
    }
    #[doc = "SRSS Multi-Counter Watch Dog Timer (MCWDT) #2 violation detected. See SRSS_MCWDT0 description."]
    #[inline(always)]
    pub fn srss_mcwdt2(self) -> &'a mut W {
        self.variant(IDX_A::SRSS_MCWDT2)
    }
    #[doc = "SRSS Multi-Counter Watch Dog Timer (MCWDT) #3 violation detected. See SRSS_MCWDT0 description."]
    #[inline(always)]
    pub fn srss_mcwdt3(self) -> &'a mut W {
        self.variant(IDX_A::SRSS_MCWDT3)
    }
}
#[doc = "Field `VALID` reader - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IDX_W<0> {
        IDX_W::new(self)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
