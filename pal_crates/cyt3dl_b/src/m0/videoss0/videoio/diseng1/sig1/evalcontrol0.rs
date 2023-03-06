#[doc = "Register `EVALCONTROL0` reader"]
pub struct R(crate::R<EVALCONTROL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL0` writer"]
pub struct W(crate::W<EVALCONTROL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL0_SPEC>;
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
impl From<crate::W<EVALCONTROL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN0` reader - When enabled (value 1) a CRC signature is computed for all pixels inside this evaluation window (SigCRC). When this bit is cleared, the internal status for this window is reset (StsSigError bit and frame counters)."]
pub type ENEVALWIN0_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN0` writer - When enabled (value 1) a CRC signature is computed for all pixels inside this evaluation window (SigCRC). When this bit is cleared, the internal status for this window is reset (StsSigError bit and frame counters)."]
pub type ENEVALWIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL0_SPEC, bool, O>;
#[doc = "Field `ENCRC0` reader - When enabled (value 1) the measured signature is checked against a reference value (SigCRCRef)."]
pub type ENCRC0_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC0` writer - When enabled (value 1) the measured signature is checked against a reference value (SigCRCRef)."]
pub type ENCRC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL0_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK0` reader - When enabled (value 1) pixels with alpha bit = 0 are ignored for signature computation."]
pub type ALPHAMASK0_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK0` writer - When enabled (value 1) pixels with alpha bit = 0 are ignored for signature computation."]
pub type ALPHAMASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL0_SPEC, bool, O>;
#[doc = "Field `ALPHAINV0` reader - When enabled (value 1) the effect of AlphaMask is inverted (pixels with alpha bit = 1 are ignored then)."]
pub type ALPHAINV0_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV0` writer - When enabled (value 1) the effect of AlphaMask is inverted (pixels with alpha bit = 1 are ignored then)."]
pub type ALPHAINV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL0_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC0` reader - When enabled (value 1) the error status this window (StsSigError) will replace all pixels inside the window by a constant color on the display. Skip regions due to other evaluation windows on top are not modified. AlphaMask, when enabled, is not considered for this replacement."]
pub type ENLOCALPANIC0_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC0` writer - When enabled (value 1) the error status this window (StsSigError) will replace all pixels inside the window by a constant color on the display. Skip regions due to other evaluation windows on top are not modified. AlphaMask, when enabled, is not considered for this replacement."]
pub type ENLOCALPANIC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL0_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC0` reader - When enabled (value 1) the error status of this window (StsSigError) will activate the panic mode of the display stream's Frame Generator, which can switch to another display mode in response."]
pub type ENGLOBALPANIC0_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC0` writer - When enabled (value 1) the error status of this window (StsSigError) will activate the panic mode of the display stream's Frame Generator, which can switch to another display mode in response."]
pub type ENGLOBALPANIC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When enabled (value 1) a CRC signature is computed for all pixels inside this evaluation window (SigCRC). When this bit is cleared, the internal status for this window is reset (StsSigError bit and frame counters)."]
    #[inline(always)]
    pub fn enevalwin0(&self) -> ENEVALWIN0_R {
        ENEVALWIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When enabled (value 1) the measured signature is checked against a reference value (SigCRCRef)."]
    #[inline(always)]
    pub fn encrc0(&self) -> ENCRC0_R {
        ENCRC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - When enabled (value 1) pixels with alpha bit = 0 are ignored for signature computation."]
    #[inline(always)]
    pub fn alphamask0(&self) -> ALPHAMASK0_R {
        ALPHAMASK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When enabled (value 1) the effect of AlphaMask is inverted (pixels with alpha bit = 1 are ignored then)."]
    #[inline(always)]
    pub fn alphainv0(&self) -> ALPHAINV0_R {
        ALPHAINV0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - When enabled (value 1) the error status this window (StsSigError) will replace all pixels inside the window by a constant color on the display. Skip regions due to other evaluation windows on top are not modified. AlphaMask, when enabled, is not considered for this replacement."]
    #[inline(always)]
    pub fn enlocalpanic0(&self) -> ENLOCALPANIC0_R {
        ENLOCALPANIC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When enabled (value 1) the error status of this window (StsSigError) will activate the panic mode of the display stream's Frame Generator, which can switch to another display mode in response."]
    #[inline(always)]
    pub fn englobalpanic0(&self) -> ENGLOBALPANIC0_R {
        ENGLOBALPANIC0_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When enabled (value 1) a CRC signature is computed for all pixels inside this evaluation window (SigCRC). When this bit is cleared, the internal status for this window is reset (StsSigError bit and frame counters)."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin0(&mut self) -> ENEVALWIN0_W<0> {
        ENEVALWIN0_W::new(self)
    }
    #[doc = "Bit 1 - When enabled (value 1) the measured signature is checked against a reference value (SigCRCRef)."]
    #[inline(always)]
    #[must_use]
    pub fn encrc0(&mut self) -> ENCRC0_W<1> {
        ENCRC0_W::new(self)
    }
    #[doc = "Bit 8 - When enabled (value 1) pixels with alpha bit = 0 are ignored for signature computation."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask0(&mut self) -> ALPHAMASK0_W<8> {
        ALPHAMASK0_W::new(self)
    }
    #[doc = "Bit 9 - When enabled (value 1) the effect of AlphaMask is inverted (pixels with alpha bit = 1 are ignored then)."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv0(&mut self) -> ALPHAINV0_W<9> {
        ALPHAINV0_W::new(self)
    }
    #[doc = "Bit 16 - When enabled (value 1) the error status this window (StsSigError) will replace all pixels inside the window by a constant color on the display. Skip regions due to other evaluation windows on top are not modified. AlphaMask, when enabled, is not considered for this replacement."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic0(&mut self) -> ENLOCALPANIC0_W<16> {
        ENLOCALPANIC0_W::new(self)
    }
    #[doc = "Bit 17 - When enabled (value 1) the error status of this window (StsSigError) will activate the panic mode of the display stream's Frame Generator, which can switch to another display mode in response."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic0(&mut self) -> ENGLOBALPANIC0_W<17> {
        ENGLOBALPANIC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol0](index.html) module"]
pub struct EVALCONTROL0_SPEC;
impl crate::RegisterSpec for EVALCONTROL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol0::R](R) reader structure"]
impl crate::Readable for EVALCONTROL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol0::W](W) writer structure"]
impl crate::Writable for EVALCONTROL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL0 to value 0"]
impl crate::Resettable for EVALCONTROL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
