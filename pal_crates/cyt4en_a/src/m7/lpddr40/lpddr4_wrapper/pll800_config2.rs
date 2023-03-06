#[doc = "Register `PLL800_CONFIG2` reader"]
pub struct R(crate::R<PLL800_CONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL800_CONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL800_CONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL800_CONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL800_CONFIG2` writer"]
pub struct W(crate::W<PLL800_CONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL800_CONFIG2_SPEC>;
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
impl From<crate::W<PLL800_CONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL800_CONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSCG_DEPTH` reader - N/A"]
pub type SSCG_DEPTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSCG_DEPTH` writer - N/A"]
pub type SSCG_DEPTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG2_SPEC, u16, u16, 10, O>;
#[doc = "Field `SSCG_RATE` reader - Control bits for SSCG modulation rate as a factor of fPFD clock frequency. Audio applications should choose a setting that results in a modulation rate that is greater than 20kHz, so fPFD/512 or fPFD/256 is recommended. 0: Modulation rate = fPFD/4096, 1: Modulation rate = fPFD/2048, 2: Modulation rate = fPFD/1024, 3. Modulation rate = fPFD/512, 4: Modulation rate = fPFD/256, 5: Modulation rate = fPFD/128, 6: Modulation rate = fPFD/744, 7: prohibited Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
pub type SSCG_RATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSCG_RATE` writer - Control bits for SSCG modulation rate as a factor of fPFD clock frequency. Audio applications should choose a setting that results in a modulation rate that is greater than 20kHz, so fPFD/512 or fPFD/256 is recommended. 0: Modulation rate = fPFD/4096, 1: Modulation rate = fPFD/2048, 2: Modulation rate = fPFD/1024, 3. Modulation rate = fPFD/512, 4: Modulation rate = fPFD/256, 5: Modulation rate = fPFD/128, 6: Modulation rate = fPFD/744, 7: prohibited Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
pub type SSCG_RATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SSCG_MODE` reader - N/A"]
pub type SSCG_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_MODE` writer - N/A"]
pub type SSCG_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL800_CONFIG2_SPEC, bool, O>;
#[doc = "Field `SSCG_EN` reader - Enables spreading mode. When using spreading, see CLK_PLL800M_CONFIG.LOCK_DELAY for an additional configuration requirement. Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
pub type SSCG_EN_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_EN` writer - Enables spreading mode. When using spreading, see CLK_PLL800M_CONFIG.LOCK_DELAY for an additional configuration requirement. Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
pub type SSCG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL800_CONFIG2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - N/A"]
    #[inline(always)]
    pub fn sscg_depth(&self) -> SSCG_DEPTH_R {
        SSCG_DEPTH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - Control bits for SSCG modulation rate as a factor of fPFD clock frequency. Audio applications should choose a setting that results in a modulation rate that is greater than 20kHz, so fPFD/512 or fPFD/256 is recommended. 0: Modulation rate = fPFD/4096, 1: Modulation rate = fPFD/2048, 2: Modulation rate = fPFD/1024, 3. Modulation rate = fPFD/512, 4: Modulation rate = fPFD/256, 5: Modulation rate = fPFD/128, 6: Modulation rate = fPFD/744, 7: prohibited Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
    #[inline(always)]
    pub fn sscg_rate(&self) -> SSCG_RATE_R {
        SSCG_RATE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    pub fn sscg_mode(&self) -> SSCG_MODE_R {
        SSCG_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables spreading mode. When using spreading, see CLK_PLL800M_CONFIG.LOCK_DELAY for an additional configuration requirement. Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
    #[inline(always)]
    pub fn sscg_en(&self) -> SSCG_EN_R {
        SSCG_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_depth(&mut self) -> SSCG_DEPTH_W<0> {
        SSCG_DEPTH_W::new(self)
    }
    #[doc = "Bits 16:18 - Control bits for SSCG modulation rate as a factor of fPFD clock frequency. Audio applications should choose a setting that results in a modulation rate that is greater than 20kHz, so fPFD/512 or fPFD/256 is recommended. 0: Modulation rate = fPFD/4096, 1: Modulation rate = fPFD/2048, 2: Modulation rate = fPFD/1024, 3. Modulation rate = fPFD/512, 4: Modulation rate = fPFD/256, 5: Modulation rate = fPFD/128, 6: Modulation rate = fPFD/744, 7: prohibited Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sscg_rate(&mut self) -> SSCG_RATE_W<16> {
        SSCG_RATE_W::new(self)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_mode(&mut self) -> SSCG_MODE_W<28> {
        SSCG_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Enables spreading mode. When using spreading, see CLK_PLL800M_CONFIG.LOCK_DELAY for an additional configuration requirement. Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sscg_en(&mut self) -> SSCG_EN_W<31> {
        SSCG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "800MHz PLL Configuration Register 2 (SSCG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll800_config2](index.html) module"]
pub struct PLL800_CONFIG2_SPEC;
impl crate::RegisterSpec for PLL800_CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll800_config2::R](R) reader structure"]
impl crate::Readable for PLL800_CONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll800_config2::W](W) writer structure"]
impl crate::Writable for PLL800_CONFIG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL800_CONFIG2 to value 0"]
impl crate::Resettable for PLL800_CONFIG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
