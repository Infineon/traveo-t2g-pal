#[doc = "Register `RX_CAPTURE_CONFIG` reader"]
pub struct R(crate::R<RX_CAPTURE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CAPTURE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CAPTURE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CAPTURE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CAPTURE_CONFIG` writer"]
pub struct W(crate::W<RX_CAPTURE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CAPTURE_CONFIG_SPEC>;
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
impl From<crate::W<RX_CAPTURE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CAPTURE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEG_SDL_TAP_SEL` reader - Determines the amount of delay through the 'neg' SDL for the receive clock coming from the pins in terms of the number of tap delays. There are 16 SDL taps and the delay resolution of each tap is controlled by and thus closely mimics the delay resolution of the MDL. i.e., each tap has a resolution roughly equal to 1/16th of the DLL reference clock period. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (effectively a full DLL reference clock period)."]
pub type NEG_SDL_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEG_SDL_TAP_SEL` writer - Determines the amount of delay through the 'neg' SDL for the receive clock coming from the pins in terms of the number of tap delays. There are 16 SDL taps and the delay resolution of each tap is controlled by and thus closely mimics the delay resolution of the MDL. i.e., each tap has a resolution roughly equal to 1/16th of the DLL reference clock period. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (effectively a full DLL reference clock period)."]
pub type NEG_SDL_TAP_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CAPTURE_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `POS_SDL_TAP_SEL` reader - Same as NEG_SDL_TAP_SEL except for the 'pos' SDL."]
pub type POS_SDL_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POS_SDL_TAP_SEL` writer - Same as NEG_SDL_TAP_SEL except for the 'pos' SDL."]
pub type POS_SDL_TAP_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CAPTURE_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `SDR_PIPELINE_NEG_DAT` reader - 0: Does not pipeline SDR receive data captured relative to the negedge of the receive clock 1: Does pipeline"]
pub type SDR_PIPELINE_NEG_DAT_R = crate::BitReader<bool>;
#[doc = "Field `SDR_PIPELINE_NEG_DAT` writer - 0: Does not pipeline SDR receive data captured relative to the negedge of the receive clock 1: Does pipeline"]
pub type SDR_PIPELINE_NEG_DAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_CAPTURE_CONFIG_SPEC, bool, O>;
#[doc = "Field `SDR_SWAP_BYTES` reader - 0: Does not swap SDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap"]
pub type SDR_SWAP_BYTES_R = crate::BitReader<bool>;
#[doc = "Field `SDR_SWAP_BYTES` writer - 0: Does not swap SDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap"]
pub type SDR_SWAP_BYTES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_CAPTURE_CONFIG_SPEC, bool, O>;
#[doc = "Field `DDR_PIPELINE_POS_DAT` reader - 0: Does not pipeline DDR receive data captured relative to the posedge of the receive clock 1: Does pipeline DDR_PIPELINE_POS_DAT must be set to '1' if CORE.CTL2.RX_CAPTURE_MODE=2"]
pub type DDR_PIPELINE_POS_DAT_R = crate::BitReader<bool>;
#[doc = "Field `DDR_PIPELINE_POS_DAT` writer - 0: Does not pipeline DDR receive data captured relative to the posedge of the receive clock 1: Does pipeline DDR_PIPELINE_POS_DAT must be set to '1' if CORE.CTL2.RX_CAPTURE_MODE=2"]
pub type DDR_PIPELINE_POS_DAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_CAPTURE_CONFIG_SPEC, bool, O>;
#[doc = "Field `DDR_SWAP_BYTES` reader - 0: Does not swap DDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap The setting DDR_SWAP_BYTES is ignored and operates as if set to 1 (even if set to 0) when CORE.CTL2.RX_CAPTURE_MODE=2"]
pub type DDR_SWAP_BYTES_R = crate::BitReader<bool>;
#[doc = "Field `DDR_SWAP_BYTES` writer - 0: Does not swap DDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap The setting DDR_SWAP_BYTES is ignored and operates as if set to 1 (even if set to 0) when CORE.CTL2.RX_CAPTURE_MODE=2"]
pub type DDR_SWAP_BYTES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_CAPTURE_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Determines the amount of delay through the 'neg' SDL for the receive clock coming from the pins in terms of the number of tap delays. There are 16 SDL taps and the delay resolution of each tap is controlled by and thus closely mimics the delay resolution of the MDL. i.e., each tap has a resolution roughly equal to 1/16th of the DLL reference clock period. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (effectively a full DLL reference clock period)."]
    #[inline(always)]
    pub fn neg_sdl_tap_sel(&self) -> NEG_SDL_TAP_SEL_R {
        NEG_SDL_TAP_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Same as NEG_SDL_TAP_SEL except for the 'pos' SDL."]
    #[inline(always)]
    pub fn pos_sdl_tap_sel(&self) -> POS_SDL_TAP_SEL_R {
        POS_SDL_TAP_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 0: Does not pipeline SDR receive data captured relative to the negedge of the receive clock 1: Does pipeline"]
    #[inline(always)]
    pub fn sdr_pipeline_neg_dat(&self) -> SDR_PIPELINE_NEG_DAT_R {
        SDR_PIPELINE_NEG_DAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Does not swap SDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap"]
    #[inline(always)]
    pub fn sdr_swap_bytes(&self) -> SDR_SWAP_BYTES_R {
        SDR_SWAP_BYTES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - 0: Does not pipeline DDR receive data captured relative to the posedge of the receive clock 1: Does pipeline DDR_PIPELINE_POS_DAT must be set to '1' if CORE.CTL2.RX_CAPTURE_MODE=2"]
    #[inline(always)]
    pub fn ddr_pipeline_pos_dat(&self) -> DDR_PIPELINE_POS_DAT_R {
        DDR_PIPELINE_POS_DAT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: Does not swap DDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap The setting DDR_SWAP_BYTES is ignored and operates as if set to 1 (even if set to 0) when CORE.CTL2.RX_CAPTURE_MODE=2"]
    #[inline(always)]
    pub fn ddr_swap_bytes(&self) -> DDR_SWAP_BYTES_R {
        DDR_SWAP_BYTES_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Determines the amount of delay through the 'neg' SDL for the receive clock coming from the pins in terms of the number of tap delays. There are 16 SDL taps and the delay resolution of each tap is controlled by and thus closely mimics the delay resolution of the MDL. i.e., each tap has a resolution roughly equal to 1/16th of the DLL reference clock period. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (effectively a full DLL reference clock period)."]
    #[inline(always)]
    #[must_use]
    pub fn neg_sdl_tap_sel(&mut self) -> NEG_SDL_TAP_SEL_W<0> {
        NEG_SDL_TAP_SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Same as NEG_SDL_TAP_SEL except for the 'pos' SDL."]
    #[inline(always)]
    #[must_use]
    pub fn pos_sdl_tap_sel(&mut self) -> POS_SDL_TAP_SEL_W<4> {
        POS_SDL_TAP_SEL_W::new(self)
    }
    #[doc = "Bit 16 - 0: Does not pipeline SDR receive data captured relative to the negedge of the receive clock 1: Does pipeline"]
    #[inline(always)]
    #[must_use]
    pub fn sdr_pipeline_neg_dat(&mut self) -> SDR_PIPELINE_NEG_DAT_W<16> {
        SDR_PIPELINE_NEG_DAT_W::new(self)
    }
    #[doc = "Bit 17 - 0: Does not swap SDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap"]
    #[inline(always)]
    #[must_use]
    pub fn sdr_swap_bytes(&mut self) -> SDR_SWAP_BYTES_W<17> {
        SDR_SWAP_BYTES_W::new(self)
    }
    #[doc = "Bit 24 - 0: Does not pipeline DDR receive data captured relative to the posedge of the receive clock 1: Does pipeline DDR_PIPELINE_POS_DAT must be set to '1' if CORE.CTL2.RX_CAPTURE_MODE=2"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_pipeline_pos_dat(&mut self) -> DDR_PIPELINE_POS_DAT_W<24> {
        DDR_PIPELINE_POS_DAT_W::new(self)
    }
    #[doc = "Bit 25 - 0: Does not swap DDR receive data captured relative to the negedge of the receive clock with the data captured relative to the posedge of the receive clock 1: Does swap The setting DDR_SWAP_BYTES is ignored and operates as if set to 1 (even if set to 0) when CORE.CTL2.RX_CAPTURE_MODE=2"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_swap_bytes(&mut self) -> DDR_SWAP_BYTES_W<25> {
        DDR_SWAP_BYTES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX capture configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_capture_config](index.html) module"]
pub struct RX_CAPTURE_CONFIG_SPEC;
impl crate::RegisterSpec for RX_CAPTURE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_capture_config::R](R) reader structure"]
impl crate::Readable for RX_CAPTURE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_capture_config::W](W) writer structure"]
impl crate::Writable for RX_CAPTURE_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CAPTURE_CONFIG to value 0"]
impl crate::Resettable for RX_CAPTURE_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
