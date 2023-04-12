#[doc = "Register `TCON_CTRL` reader"]
pub struct R(crate::R<TCON_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCON_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCON_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCON_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCON_CTRL` writer"]
pub struct W(crate::W<TCON_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCON_CTRL_SPEC>;
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
impl From<crate::W<TCON_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCON_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNELMODE` reader - Selects one of tcon operation modes, SINGLE, DUAL_INTERLEAVED or DUAL_SPLIT. If MiniLVDS operation is selected (EnLVDS = ENABLE_LVDS and LVDSMode = Mini_LVDS), tcon operates in MiniLVDS mode, indepent on the Value of channelMode. SplitPosition must be specified in MiniLVDS operation in DUAL_INTERLEAVED or DUAL_SPLIT mode, the horizontal parameter of signal generator have to set twice as they are specified in the panel-specification (panel 320, tsig_start 0, tsig_stop 320 on DUAL-Mode tsig_start 0, tsig_stop 640 ... (SplitPosition is automatically adjusted) )"]
pub type CHANNELMODE_R = crate::FieldReader<u8, CHANNELMODE_A>;
#[doc = "Selects one of tcon operation modes, SINGLE, DUAL_INTERLEAVED or DUAL_SPLIT. If MiniLVDS operation is selected (EnLVDS = ENABLE_LVDS and LVDSMode = Mini_LVDS), tcon operates in MiniLVDS mode, indepent on the Value of channelMode. SplitPosition must be specified in MiniLVDS operation in DUAL_INTERLEAVED or DUAL_SPLIT mode, the horizontal parameter of signal generator have to set twice as they are specified in the panel-specification (panel 320, tsig_start 0, tsig_stop 320 on DUAL-Mode tsig_start 0, tsig_stop 640 ... (SplitPosition is automatically adjusted) )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNELMODE_A {
    #[doc = "0: Single pixel mode. Both channels channel are active at full pixel clock. If bitmap of both panels are the same, both panels are identical"]
    SINGLE = 0,
    #[doc = "1: Dual pixel mode. Both channels are active at half the pixel clock. 1st channel drives display columns with even and 2nd one with odd index."]
    DUAL_INTERLEAVED = 1,
    #[doc = "2: Dual pixel mode. Both channels are active at half the pixel clock. 1st channel drives the left and 2nd one the righ half of the display. Note data_en is needed in this mode"]
    DUAL_SPLIT = 2,
}
impl From<CHANNELMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNELMODE_A) -> Self {
        variant as _
    }
}
impl CHANNELMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNELMODE_A> {
        match self.bits {
            0 => Some(CHANNELMODE_A::SINGLE),
            1 => Some(CHANNELMODE_A::DUAL_INTERLEAVED),
            2 => Some(CHANNELMODE_A::DUAL_SPLIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CHANNELMODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_INTERLEAVED`"]
    #[inline(always)]
    pub fn is_dual_interleaved(&self) -> bool {
        *self == CHANNELMODE_A::DUAL_INTERLEAVED
    }
    #[doc = "Checks if the value of the field is `DUAL_SPLIT`"]
    #[inline(always)]
    pub fn is_dual_split(&self) -> bool {
        *self == CHANNELMODE_A::DUAL_SPLIT
    }
}
#[doc = "Field `CHANNELMODE` writer - Selects one of tcon operation modes, SINGLE, DUAL_INTERLEAVED or DUAL_SPLIT. If MiniLVDS operation is selected (EnLVDS = ENABLE_LVDS and LVDSMode = Mini_LVDS), tcon operates in MiniLVDS mode, indepent on the Value of channelMode. SplitPosition must be specified in MiniLVDS operation in DUAL_INTERLEAVED or DUAL_SPLIT mode, the horizontal parameter of signal generator have to set twice as they are specified in the panel-specification (panel 320, tsig_start 0, tsig_stop 320 on DUAL-Mode tsig_start 0, tsig_stop 640 ... (SplitPosition is automatically adjusted) )"]
pub type CHANNELMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TCON_CTRL_SPEC, u8, CHANNELMODE_A, 2, O>;
impl<'a, const O: u8> CHANNELMODE_W<'a, O> {
    #[doc = "Single pixel mode. Both channels channel are active at full pixel clock. If bitmap of both panels are the same, both panels are identical"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CHANNELMODE_A::SINGLE)
    }
    #[doc = "Dual pixel mode. Both channels are active at half the pixel clock. 1st channel drives display columns with even and 2nd one with odd index."]
    #[inline(always)]
    pub fn dual_interleaved(self) -> &'a mut W {
        self.variant(CHANNELMODE_A::DUAL_INTERLEAVED)
    }
    #[doc = "Dual pixel mode. Both channels are active at half the pixel clock. 1st channel drives the left and 2nd one the righ half of the display. Note data_en is needed in this mode"]
    #[inline(always)]
    pub fn dual_split(self) -> &'a mut W {
        self.variant(CHANNELMODE_A::DUAL_SPLIT)
    }
}
#[doc = "Field `TCON_SYNC` reader - Select synchronization between hsync/vsync and hlast/vlast"]
pub type TCON_SYNC_R = crate::BitReader<TCON_SYNC_A>;
#[doc = "Select synchronization between hsync/vsync and hlast/vlast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON_SYNC_A {
    #[doc = "0: tcon timing generator synchronized to hlast, vlast"]
    H_VLAST = 0,
    #[doc = "1: tcon timing generator synchronized to hsync, vsync where horizontal synchronization is synchronized at the falling edge of hsync"]
    H_VSYNC = 1,
}
impl From<TCON_SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TCON_SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl TCON_SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCON_SYNC_A {
        match self.bits {
            false => TCON_SYNC_A::H_VLAST,
            true => TCON_SYNC_A::H_VSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `H_VLAST`"]
    #[inline(always)]
    pub fn is_h_vlast(&self) -> bool {
        *self == TCON_SYNC_A::H_VLAST
    }
    #[doc = "Checks if the value of the field is `H_VSYNC`"]
    #[inline(always)]
    pub fn is_h_vsync(&self) -> bool {
        *self == TCON_SYNC_A::H_VSYNC
    }
}
#[doc = "Field `TCON_SYNC` writer - Select synchronization between hsync/vsync and hlast/vlast"]
pub type TCON_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_CTRL_SPEC, TCON_SYNC_A, O>;
impl<'a, const O: u8> TCON_SYNC_W<'a, O> {
    #[doc = "tcon timing generator synchronized to hlast, vlast"]
    #[inline(always)]
    pub fn h_vlast(self) -> &'a mut W {
        self.variant(TCON_SYNC_A::H_VLAST)
    }
    #[doc = "tcon timing generator synchronized to hsync, vsync where horizontal synchronization is synchronized at the falling edge of hsync"]
    #[inline(always)]
    pub fn h_vsync(self) -> &'a mut W {
        self.variant(TCON_SYNC_A::H_VSYNC)
    }
}
#[doc = "Field `BYPASS` reader - Bypassing synchronization"]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
#[doc = "Bypassing synchronization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_A {
    #[doc = "0: tcon operation mode"]
    TCON_MODE = 0,
    #[doc = "1: tcon in Bypass mode. input pixel and its sync-signals are bypassed to tcon-output"]
    BYPASS_MODE = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::TCON_MODE,
            true => BYPASS_A::BYPASS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `TCON_MODE`"]
    #[inline(always)]
    pub fn is_tcon_mode(&self) -> bool {
        *self == BYPASS_A::TCON_MODE
    }
    #[doc = "Checks if the value of the field is `BYPASS_MODE`"]
    #[inline(always)]
    pub fn is_bypass_mode(&self) -> bool {
        *self == BYPASS_A::BYPASS_MODE
    }
}
#[doc = "Field `BYPASS` writer - Bypassing synchronization"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_CTRL_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "tcon operation mode"]
    #[inline(always)]
    pub fn tcon_mode(self) -> &'a mut W {
        self.variant(BYPASS_A::TCON_MODE)
    }
    #[doc = "tcon in Bypass mode. input pixel and its sync-signals are bypassed to tcon-output"]
    #[inline(always)]
    pub fn bypass_mode(self) -> &'a mut W {
        self.variant(BYPASS_A::BYPASS_MODE)
    }
}
#[doc = "Field `INV_CTRL` reader - Minimize the toggle rate of tcon output for display panel, that supports data inversion control. Otherwise set Inv_Ctrl = 0. Valid for all channels . Inv_Ctrl does not effect any function on LVDS-Output."]
pub type INV_CTRL_R = crate::FieldReader<u8, INV_CTRL_A>;
#[doc = "Minimize the toggle rate of tcon output for display panel, that supports data inversion control. Otherwise set Inv_Ctrl = 0. Valid for all channels . Inv_Ctrl does not effect any function on LVDS-Output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INV_CTRL_A {
    #[doc = "0: Disable inversion control"]
    DISABLE = 0,
    #[doc = "1: Enable inversion control for number of RGB-Bits = 2"]
    RGB_2_BITS = 1,
    #[doc = "2: Enable inversion control for number of RGB-Bits = 4"]
    RGB_4_BITS = 2,
    #[doc = "3: Enable inversion control for number of RGB-Bits = 6"]
    RGB_6_BITS = 3,
    #[doc = "4: Enable inversion control for number of RGB-Bits = 8"]
    RGB_8_BITS = 4,
    #[doc = "5: Enable inversion control for number of RGB-Bits = 10"]
    RGB_10_BITS = 5,
    #[doc = "6: Enable inversion control for number of RGB-Bits = 12"]
    RGB_12_BITS = 6,
    #[doc = "7: Enable inversion control for number of RGB-Bits = 14"]
    RGB_14_BITS = 7,
    #[doc = "8: Enable inversion control for number of RGB-Bits = 16"]
    RGB_16_BITS = 8,
    #[doc = "9: Enable inversion control for number of RGB-Bits = 18"]
    RGB_18_BITS = 9,
    #[doc = "10: Enable inversion control for number of RGB-Bits = 20"]
    RGB_20_BITS = 10,
    #[doc = "11: Enable inversion control for number of RGB-Bits = 22"]
    RGB_22_BITS = 11,
    #[doc = "12: Enable inversion control for number of RGB-Bits = 24"]
    RGB_24_BITS = 12,
    #[doc = "13: N/A"]
    RSVD1 = 13,
    #[doc = "14: N/A"]
    RSVD2 = 14,
    #[doc = "15: N/A"]
    RSVD3 = 15,
}
impl From<INV_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: INV_CTRL_A) -> Self {
        variant as _
    }
}
impl INV_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_CTRL_A {
        match self.bits {
            0 => INV_CTRL_A::DISABLE,
            1 => INV_CTRL_A::RGB_2_BITS,
            2 => INV_CTRL_A::RGB_4_BITS,
            3 => INV_CTRL_A::RGB_6_BITS,
            4 => INV_CTRL_A::RGB_8_BITS,
            5 => INV_CTRL_A::RGB_10_BITS,
            6 => INV_CTRL_A::RGB_12_BITS,
            7 => INV_CTRL_A::RGB_14_BITS,
            8 => INV_CTRL_A::RGB_16_BITS,
            9 => INV_CTRL_A::RGB_18_BITS,
            10 => INV_CTRL_A::RGB_20_BITS,
            11 => INV_CTRL_A::RGB_22_BITS,
            12 => INV_CTRL_A::RGB_24_BITS,
            13 => INV_CTRL_A::RSVD1,
            14 => INV_CTRL_A::RSVD2,
            15 => INV_CTRL_A::RSVD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INV_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RGB_2_BITS`"]
    #[inline(always)]
    pub fn is_rgb_2_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_2_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_4_BITS`"]
    #[inline(always)]
    pub fn is_rgb_4_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_4_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_6_BITS`"]
    #[inline(always)]
    pub fn is_rgb_6_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_6_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_8_BITS`"]
    #[inline(always)]
    pub fn is_rgb_8_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_8_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_10_BITS`"]
    #[inline(always)]
    pub fn is_rgb_10_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_10_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_12_BITS`"]
    #[inline(always)]
    pub fn is_rgb_12_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_12_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_14_BITS`"]
    #[inline(always)]
    pub fn is_rgb_14_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_14_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_16_BITS`"]
    #[inline(always)]
    pub fn is_rgb_16_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_16_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_18_BITS`"]
    #[inline(always)]
    pub fn is_rgb_18_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_18_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_20_BITS`"]
    #[inline(always)]
    pub fn is_rgb_20_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_20_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_22_BITS`"]
    #[inline(always)]
    pub fn is_rgb_22_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_22_BITS
    }
    #[doc = "Checks if the value of the field is `RGB_24_BITS`"]
    #[inline(always)]
    pub fn is_rgb_24_bits(&self) -> bool {
        *self == INV_CTRL_A::RGB_24_BITS
    }
    #[doc = "Checks if the value of the field is `RSVD1`"]
    #[inline(always)]
    pub fn is_rsvd1(&self) -> bool {
        *self == INV_CTRL_A::RSVD1
    }
    #[doc = "Checks if the value of the field is `RSVD2`"]
    #[inline(always)]
    pub fn is_rsvd2(&self) -> bool {
        *self == INV_CTRL_A::RSVD2
    }
    #[doc = "Checks if the value of the field is `RSVD3`"]
    #[inline(always)]
    pub fn is_rsvd3(&self) -> bool {
        *self == INV_CTRL_A::RSVD3
    }
}
#[doc = "Field `INV_CTRL` writer - Minimize the toggle rate of tcon output for display panel, that supports data inversion control. Otherwise set Inv_Ctrl = 0. Valid for all channels . Inv_Ctrl does not effect any function on LVDS-Output."]
pub type INV_CTRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TCON_CTRL_SPEC, u8, INV_CTRL_A, 4, O>;
impl<'a, const O: u8> INV_CTRL_W<'a, O> {
    #[doc = "Disable inversion control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INV_CTRL_A::DISABLE)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 2"]
    #[inline(always)]
    pub fn rgb_2_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_2_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 4"]
    #[inline(always)]
    pub fn rgb_4_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_4_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 6"]
    #[inline(always)]
    pub fn rgb_6_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_6_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 8"]
    #[inline(always)]
    pub fn rgb_8_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_8_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 10"]
    #[inline(always)]
    pub fn rgb_10_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_10_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 12"]
    #[inline(always)]
    pub fn rgb_12_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_12_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 14"]
    #[inline(always)]
    pub fn rgb_14_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_14_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 16"]
    #[inline(always)]
    pub fn rgb_16_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_16_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 18"]
    #[inline(always)]
    pub fn rgb_18_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_18_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 20"]
    #[inline(always)]
    pub fn rgb_20_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_20_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 22"]
    #[inline(always)]
    pub fn rgb_22_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_22_BITS)
    }
    #[doc = "Enable inversion control for number of RGB-Bits = 24"]
    #[inline(always)]
    pub fn rgb_24_bits(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RGB_24_BITS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd1(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RSVD1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd2(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RSVD2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd3(self) -> &'a mut W {
        self.variant(INV_CTRL_A::RSVD3)
    }
}
#[doc = "Field `ENLVDS` reader - Enable LVDS Mode"]
pub type ENLVDS_R = crate::BitReader<ENLVDS_A>;
#[doc = "Enable LVDS Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENLVDS_A {
    #[doc = "1: Enable LVDS , TTL and RSDS are disable"]
    ENABLE_LVDS = 1,
    #[doc = "0: Disable LVDS, Enable TTL and RSDS"]
    DISABLE_LVDS = 0,
}
impl From<ENLVDS_A> for bool {
    #[inline(always)]
    fn from(variant: ENLVDS_A) -> Self {
        variant as u8 != 0
    }
}
impl ENLVDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENLVDS_A {
        match self.bits {
            true => ENLVDS_A::ENABLE_LVDS,
            false => ENLVDS_A::DISABLE_LVDS,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_LVDS`"]
    #[inline(always)]
    pub fn is_enable_lvds(&self) -> bool {
        *self == ENLVDS_A::ENABLE_LVDS
    }
    #[doc = "Checks if the value of the field is `DISABLE_LVDS`"]
    #[inline(always)]
    pub fn is_disable_lvds(&self) -> bool {
        *self == ENLVDS_A::DISABLE_LVDS
    }
}
#[doc = "Field `ENLVDS` writer - Enable LVDS Mode"]
pub type ENLVDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_CTRL_SPEC, ENLVDS_A, O>;
impl<'a, const O: u8> ENLVDS_W<'a, O> {
    #[doc = "Enable LVDS , TTL and RSDS are disable"]
    #[inline(always)]
    pub fn enable_lvds(self) -> &'a mut W {
        self.variant(ENLVDS_A::ENABLE_LVDS)
    }
    #[doc = "Disable LVDS, Enable TTL and RSDS"]
    #[inline(always)]
    pub fn disable_lvds(self) -> &'a mut W {
        self.variant(ENLVDS_A::DISABLE_LVDS)
    }
}
#[doc = "Field `LVDSMODE` reader - Selection the LVDS Mode if EnLVDS = ENABLE_LVDS"]
pub type LVDSMODE_R = crate::BitReader<LVDSMODE_A>;
#[doc = "Selection the LVDS Mode if EnLVDS = ENABLE_LVDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDSMODE_A {
    #[doc = "1: MiniLVDS"]
    MINI_LVDS = 1,
    #[doc = "0: LVDS Mode, refered to OpenLDI"]
    LVDS = 0,
}
impl From<LVDSMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDSMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDSMODE_A {
        match self.bits {
            true => LVDSMODE_A::MINI_LVDS,
            false => LVDSMODE_A::LVDS,
        }
    }
    #[doc = "Checks if the value of the field is `MINI_LVDS`"]
    #[inline(always)]
    pub fn is_mini_lvds(&self) -> bool {
        *self == LVDSMODE_A::MINI_LVDS
    }
    #[doc = "Checks if the value of the field is `LVDS`"]
    #[inline(always)]
    pub fn is_lvds(&self) -> bool {
        *self == LVDSMODE_A::LVDS
    }
}
#[doc = "Field `LVDSMODE` writer - Selection the LVDS Mode if EnLVDS = ENABLE_LVDS"]
pub type LVDSMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_CTRL_SPEC, LVDSMODE_A, O>;
impl<'a, const O: u8> LVDSMODE_W<'a, O> {
    #[doc = "MiniLVDS"]
    #[inline(always)]
    pub fn mini_lvds(self) -> &'a mut W {
        self.variant(LVDSMODE_A::MINI_LVDS)
    }
    #[doc = "LVDS Mode, refered to OpenLDI"]
    #[inline(always)]
    pub fn lvds(self) -> &'a mut W {
        self.variant(LVDSMODE_A::LVDS)
    }
}
#[doc = "Field `LVDS_BALANCE` reader - Operation mode of LVDS-OpenLDI"]
pub type LVDS_BALANCE_R = crate::BitReader<LVDS_BALANCE_A>;
#[doc = "Operation mode of LVDS-OpenLDI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDS_BALANCE_A {
    #[doc = "1: LVDS operates in 24 bits Balanced Mode"]
    BALANCED = 1,
    #[doc = "0: LVDS operates in 24 bits Unbalanced Mode"]
    UNBALANCED = 0,
}
impl From<LVDS_BALANCE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_BALANCE_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDS_BALANCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_BALANCE_A {
        match self.bits {
            true => LVDS_BALANCE_A::BALANCED,
            false => LVDS_BALANCE_A::UNBALANCED,
        }
    }
    #[doc = "Checks if the value of the field is `BALANCED`"]
    #[inline(always)]
    pub fn is_balanced(&self) -> bool {
        *self == LVDS_BALANCE_A::BALANCED
    }
    #[doc = "Checks if the value of the field is `UNBALANCED`"]
    #[inline(always)]
    pub fn is_unbalanced(&self) -> bool {
        *self == LVDS_BALANCE_A::UNBALANCED
    }
}
#[doc = "Field `LVDS_BALANCE` writer - Operation mode of LVDS-OpenLDI"]
pub type LVDS_BALANCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TCON_CTRL_SPEC, LVDS_BALANCE_A, O>;
impl<'a, const O: u8> LVDS_BALANCE_W<'a, O> {
    #[doc = "LVDS operates in 24 bits Balanced Mode"]
    #[inline(always)]
    pub fn balanced(self) -> &'a mut W {
        self.variant(LVDS_BALANCE_A::BALANCED)
    }
    #[doc = "LVDS operates in 24 bits Unbalanced Mode"]
    #[inline(always)]
    pub fn unbalanced(self) -> &'a mut W {
        self.variant(LVDS_BALANCE_A::UNBALANCED)
    }
}
#[doc = "Field `LVDS_CLOCK_INV` reader - Inversion the polatity of lvds clock in OpenLDI Mode"]
pub type LVDS_CLOCK_INV_R = crate::BitReader<LVDS_CLOCK_INV_A>;
#[doc = "Inversion the polatity of lvds clock in OpenLDI Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDS_CLOCK_INV_A {
    #[doc = "1: Invert LVDS Clock"]
    INV = 1,
    #[doc = "0: NON-Invert LVDS Clock"]
    NON_INV = 0,
}
impl From<LVDS_CLOCK_INV_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_CLOCK_INV_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDS_CLOCK_INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_CLOCK_INV_A {
        match self.bits {
            true => LVDS_CLOCK_INV_A::INV,
            false => LVDS_CLOCK_INV_A::NON_INV,
        }
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == LVDS_CLOCK_INV_A::INV
    }
    #[doc = "Checks if the value of the field is `NON_INV`"]
    #[inline(always)]
    pub fn is_non_inv(&self) -> bool {
        *self == LVDS_CLOCK_INV_A::NON_INV
    }
}
#[doc = "Field `LVDS_CLOCK_INV` writer - Inversion the polatity of lvds clock in OpenLDI Mode"]
pub type LVDS_CLOCK_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TCON_CTRL_SPEC, LVDS_CLOCK_INV_A, O>;
impl<'a, const O: u8> LVDS_CLOCK_INV_W<'a, O> {
    #[doc = "Invert LVDS Clock"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(LVDS_CLOCK_INV_A::INV)
    }
    #[doc = "NON-Invert LVDS Clock"]
    #[inline(always)]
    pub fn non_inv(self) -> &'a mut W {
        self.variant(LVDS_CLOCK_INV_A::NON_INV)
    }
}
#[doc = "Field `MINILVDS_OPCODE` reader - Operation mode of MiniLVDS"]
pub type MINILVDS_OPCODE_R = crate::FieldReader<u8, MINILVDS_OPCODE_A>;
#[doc = "Operation mode of MiniLVDS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MINILVDS_OPCODE_A {
    #[doc = "0: MiniLVDS operates in 6 and 8 bit data, three pairs"]
    MODE_3PAIRS = 0,
    #[doc = "1: Not Implemented"]
    MODE_4PAIRS = 1,
    #[doc = "2: Not Implemented"]
    MODE_5PAIRS = 2,
    #[doc = "3: MiniLVDS operates in 6 and 8 bit data, six pairs"]
    MODE_6PAIRS = 3,
    #[doc = "4: N/A"]
    RSVD1 = 4,
    #[doc = "5: N/A"]
    RSVD2 = 5,
    #[doc = "6: N/A"]
    RSVD3 = 6,
    #[doc = "7: N/A"]
    RSVD4 = 7,
}
impl From<MINILVDS_OPCODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MINILVDS_OPCODE_A) -> Self {
        variant as _
    }
}
impl MINILVDS_OPCODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINILVDS_OPCODE_A {
        match self.bits {
            0 => MINILVDS_OPCODE_A::MODE_3PAIRS,
            1 => MINILVDS_OPCODE_A::MODE_4PAIRS,
            2 => MINILVDS_OPCODE_A::MODE_5PAIRS,
            3 => MINILVDS_OPCODE_A::MODE_6PAIRS,
            4 => MINILVDS_OPCODE_A::RSVD1,
            5 => MINILVDS_OPCODE_A::RSVD2,
            6 => MINILVDS_OPCODE_A::RSVD3,
            7 => MINILVDS_OPCODE_A::RSVD4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_3PAIRS`"]
    #[inline(always)]
    pub fn is_mode_3pairs(&self) -> bool {
        *self == MINILVDS_OPCODE_A::MODE_3PAIRS
    }
    #[doc = "Checks if the value of the field is `MODE_4PAIRS`"]
    #[inline(always)]
    pub fn is_mode_4pairs(&self) -> bool {
        *self == MINILVDS_OPCODE_A::MODE_4PAIRS
    }
    #[doc = "Checks if the value of the field is `MODE_5PAIRS`"]
    #[inline(always)]
    pub fn is_mode_5pairs(&self) -> bool {
        *self == MINILVDS_OPCODE_A::MODE_5PAIRS
    }
    #[doc = "Checks if the value of the field is `MODE_6PAIRS`"]
    #[inline(always)]
    pub fn is_mode_6pairs(&self) -> bool {
        *self == MINILVDS_OPCODE_A::MODE_6PAIRS
    }
    #[doc = "Checks if the value of the field is `RSVD1`"]
    #[inline(always)]
    pub fn is_rsvd1(&self) -> bool {
        *self == MINILVDS_OPCODE_A::RSVD1
    }
    #[doc = "Checks if the value of the field is `RSVD2`"]
    #[inline(always)]
    pub fn is_rsvd2(&self) -> bool {
        *self == MINILVDS_OPCODE_A::RSVD2
    }
    #[doc = "Checks if the value of the field is `RSVD3`"]
    #[inline(always)]
    pub fn is_rsvd3(&self) -> bool {
        *self == MINILVDS_OPCODE_A::RSVD3
    }
    #[doc = "Checks if the value of the field is `RSVD4`"]
    #[inline(always)]
    pub fn is_rsvd4(&self) -> bool {
        *self == MINILVDS_OPCODE_A::RSVD4
    }
}
#[doc = "Field `MINILVDS_OPCODE` writer - Operation mode of MiniLVDS"]
pub type MINILVDS_OPCODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TCON_CTRL_SPEC, u8, MINILVDS_OPCODE_A, 3, O>;
impl<'a, const O: u8> MINILVDS_OPCODE_W<'a, O> {
    #[doc = "MiniLVDS operates in 6 and 8 bit data, three pairs"]
    #[inline(always)]
    pub fn mode_3pairs(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::MODE_3PAIRS)
    }
    #[doc = "Not Implemented"]
    #[inline(always)]
    pub fn mode_4pairs(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::MODE_4PAIRS)
    }
    #[doc = "Not Implemented"]
    #[inline(always)]
    pub fn mode_5pairs(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::MODE_5PAIRS)
    }
    #[doc = "MiniLVDS operates in 6 and 8 bit data, six pairs"]
    #[inline(always)]
    pub fn mode_6pairs(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::MODE_6PAIRS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd1(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::RSVD1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd2(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::RSVD2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd3(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::RSVD3)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd4(self) -> &'a mut W {
        self.variant(MINILVDS_OPCODE_A::RSVD4)
    }
}
#[doc = "Field `DUAL_SWAP` reader - pixels of lower/upper channel can be swapped if tcon operates in DUAL-mode (include LVDS/miniLVDS) no effect in SINGLE-mode"]
pub type DUAL_SWAP_R = crate::BitReader<DUAL_SWAP_A>;
#[doc = "pixels of lower/upper channel can be swapped if tcon operates in DUAL-mode (include LVDS/miniLVDS) no effect in SINGLE-mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUAL_SWAP_A {
    #[doc = "1: swapping pixels between lower-channel and upper-channel"]
    SWAP = 1,
    #[doc = "0: NON-swapping pixels between lower-channel and upper-channel"]
    NON_SWAP = 0,
}
impl From<DUAL_SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: DUAL_SWAP_A) -> Self {
        variant as u8 != 0
    }
}
impl DUAL_SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUAL_SWAP_A {
        match self.bits {
            true => DUAL_SWAP_A::SWAP,
            false => DUAL_SWAP_A::NON_SWAP,
        }
    }
    #[doc = "Checks if the value of the field is `SWAP`"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == DUAL_SWAP_A::SWAP
    }
    #[doc = "Checks if the value of the field is `NON_SWAP`"]
    #[inline(always)]
    pub fn is_non_swap(&self) -> bool {
        *self == DUAL_SWAP_A::NON_SWAP
    }
}
#[doc = "Field `DUAL_SWAP` writer - pixels of lower/upper channel can be swapped if tcon operates in DUAL-mode (include LVDS/miniLVDS) no effect in SINGLE-mode"]
pub type DUAL_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_CTRL_SPEC, DUAL_SWAP_A, O>;
impl<'a, const O: u8> DUAL_SWAP_W<'a, O> {
    #[doc = "swapping pixels between lower-channel and upper-channel"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut W {
        self.variant(DUAL_SWAP_A::SWAP)
    }
    #[doc = "NON-swapping pixels between lower-channel and upper-channel"]
    #[inline(always)]
    pub fn non_swap(self) -> &'a mut W {
        self.variant(DUAL_SWAP_A::NON_SWAP)
    }
}
#[doc = "Field `SPLITPOSITION` reader - Index of first column of right display half when ChannelMode is DUAL_SPLIT. - SplitPosition must be less or equal 1280 - (Hact - SplitPosition) must be less or equal 1280 - If (SplitPosition greater than (Hact - SplitPosition)) Htotal greather 2*SplitPosition else Htotal greather (Hact - SplitPosition) - NOTE once setting SplitPosition data_en is needed"]
pub type SPLITPOSITION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPLITPOSITION` writer - Index of first column of right display half when ChannelMode is DUAL_SPLIT. - SplitPosition must be less or equal 1280 - (Hact - SplitPosition) must be less or equal 1280 - If (SplitPosition greater than (Hact - SplitPosition)) Htotal greather 2*SplitPosition else Htotal greather (Hact - SplitPosition) - NOTE once setting SplitPosition data_en is needed"]
pub type SPLITPOSITION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TCON_CTRL_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:1 - Selects one of tcon operation modes, SINGLE, DUAL_INTERLEAVED or DUAL_SPLIT. If MiniLVDS operation is selected (EnLVDS = ENABLE_LVDS and LVDSMode = Mini_LVDS), tcon operates in MiniLVDS mode, indepent on the Value of channelMode. SplitPosition must be specified in MiniLVDS operation in DUAL_INTERLEAVED or DUAL_SPLIT mode, the horizontal parameter of signal generator have to set twice as they are specified in the panel-specification (panel 320, tsig_start 0, tsig_stop 320 on DUAL-Mode tsig_start 0, tsig_stop 640 ... (SplitPosition is automatically adjusted) )"]
    #[inline(always)]
    pub fn channelmode(&self) -> CHANNELMODE_R {
        CHANNELMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Select synchronization between hsync/vsync and hlast/vlast"]
    #[inline(always)]
    pub fn tcon_sync(&self) -> TCON_SYNC_R {
        TCON_SYNC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bypassing synchronization"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Minimize the toggle rate of tcon output for display panel, that supports data inversion control. Otherwise set Inv_Ctrl = 0. Valid for all channels . Inv_Ctrl does not effect any function on LVDS-Output."]
    #[inline(always)]
    pub fn inv_ctrl(&self) -> INV_CTRL_R {
        INV_CTRL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable LVDS Mode"]
    #[inline(always)]
    pub fn enlvds(&self) -> ENLVDS_R {
        ENLVDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selection the LVDS Mode if EnLVDS = ENABLE_LVDS"]
    #[inline(always)]
    pub fn lvdsmode(&self) -> LVDSMODE_R {
        LVDSMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Operation mode of LVDS-OpenLDI"]
    #[inline(always)]
    pub fn lvds_balance(&self) -> LVDS_BALANCE_R {
        LVDS_BALANCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Inversion the polatity of lvds clock in OpenLDI Mode"]
    #[inline(always)]
    pub fn lvds_clock_inv(&self) -> LVDS_CLOCK_INV_R {
        LVDS_CLOCK_INV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Operation mode of MiniLVDS"]
    #[inline(always)]
    pub fn minilvds_opcode(&self) -> MINILVDS_OPCODE_R {
        MINILVDS_OPCODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - pixels of lower/upper channel can be swapped if tcon operates in DUAL-mode (include LVDS/miniLVDS) no effect in SINGLE-mode"]
    #[inline(always)]
    pub fn dual_swap(&self) -> DUAL_SWAP_R {
        DUAL_SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Index of first column of right display half when ChannelMode is DUAL_SPLIT. - SplitPosition must be less or equal 1280 - (Hact - SplitPosition) must be less or equal 1280 - If (SplitPosition greater than (Hact - SplitPosition)) Htotal greather 2*SplitPosition else Htotal greather (Hact - SplitPosition) - NOTE once setting SplitPosition data_en is needed"]
    #[inline(always)]
    pub fn splitposition(&self) -> SPLITPOSITION_R {
        SPLITPOSITION_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects one of tcon operation modes, SINGLE, DUAL_INTERLEAVED or DUAL_SPLIT. If MiniLVDS operation is selected (EnLVDS = ENABLE_LVDS and LVDSMode = Mini_LVDS), tcon operates in MiniLVDS mode, indepent on the Value of channelMode. SplitPosition must be specified in MiniLVDS operation in DUAL_INTERLEAVED or DUAL_SPLIT mode, the horizontal parameter of signal generator have to set twice as they are specified in the panel-specification (panel 320, tsig_start 0, tsig_stop 320 on DUAL-Mode tsig_start 0, tsig_stop 640 ... (SplitPosition is automatically adjusted) )"]
    #[inline(always)]
    #[must_use]
    pub fn channelmode(&mut self) -> CHANNELMODE_W<0> {
        CHANNELMODE_W::new(self)
    }
    #[doc = "Bit 2 - Select synchronization between hsync/vsync and hlast/vlast"]
    #[inline(always)]
    #[must_use]
    pub fn tcon_sync(&mut self) -> TCON_SYNC_W<2> {
        TCON_SYNC_W::new(self)
    }
    #[doc = "Bit 3 - Bypassing synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<3> {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 4:7 - Minimize the toggle rate of tcon output for display panel, that supports data inversion control. Otherwise set Inv_Ctrl = 0. Valid for all channels . Inv_Ctrl does not effect any function on LVDS-Output."]
    #[inline(always)]
    #[must_use]
    pub fn inv_ctrl(&mut self) -> INV_CTRL_W<4> {
        INV_CTRL_W::new(self)
    }
    #[doc = "Bit 8 - Enable LVDS Mode"]
    #[inline(always)]
    #[must_use]
    pub fn enlvds(&mut self) -> ENLVDS_W<8> {
        ENLVDS_W::new(self)
    }
    #[doc = "Bit 9 - Selection the LVDS Mode if EnLVDS = ENABLE_LVDS"]
    #[inline(always)]
    #[must_use]
    pub fn lvdsmode(&mut self) -> LVDSMODE_W<9> {
        LVDSMODE_W::new(self)
    }
    #[doc = "Bit 10 - Operation mode of LVDS-OpenLDI"]
    #[inline(always)]
    #[must_use]
    pub fn lvds_balance(&mut self) -> LVDS_BALANCE_W<10> {
        LVDS_BALANCE_W::new(self)
    }
    #[doc = "Bit 11 - Inversion the polatity of lvds clock in OpenLDI Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lvds_clock_inv(&mut self) -> LVDS_CLOCK_INV_W<11> {
        LVDS_CLOCK_INV_W::new(self)
    }
    #[doc = "Bits 12:14 - Operation mode of MiniLVDS"]
    #[inline(always)]
    #[must_use]
    pub fn minilvds_opcode(&mut self) -> MINILVDS_OPCODE_W<12> {
        MINILVDS_OPCODE_W::new(self)
    }
    #[doc = "Bit 15 - pixels of lower/upper channel can be swapped if tcon operates in DUAL-mode (include LVDS/miniLVDS) no effect in SINGLE-mode"]
    #[inline(always)]
    #[must_use]
    pub fn dual_swap(&mut self) -> DUAL_SWAP_W<15> {
        DUAL_SWAP_W::new(self)
    }
    #[doc = "Bits 16:29 - Index of first column of right display half when ChannelMode is DUAL_SPLIT. - SplitPosition must be less or equal 1280 - (Hact - SplitPosition) must be less or equal 1280 - If (SplitPosition greater than (Hact - SplitPosition)) Htotal greather 2*SplitPosition else Htotal greather (Hact - SplitPosition) - NOTE once setting SplitPosition data_en is needed"]
    #[inline(always)]
    #[must_use]
    pub fn splitposition(&mut self) -> SPLITPOSITION_W<16> {
        SPLITPOSITION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcon_ctrl](index.html) module"]
pub struct TCON_CTRL_SPEC;
impl crate::RegisterSpec for TCON_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcon_ctrl::R](R) reader structure"]
impl crate::Readable for TCON_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcon_ctrl::W](W) writer structure"]
impl crate::Writable for TCON_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCON_CTRL to value 0x0140_1408"]
impl crate::Resettable for TCON_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0140_1408;
}
