#[doc = "Register `POST_CTL` reader"]
pub struct R(crate::R<POST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POST_CTL` writer"]
pub struct W(crate::W<POST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POST_CTL_SPEC>;
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
impl From<crate::W<POST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POST_PROC` reader - Post processing"]
pub type POST_PROC_R = crate::FieldReader<u8, POST_PROC_A>;
#[doc = "Post processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POST_PROC_A {
    #[doc = "0: No postprocessing"]
    NONE = 0,
    #[doc = "1: Averaging"]
    AVG = 1,
    #[doc = "2: Averaging followed by Range detect"]
    AVG_RANGE = 2,
    #[doc = "3: Range detect"]
    RANGE = 3,
    #[doc = "4: Range detect followed by pulse detect"]
    RANGE_PULSE = 4,
    #[doc = "5: N/A"]
    RSVD0 = 5,
    #[doc = "6: N/A"]
    RSVD1 = 6,
    #[doc = "7: N/A"]
    RSVD2 = 7,
}
impl From<POST_PROC_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_PROC_A) -> Self {
        variant as _
    }
}
impl POST_PROC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POST_PROC_A {
        match self.bits {
            0 => POST_PROC_A::NONE,
            1 => POST_PROC_A::AVG,
            2 => POST_PROC_A::AVG_RANGE,
            3 => POST_PROC_A::RANGE,
            4 => POST_PROC_A::RANGE_PULSE,
            5 => POST_PROC_A::RSVD0,
            6 => POST_PROC_A::RSVD1,
            7 => POST_PROC_A::RSVD2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == POST_PROC_A::NONE
    }
    #[doc = "Checks if the value of the field is `AVG`"]
    #[inline(always)]
    pub fn is_avg(&self) -> bool {
        *self == POST_PROC_A::AVG
    }
    #[doc = "Checks if the value of the field is `AVG_RANGE`"]
    #[inline(always)]
    pub fn is_avg_range(&self) -> bool {
        *self == POST_PROC_A::AVG_RANGE
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == POST_PROC_A::RANGE
    }
    #[doc = "Checks if the value of the field is `RANGE_PULSE`"]
    #[inline(always)]
    pub fn is_range_pulse(&self) -> bool {
        *self == POST_PROC_A::RANGE_PULSE
    }
    #[doc = "Checks if the value of the field is `RSVD0`"]
    #[inline(always)]
    pub fn is_rsvd0(&self) -> bool {
        *self == POST_PROC_A::RSVD0
    }
    #[doc = "Checks if the value of the field is `RSVD1`"]
    #[inline(always)]
    pub fn is_rsvd1(&self) -> bool {
        *self == POST_PROC_A::RSVD1
    }
    #[doc = "Checks if the value of the field is `RSVD2`"]
    #[inline(always)]
    pub fn is_rsvd2(&self) -> bool {
        *self == POST_PROC_A::RSVD2
    }
}
#[doc = "Field `POST_PROC` writer - Post processing"]
pub type POST_PROC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, POST_CTL_SPEC, u8, POST_PROC_A, 3, O>;
impl<'a, const O: u8> POST_PROC_W<'a, O> {
    #[doc = "No postprocessing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(POST_PROC_A::NONE)
    }
    #[doc = "Averaging"]
    #[inline(always)]
    pub fn avg(self) -> &'a mut W {
        self.variant(POST_PROC_A::AVG)
    }
    #[doc = "Averaging followed by Range detect"]
    #[inline(always)]
    pub fn avg_range(self) -> &'a mut W {
        self.variant(POST_PROC_A::AVG_RANGE)
    }
    #[doc = "Range detect"]
    #[inline(always)]
    pub fn range(self) -> &'a mut W {
        self.variant(POST_PROC_A::RANGE)
    }
    #[doc = "Range detect followed by pulse detect"]
    #[inline(always)]
    pub fn range_pulse(self) -> &'a mut W {
        self.variant(POST_PROC_A::RANGE_PULSE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd0(self) -> &'a mut W {
        self.variant(POST_PROC_A::RSVD0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd1(self) -> &'a mut W {
        self.variant(POST_PROC_A::RSVD1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd2(self) -> &'a mut W {
        self.variant(POST_PROC_A::RSVD2)
    }
}
#[doc = "Field `LEFT_ALIGN` reader - Left or right align data in result\\[15:0\\]. 0: the data is right aligned in result\\[11:0\\], with sign extension to 16 bits if enabled 1: the data is left aligned in result\\[15:4\\]
with the lower nibble 0. Caveat if the result was more than 12 bits (e.g. after averaging) then the bits above 12 will be discarded."]
pub type LEFT_ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `LEFT_ALIGN` writer - Left or right align data in result\\[15:0\\]. 0: the data is right aligned in result\\[11:0\\], with sign extension to 16 bits if enabled 1: the data is left aligned in result\\[15:4\\]
with the lower nibble 0. Caveat if the result was more than 12 bits (e.g. after averaging) then the bits above 12 will be discarded."]
pub type LEFT_ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POST_CTL_SPEC, bool, O>;
#[doc = "Field `SIGN_EXT` reader - Output data is sign extended"]
pub type SIGN_EXT_R = crate::BitReader<SIGN_EXT_A>;
#[doc = "Output data is sign extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGN_EXT_A {
    #[doc = "0: Default: result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "1: Result data is signed (sign extended if needed)"]
    SIGNED = 1,
}
impl From<SIGN_EXT_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_EXT_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGN_EXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGN_EXT_A {
        match self.bits {
            false => SIGN_EXT_A::UNSIGNED,
            true => SIGN_EXT_A::SIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED`"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == SIGN_EXT_A::UNSIGNED
    }
    #[doc = "Checks if the value of the field is `SIGNED`"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == SIGN_EXT_A::SIGNED
    }
}
#[doc = "Field `SIGN_EXT` writer - Output data is sign extended"]
pub type SIGN_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, POST_CTL_SPEC, SIGN_EXT_A, O>;
impl<'a, const O: u8> SIGN_EXT_W<'a, O> {
    #[doc = "Default: result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut W {
        self.variant(SIGN_EXT_A::UNSIGNED)
    }
    #[doc = "Result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut W {
        self.variant(SIGN_EXT_A::SIGNED)
    }
}
#[doc = "Field `AVG_CNT` reader - Either averaging count (minus 1) or Pulse positive reload value Averaging Count for channels that have averaging enabled. A channel will be sampled (AVG_CNT+1) = \\[1..256\\]
times. The signal will be acquired back to back (1st order accumulate and dump filter), the average result is calculated and stored and then the next enabled channel is sampled. If more than 16 sample are taken (AVG_CNT>=16) then AVG_SHIFT must be set so that the result after shifting fits in 16 bits Pulse detect positive reload value PULSE_POS_RL\\[7:0\\]"]
pub type AVG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVG_CNT` writer - Either averaging count (minus 1) or Pulse positive reload value Averaging Count for channels that have averaging enabled. A channel will be sampled (AVG_CNT+1) = \\[1..256\\]
times. The signal will be acquired back to back (1st order accumulate and dump filter), the average result is calculated and stored and then the next enabled channel is sampled. If more than 16 sample are taken (AVG_CNT>=16) then AVG_SHIFT must be set so that the result after shifting fits in 16 bits Pulse detect positive reload value PULSE_POS_RL\\[7:0\\]"]
pub type AVG_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POST_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `SHIFT_R` reader - Either Shift Right (no pulse detection) or Pulse negative reload value (if pulse detection is enabled) Shift right SHIFT_R\\[3:0\\]
= \\[0..12\\]: the result (typically after averaging) is shifted right as specified here. Software has to make sure that the result fits in less than 16 bits. Any value >12 will be treated as 12, bit \\[4\\]
is always ignored. This can also be used to fit the 12-bit result in 8 bits. Pulse detect negative reload value PULSE_NEG_RL\\[4:0\\]"]
pub type SHIFT_R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHIFT_R` writer - Either Shift Right (no pulse detection) or Pulse negative reload value (if pulse detection is enabled) Shift right SHIFT_R\\[3:0\\]
= \\[0..12\\]: the result (typically after averaging) is shifted right as specified here. Software has to make sure that the result fits in less than 16 bits. Any value >12 will be treated as 12, bit \\[4\\]
is always ignored. This can also be used to fit the 12-bit result in 8 bits. Pulse detect negative reload value PULSE_NEG_RL\\[4:0\\]"]
pub type SHIFT_R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POST_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `RANGE_MODE` reader - Range detect mode"]
pub type RANGE_MODE_R = crate::FieldReader<u8, RANGE_MODE_A>;
#[doc = "Range detect mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGE_MODE_A {
    #[doc = "0: Below Low threshold (result &lt; Lo)"]
    BELOW_LO = 0,
    #[doc = "1: Inside range (Lo &lt;= result &lt; Hi)"]
    INSIDE_RANGE = 1,
    #[doc = "2: Above high threshold (Hi &lt;= result)"]
    ABOVE_HI = 2,
    #[doc = "3: Outside range (result &lt; Lo || Hi &lt;= result)"]
    OUTSIDE_RANGE = 3,
}
impl From<RANGE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_MODE_A) -> Self {
        variant as _
    }
}
impl RANGE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGE_MODE_A {
        match self.bits {
            0 => RANGE_MODE_A::BELOW_LO,
            1 => RANGE_MODE_A::INSIDE_RANGE,
            2 => RANGE_MODE_A::ABOVE_HI,
            3 => RANGE_MODE_A::OUTSIDE_RANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BELOW_LO`"]
    #[inline(always)]
    pub fn is_below_lo(&self) -> bool {
        *self == RANGE_MODE_A::BELOW_LO
    }
    #[doc = "Checks if the value of the field is `INSIDE_RANGE`"]
    #[inline(always)]
    pub fn is_inside_range(&self) -> bool {
        *self == RANGE_MODE_A::INSIDE_RANGE
    }
    #[doc = "Checks if the value of the field is `ABOVE_HI`"]
    #[inline(always)]
    pub fn is_above_hi(&self) -> bool {
        *self == RANGE_MODE_A::ABOVE_HI
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_RANGE`"]
    #[inline(always)]
    pub fn is_outside_range(&self) -> bool {
        *self == RANGE_MODE_A::OUTSIDE_RANGE
    }
}
#[doc = "Field `RANGE_MODE` writer - Range detect mode"]
pub type RANGE_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, POST_CTL_SPEC, u8, RANGE_MODE_A, 2, O>;
impl<'a, const O: u8> RANGE_MODE_W<'a, O> {
    #[doc = "Below Low threshold (result &lt; Lo)"]
    #[inline(always)]
    pub fn below_lo(self) -> &'a mut W {
        self.variant(RANGE_MODE_A::BELOW_LO)
    }
    #[doc = "Inside range (Lo &lt;= result &lt; Hi)"]
    #[inline(always)]
    pub fn inside_range(self) -> &'a mut W {
        self.variant(RANGE_MODE_A::INSIDE_RANGE)
    }
    #[doc = "Above high threshold (Hi &lt;= result)"]
    #[inline(always)]
    pub fn above_hi(self) -> &'a mut W {
        self.variant(RANGE_MODE_A::ABOVE_HI)
    }
    #[doc = "Outside range (result &lt; Lo || Hi &lt;= result)"]
    #[inline(always)]
    pub fn outside_range(self) -> &'a mut W {
        self.variant(RANGE_MODE_A::OUTSIDE_RANGE)
    }
}
#[doc = "Field `TR_DONE_GRP_VIO` reader - Select tr_sar_ch_done mode for last channel of a group, ignored for all other channels Also see TR_CTL.DONE_LEVEL"]
pub type TR_DONE_GRP_VIO_R = crate::BitReader<TR_DONE_GRP_VIO_A>;
#[doc = "Select tr_sar_ch_done mode for last channel of a group, ignored for all other channels Also see TR_CTL.DONE_LEVEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR_DONE_GRP_VIO_A {
    #[doc = "0: Default: tr_sar_ch_done is set when the group is done"]
    DONE = 0,
    #[doc = "1: tr_sar_ch_done is only set if any of the channels in the group has a Range Violation. This mode is ignored if this is not the last channel in the group. Note that if none of the channels in the group have Range detection enabled then the trigger will never get set."]
    GRP_RANGE_VIO = 1,
}
impl From<TR_DONE_GRP_VIO_A> for bool {
    #[inline(always)]
    fn from(variant: TR_DONE_GRP_VIO_A) -> Self {
        variant as u8 != 0
    }
}
impl TR_DONE_GRP_VIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR_DONE_GRP_VIO_A {
        match self.bits {
            false => TR_DONE_GRP_VIO_A::DONE,
            true => TR_DONE_GRP_VIO_A::GRP_RANGE_VIO,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == TR_DONE_GRP_VIO_A::DONE
    }
    #[doc = "Checks if the value of the field is `GRP_RANGE_VIO`"]
    #[inline(always)]
    pub fn is_grp_range_vio(&self) -> bool {
        *self == TR_DONE_GRP_VIO_A::GRP_RANGE_VIO
    }
}
#[doc = "Field `TR_DONE_GRP_VIO` writer - Select tr_sar_ch_done mode for last channel of a group, ignored for all other channels Also see TR_CTL.DONE_LEVEL"]
pub type TR_DONE_GRP_VIO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, POST_CTL_SPEC, TR_DONE_GRP_VIO_A, O>;
impl<'a, const O: u8> TR_DONE_GRP_VIO_W<'a, O> {
    #[doc = "Default: tr_sar_ch_done is set when the group is done"]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(TR_DONE_GRP_VIO_A::DONE)
    }
    #[doc = "tr_sar_ch_done is only set if any of the channels in the group has a Range Violation. This mode is ignored if this is not the last channel in the group. Note that if none of the channels in the group have Range detection enabled then the trigger will never get set."]
    #[inline(always)]
    pub fn grp_range_vio(self) -> &'a mut W {
        self.variant(TR_DONE_GRP_VIO_A::GRP_RANGE_VIO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Post processing"]
    #[inline(always)]
    pub fn post_proc(&self) -> POST_PROC_R {
        POST_PROC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Left or right align data in result\\[15:0\\]. 0: the data is right aligned in result\\[11:0\\], with sign extension to 16 bits if enabled 1: the data is left aligned in result\\[15:4\\]
with the lower nibble 0. Caveat if the result was more than 12 bits (e.g. after averaging) then the bits above 12 will be discarded."]
    #[inline(always)]
    pub fn left_align(&self) -> LEFT_ALIGN_R {
        LEFT_ALIGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output data is sign extended"]
    #[inline(always)]
    pub fn sign_ext(&self) -> SIGN_EXT_R {
        SIGN_EXT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Either averaging count (minus 1) or Pulse positive reload value Averaging Count for channels that have averaging enabled. A channel will be sampled (AVG_CNT+1) = \\[1..256\\]
times. The signal will be acquired back to back (1st order accumulate and dump filter), the average result is calculated and stored and then the next enabled channel is sampled. If more than 16 sample are taken (AVG_CNT>=16) then AVG_SHIFT must be set so that the result after shifting fits in 16 bits Pulse detect positive reload value PULSE_POS_RL\\[7:0\\]"]
    #[inline(always)]
    pub fn avg_cnt(&self) -> AVG_CNT_R {
        AVG_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Either Shift Right (no pulse detection) or Pulse negative reload value (if pulse detection is enabled) Shift right SHIFT_R\\[3:0\\]
= \\[0..12\\]: the result (typically after averaging) is shifted right as specified here. Software has to make sure that the result fits in less than 16 bits. Any value >12 will be treated as 12, bit \\[4\\]
is always ignored. This can also be used to fit the 12-bit result in 8 bits. Pulse detect negative reload value PULSE_NEG_RL\\[4:0\\]"]
    #[inline(always)]
    pub fn shift_r(&self) -> SHIFT_R_R {
        SHIFT_R_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Range detect mode"]
    #[inline(always)]
    pub fn range_mode(&self) -> RANGE_MODE_R {
        RANGE_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 25 - Select tr_sar_ch_done mode for last channel of a group, ignored for all other channels Also see TR_CTL.DONE_LEVEL"]
    #[inline(always)]
    pub fn tr_done_grp_vio(&self) -> TR_DONE_GRP_VIO_R {
        TR_DONE_GRP_VIO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Post processing"]
    #[inline(always)]
    #[must_use]
    pub fn post_proc(&mut self) -> POST_PROC_W<0> {
        POST_PROC_W::new(self)
    }
    #[doc = "Bit 6 - Left or right align data in result\\[15:0\\]. 0: the data is right aligned in result\\[11:0\\], with sign extension to 16 bits if enabled 1: the data is left aligned in result\\[15:4\\]
with the lower nibble 0. Caveat if the result was more than 12 bits (e.g. after averaging) then the bits above 12 will be discarded."]
    #[inline(always)]
    #[must_use]
    pub fn left_align(&mut self) -> LEFT_ALIGN_W<6> {
        LEFT_ALIGN_W::new(self)
    }
    #[doc = "Bit 7 - Output data is sign extended"]
    #[inline(always)]
    #[must_use]
    pub fn sign_ext(&mut self) -> SIGN_EXT_W<7> {
        SIGN_EXT_W::new(self)
    }
    #[doc = "Bits 8:15 - Either averaging count (minus 1) or Pulse positive reload value Averaging Count for channels that have averaging enabled. A channel will be sampled (AVG_CNT+1) = \\[1..256\\]
times. The signal will be acquired back to back (1st order accumulate and dump filter), the average result is calculated and stored and then the next enabled channel is sampled. If more than 16 sample are taken (AVG_CNT>=16) then AVG_SHIFT must be set so that the result after shifting fits in 16 bits Pulse detect positive reload value PULSE_POS_RL\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn avg_cnt(&mut self) -> AVG_CNT_W<8> {
        AVG_CNT_W::new(self)
    }
    #[doc = "Bits 16:20 - Either Shift Right (no pulse detection) or Pulse negative reload value (if pulse detection is enabled) Shift right SHIFT_R\\[3:0\\]
= \\[0..12\\]: the result (typically after averaging) is shifted right as specified here. Software has to make sure that the result fits in less than 16 bits. Any value >12 will be treated as 12, bit \\[4\\]
is always ignored. This can also be used to fit the 12-bit result in 8 bits. Pulse detect negative reload value PULSE_NEG_RL\\[4:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn shift_r(&mut self) -> SHIFT_R_W<16> {
        SHIFT_R_W::new(self)
    }
    #[doc = "Bits 22:23 - Range detect mode"]
    #[inline(always)]
    #[must_use]
    pub fn range_mode(&mut self) -> RANGE_MODE_W<22> {
        RANGE_MODE_W::new(self)
    }
    #[doc = "Bit 25 - Select tr_sar_ch_done mode for last channel of a group, ignored for all other channels Also see TR_CTL.DONE_LEVEL"]
    #[inline(always)]
    #[must_use]
    pub fn tr_done_grp_vio(&mut self) -> TR_DONE_GRP_VIO_W<25> {
        TR_DONE_GRP_VIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Post processing control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [post_ctl](index.html) module"]
pub struct POST_CTL_SPEC;
impl crate::RegisterSpec for POST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [post_ctl::R](R) reader structure"]
impl crate::Readable for POST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [post_ctl::W](W) writer structure"]
impl crate::Writable for POST_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POST_CTL to value 0"]
impl crate::Resettable for POST_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
