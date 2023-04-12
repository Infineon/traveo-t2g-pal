#[doc = "Register `FGSRCR1` reader"]
pub struct R(crate::R<FGSRCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSRCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSRCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSRCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGSRCR1` writer"]
pub struct W(crate::W<FGSRCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSRCR1_SPEC>;
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
impl From<crate::W<FGSRCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSRCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SREN` reader - If enabled, skew control for secondary channel is active. If disabled, skew control for secondary channel is in fetch mode. Fetch mode: secondary channel fetches frames and can intentionally produce a backstall via setting sbusy signal to active. This mode is intended to fetch frames from memory via a stallable pipeline, like the primary interface. No timing adaption of frames takes place in this mode."]
pub type SREN_R = crate::BitReader<bool>;
#[doc = "Field `SREN` writer - If enabled, skew control for secondary channel is active. If disabled, skew control for secondary channel is in fetch mode. Fetch mode: secondary channel fetches frames and can intentionally produce a backstall via setting sbusy signal to active. This mode is intended to fetch frames from memory via a stallable pipeline, like the primary interface. No timing adaption of frames takes place in this mode."]
pub type SREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSRCR1_SPEC, bool, O>;
#[doc = "Field `SRMODE` reader - Skew Control Operating Mode."]
pub type SRMODE_R = crate::FieldReader<u8, SRMODE_A>;
#[doc = "Skew Control Operating Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRMODE_A {
    #[doc = "0: Skew Regulation is off."]
    OFF = 0,
    #[doc = "1: Horizontal regulation enabled."]
    HREG = 1,
    #[doc = "2: Vertical regulation enabled."]
    VREG = 2,
    #[doc = "3: Both regulation modes are enabled."]
    BOTH = 3,
}
impl From<SRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRMODE_A) -> Self {
        variant as _
    }
}
impl SRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRMODE_A {
        match self.bits {
            0 => SRMODE_A::OFF,
            1 => SRMODE_A::HREG,
            2 => SRMODE_A::VREG,
            3 => SRMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SRMODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `HREG`"]
    #[inline(always)]
    pub fn is_hreg(&self) -> bool {
        *self == SRMODE_A::HREG
    }
    #[doc = "Checks if the value of the field is `VREG`"]
    #[inline(always)]
    pub fn is_vreg(&self) -> bool {
        *self == SRMODE_A::VREG
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SRMODE_A::BOTH
    }
}
#[doc = "Field `SRMODE` writer - Skew Control Operating Mode."]
pub type SRMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FGSRCR1_SPEC, u8, SRMODE_A, 2, O>;
impl<'a, const O: u8> SRMODE_W<'a, O> {
    #[doc = "Skew Regulation is off."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRMODE_A::OFF)
    }
    #[doc = "Horizontal regulation enabled."]
    #[inline(always)]
    pub fn hreg(self) -> &'a mut W {
        self.variant(SRMODE_A::HREG)
    }
    #[doc = "Vertical regulation enabled."]
    #[inline(always)]
    pub fn vreg(self) -> &'a mut W {
        self.variant(SRMODE_A::VREG)
    }
    #[doc = "Both regulation modes are enabled."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SRMODE_A::BOTH)
    }
}
#[doc = "Field `SRADJ` reader - Enables line length adjustment for HTOTAL. If SRQAlign=0, SREven determines whether HTOTAL has odd (SREven=0) or even (SrEven=1) number of pixels. If SRQAlign=1, SRQVal determines the value of the two LSB bits of HTOTAL."]
pub type SRADJ_R = crate::BitReader<bool>;
#[doc = "Field `SRADJ` writer - Enables line length adjustment for HTOTAL. If SRQAlign=0, SREven determines whether HTOTAL has odd (SREven=0) or even (SrEven=1) number of pixels. If SRQAlign=1, SRQVal determines the value of the two LSB bits of HTOTAL."]
pub type SRADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSRCR1_SPEC, bool, O>;
#[doc = "Field `SREVEN` reader - Total line length HTOTAL is even when SRAdj is enabled. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
pub type SREVEN_R = crate::BitReader<bool>;
#[doc = "Field `SREVEN` writer - Total line length HTOTAL is even when SRAdj is enabled. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
pub type SREVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSRCR1_SPEC, bool, O>;
#[doc = "Field `SRFASTSYNC` reader - Fast Synchronization Mode. Frame Generator is started synchronized to frame timing on secondary channel when switching FgEn from 0 to 1. This allows fast locking of skew regulation."]
pub type SRFASTSYNC_R = crate::BitReader<bool>;
#[doc = "Field `SRFASTSYNC` writer - Fast Synchronization Mode. Frame Generator is started synchronized to frame timing on secondary channel when switching FgEn from 0 to 1. This allows fast locking of skew regulation."]
pub type SRFASTSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSRCR1_SPEC, bool, O>;
#[doc = "Field `SRQALIGN` reader - Enables alignment of HTOTAL to be a multiple of 4. Overrides SREven field. Program field SRQVal for the desired value of the two LSB bits of HTOTAL."]
pub type SRQALIGN_R = crate::BitReader<bool>;
#[doc = "Field `SRQALIGN` writer - Enables alignment of HTOTAL to be a multiple of 4. Overrides SREven field. Program field SRQVal for the desired value of the two LSB bits of HTOTAL."]
pub type SRQALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSRCR1_SPEC, bool, O>;
#[doc = "Field `SRQVAL` reader - If SRQAlign is enabled, this field determines the fixed value of the two LSB bits of HTOTAL. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
pub type SRQVAL_R = crate::FieldReader<u8, SRQVAL_A>;
#[doc = "If SRQAlign is enabled, this field determines the fixed value of the two LSB bits of HTOTAL. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRQVAL_A {
    #[doc = "0: Fixed two LSB values of HTOTAL are 0b00."]
    ZERO = 0,
    #[doc = "1: Fixed two LSB values of HTOTAL are 0b01."]
    ONE = 1,
    #[doc = "2: Fixed two LSB values of HTOTAL are 0b10."]
    TWO = 2,
    #[doc = "3: Fixed two LSB values of HTOTAL are 0b11."]
    THREE = 3,
}
impl From<SRQVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SRQVAL_A) -> Self {
        variant as _
    }
}
impl SRQVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRQVAL_A {
        match self.bits {
            0 => SRQVAL_A::ZERO,
            1 => SRQVAL_A::ONE,
            2 => SRQVAL_A::TWO,
            3 => SRQVAL_A::THREE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == SRQVAL_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SRQVAL_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SRQVAL_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == SRQVAL_A::THREE
    }
}
#[doc = "Field `SRQVAL` writer - If SRQAlign is enabled, this field determines the fixed value of the two LSB bits of HTOTAL. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
pub type SRQVAL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FGSRCR1_SPEC, u8, SRQVAL_A, 2, O>;
impl<'a, const O: u8> SRQVAL_W<'a, O> {
    #[doc = "Fixed two LSB values of HTOTAL are 0b00."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(SRQVAL_A::ZERO)
    }
    #[doc = "Fixed two LSB values of HTOTAL are 0b01."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SRQVAL_A::ONE)
    }
    #[doc = "Fixed two LSB values of HTOTAL are 0b10."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(SRQVAL_A::TWO)
    }
    #[doc = "Fixed two LSB values of HTOTAL are 0b11."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(SRQVAL_A::THREE)
    }
}
#[doc = "Field `SRDBGDISP` reader - If enabled, the pixels are displayed that are read from FIFO when secondary channel is not in sync yet. Otherwise constant color is displayed in unsynchronized state."]
pub type SRDBGDISP_R = crate::BitReader<bool>;
#[doc = "Field `SRDBGDISP` writer - If enabled, the pixels are displayed that are read from FIFO when secondary channel is not in sync yet. Otherwise constant color is displayed in unsynchronized state."]
pub type SRDBGDISP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSRCR1_SPEC, bool, O>;
#[doc = "Field `SREPOFF` reader - Disables the skew Extrapolation in blanking."]
pub type SREPOFF_R = crate::BitReader<bool>;
#[doc = "Field `SREPOFF` writer - Disables the skew Extrapolation in blanking."]
pub type SREPOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSRCR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If enabled, skew control for secondary channel is active. If disabled, skew control for secondary channel is in fetch mode. Fetch mode: secondary channel fetches frames and can intentionally produce a backstall via setting sbusy signal to active. This mode is intended to fetch frames from memory via a stallable pipeline, like the primary interface. No timing adaption of frames takes place in this mode."]
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Skew Control Operating Mode."]
    #[inline(always)]
    pub fn srmode(&self) -> SRMODE_R {
        SRMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Enables line length adjustment for HTOTAL. If SRQAlign=0, SREven determines whether HTOTAL has odd (SREven=0) or even (SrEven=1) number of pixels. If SRQAlign=1, SRQVal determines the value of the two LSB bits of HTOTAL."]
    #[inline(always)]
    pub fn sradj(&self) -> SRADJ_R {
        SRADJ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Total line length HTOTAL is even when SRAdj is enabled. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
    #[inline(always)]
    pub fn sreven(&self) -> SREVEN_R {
        SREVEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Synchronization Mode. Frame Generator is started synchronized to frame timing on secondary channel when switching FgEn from 0 to 1. This allows fast locking of skew regulation."]
    #[inline(always)]
    pub fn srfastsync(&self) -> SRFASTSYNC_R {
        SRFASTSYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables alignment of HTOTAL to be a multiple of 4. Overrides SREven field. Program field SRQVal for the desired value of the two LSB bits of HTOTAL."]
    #[inline(always)]
    pub fn srqalign(&self) -> SRQALIGN_R {
        SRQALIGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - If SRQAlign is enabled, this field determines the fixed value of the two LSB bits of HTOTAL. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
    #[inline(always)]
    pub fn srqval(&self) -> SRQVAL_R {
        SRQVAL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 16 - If enabled, the pixels are displayed that are read from FIFO when secondary channel is not in sync yet. Otherwise constant color is displayed in unsynchronized state."]
    #[inline(always)]
    pub fn srdbgdisp(&self) -> SRDBGDISP_R {
        SRDBGDISP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disables the skew Extrapolation in blanking."]
    #[inline(always)]
    pub fn srepoff(&self) -> SREPOFF_R {
        SREPOFF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If enabled, skew control for secondary channel is active. If disabled, skew control for secondary channel is in fetch mode. Fetch mode: secondary channel fetches frames and can intentionally produce a backstall via setting sbusy signal to active. This mode is intended to fetch frames from memory via a stallable pipeline, like the primary interface. No timing adaption of frames takes place in this mode."]
    #[inline(always)]
    #[must_use]
    pub fn sren(&mut self) -> SREN_W<0> {
        SREN_W::new(self)
    }
    #[doc = "Bits 1:2 - Skew Control Operating Mode."]
    #[inline(always)]
    #[must_use]
    pub fn srmode(&mut self) -> SRMODE_W<1> {
        SRMODE_W::new(self)
    }
    #[doc = "Bit 3 - Enables line length adjustment for HTOTAL. If SRQAlign=0, SREven determines whether HTOTAL has odd (SREven=0) or even (SrEven=1) number of pixels. If SRQAlign=1, SRQVal determines the value of the two LSB bits of HTOTAL."]
    #[inline(always)]
    #[must_use]
    pub fn sradj(&mut self) -> SRADJ_W<3> {
        SRADJ_W::new(self)
    }
    #[doc = "Bit 4 - Total line length HTOTAL is even when SRAdj is enabled. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
    #[inline(always)]
    #[must_use]
    pub fn sreven(&mut self) -> SREVEN_W<4> {
        SREVEN_W::new(self)
    }
    #[doc = "Bit 5 - Fast Synchronization Mode. Frame Generator is started synchronized to frame timing on secondary channel when switching FgEn from 0 to 1. This allows fast locking of skew regulation."]
    #[inline(always)]
    #[must_use]
    pub fn srfastsync(&mut self) -> SRFASTSYNC_W<5> {
        SRFASTSYNC_W::new(self)
    }
    #[doc = "Bit 6 - Enables alignment of HTOTAL to be a multiple of 4. Overrides SREven field. Program field SRQVal for the desired value of the two LSB bits of HTOTAL."]
    #[inline(always)]
    #[must_use]
    pub fn srqalign(&mut self) -> SRQALIGN_W<6> {
        SRQALIGN_W::new(self)
    }
    #[doc = "Bits 7:8 - If SRQAlign is enabled, this field determines the fixed value of the two LSB bits of HTOTAL. Note: This has impact on the settings for Htotal, HtotalMin and HtotalMax."]
    #[inline(always)]
    #[must_use]
    pub fn srqval(&mut self) -> SRQVAL_W<7> {
        SRQVAL_W::new(self)
    }
    #[doc = "Bit 16 - If enabled, the pixels are displayed that are read from FIFO when secondary channel is not in sync yet. Otherwise constant color is displayed in unsynchronized state."]
    #[inline(always)]
    #[must_use]
    pub fn srdbgdisp(&mut self) -> SRDBGDISP_W<16> {
        SRDBGDISP_W::new(self)
    }
    #[doc = "Bit 17 - Disables the skew Extrapolation in blanking."]
    #[inline(always)]
    #[must_use]
    pub fn srepoff(&mut self) -> SREPOFF_W<17> {
        SREPOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Skew Regulation Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrcr1](index.html) module"]
pub struct FGSRCR1_SPEC;
impl crate::RegisterSpec for FGSRCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrcr1::R](R) reader structure"]
impl crate::Readable for FGSRCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgsrcr1::W](W) writer structure"]
impl crate::Writable for FGSRCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSRCR1 to value 0"]
impl crate::Resettable for FGSRCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
