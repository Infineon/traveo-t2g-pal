#[doc = "Register `TST_XRES_KEY` reader"]
pub struct R(crate::R<TST_XRES_KEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_XRES_KEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_XRES_KEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_XRES_KEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_XRES_KEY` writer"]
pub struct W(crate::W<TST_XRES_KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_XRES_KEY_SPEC>;
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
impl From<crate::W<TST_XRES_KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_XRES_KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_IN` reader - Connected to the XRES DFT key shift register input. Value of this bit must be set before change KEY_CLK from low to high. Value may be changed on the same cycle that changes KEY_CLK from high to low."]
pub type KEY_IN_R = crate::BitReader<bool>;
#[doc = "Field `KEY_IN` writer - Connected to the XRES DFT key shift register input. Value of this bit must be set before change KEY_CLK from low to high. Value may be changed on the same cycle that changes KEY_CLK from high to low."]
pub type KEY_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_XRES_KEY_SPEC, bool, O>;
#[doc = "Field `KEY_CLK` reader - Connected to the XRES DFT key shift register clock. Must be toggled low to high and back for each key bit to be shifted in."]
pub type KEY_CLK_R = crate::BitReader<bool>;
#[doc = "Field `KEY_CLK` writer - Connected to the XRES DFT key shift register clock. Must be toggled low to high and back for each key bit to be shifted in."]
pub type KEY_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_XRES_KEY_SPEC, bool, O>;
#[doc = "Field `KEY_START` reader - This field is used to start the state machine controlling firmware-initiated key entry. Write this field high when the 128b key is being sent, and otherwise keep it low."]
pub type KEY_START_R = crate::BitReader<bool>;
#[doc = "Field `KEY_START` writer - This field is used to start the state machine controlling firmware-initiated key entry. Write this field high when the 128b key is being sent, and otherwise keep it low."]
pub type KEY_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_XRES_KEY_SPEC, bool, O>;
#[doc = "Field `KEY_MODE` reader - 'Declares' the test mode that firmware is going to shift into the key register using KEY_IN/KEY_CLK. This field must maintain the proper test mode value for a firmware-initiated test mode key to be effective. If this field is changed (or reset), the firmware-initiated test mode is exited immediately. This field has no effect on XRES or PWR_DDFT_XRES initiated test modes."]
pub type KEY_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_MODE` writer - 'Declares' the test mode that firmware is going to shift into the key register using KEY_IN/KEY_CLK. This field must maintain the proper test mode value for a firmware-initiated test mode key to be effective. If this field is changed (or reset), the firmware-initiated test mode is exited immediately. This field has no effect on XRES or PWR_DDFT_XRES initiated test modes."]
pub type KEY_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TST_XRES_KEY_SPEC, u8, u8, 4, O>;
#[doc = "Field `DISABLE` reader - Disables the firmware key entry capability until next reset."]
pub type DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE` writer - Disables the firmware key entry capability until next reset."]
pub type DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_XRES_KEY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Connected to the XRES DFT key shift register input. Value of this bit must be set before change KEY_CLK from low to high. Value may be changed on the same cycle that changes KEY_CLK from high to low."]
    #[inline(always)]
    pub fn key_in(&self) -> KEY_IN_R {
        KEY_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connected to the XRES DFT key shift register clock. Must be toggled low to high and back for each key bit to be shifted in."]
    #[inline(always)]
    pub fn key_clk(&self) -> KEY_CLK_R {
        KEY_CLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field is used to start the state machine controlling firmware-initiated key entry. Write this field high when the 128b key is being sent, and otherwise keep it low."]
    #[inline(always)]
    pub fn key_start(&self) -> KEY_START_R {
        KEY_START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 'Declares' the test mode that firmware is going to shift into the key register using KEY_IN/KEY_CLK. This field must maintain the proper test mode value for a firmware-initiated test mode key to be effective. If this field is changed (or reset), the firmware-initiated test mode is exited immediately. This field has no effect on XRES or PWR_DDFT_XRES initiated test modes."]
    #[inline(always)]
    pub fn key_mode(&self) -> KEY_MODE_R {
        KEY_MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Disables the firmware key entry capability until next reset."]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Connected to the XRES DFT key shift register input. Value of this bit must be set before change KEY_CLK from low to high. Value may be changed on the same cycle that changes KEY_CLK from high to low."]
    #[inline(always)]
    #[must_use]
    pub fn key_in(&mut self) -> KEY_IN_W<0> {
        KEY_IN_W::new(self)
    }
    #[doc = "Bit 1 - Connected to the XRES DFT key shift register clock. Must be toggled low to high and back for each key bit to be shifted in."]
    #[inline(always)]
    #[must_use]
    pub fn key_clk(&mut self) -> KEY_CLK_W<1> {
        KEY_CLK_W::new(self)
    }
    #[doc = "Bit 2 - This field is used to start the state machine controlling firmware-initiated key entry. Write this field high when the 128b key is being sent, and otherwise keep it low."]
    #[inline(always)]
    #[must_use]
    pub fn key_start(&mut self) -> KEY_START_W<2> {
        KEY_START_W::new(self)
    }
    #[doc = "Bits 8:11 - 'Declares' the test mode that firmware is going to shift into the key register using KEY_IN/KEY_CLK. This field must maintain the proper test mode value for a firmware-initiated test mode key to be effective. If this field is changed (or reset), the firmware-initiated test mode is exited immediately. This field has no effect on XRES or PWR_DDFT_XRES initiated test modes."]
    #[inline(always)]
    #[must_use]
    pub fn key_mode(&mut self) -> KEY_MODE_W<8> {
        KEY_MODE_W::new(self)
    }
    #[doc = "Bit 14 - Disables the firmware key entry capability until next reset."]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DISABLE_W<14> {
        DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XRES DFT Key firmware controlled test mode entry register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_xres_key](index.html) module"]
pub struct TST_XRES_KEY_SPEC;
impl crate::RegisterSpec for TST_XRES_KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst_xres_key::R](R) reader structure"]
impl crate::Readable for TST_XRES_KEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_xres_key::W](W) writer structure"]
impl crate::Writable for TST_XRES_KEY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST_XRES_KEY to value 0"]
impl crate::Resettable for TST_XRES_KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
