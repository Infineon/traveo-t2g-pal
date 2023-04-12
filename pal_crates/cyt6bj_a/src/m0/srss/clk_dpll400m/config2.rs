#[doc = "Register `CONFIG2` reader"]
pub struct R(crate::R<CONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG2` writer"]
pub struct W(crate::W<CONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG2_SPEC>;
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
impl From<crate::W<CONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC_DIV` reader - Control bits for fractional divider. This value is interpreted as a fraction of the PFD frequency, i.e. fPFD * (FRAC_DIV/2^24). This field can be dynamically updated within the 1000ppm control limit. It takes up to 115 AHB cycles to transfer the setting to the PLL, and writes that occur faster may be silently ignored and require the application to write again after the previous update has finished. Reading the register returns the accepted value. The PLL will start targeting the new value, but it may take significant time (milliseconds) to stabilize at the new average value. Do not change the FRAC_DIV setting while the PLL is initially locking."]
pub type FRAC_DIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRAC_DIV` writer - Control bits for fractional divider. This value is interpreted as a fraction of the PFD frequency, i.e. fPFD * (FRAC_DIV/2^24). This field can be dynamically updated within the 1000ppm control limit. It takes up to 115 AHB cycles to transfer the setting to the PLL, and writes that occur faster may be silently ignored and require the application to write again after the previous update has finished. Reading the register returns the accepted value. The PLL will start targeting the new value, but it may take significant time (milliseconds) to stabilize at the new average value. Do not change the FRAC_DIV setting while the PLL is initially locking."]
pub type FRAC_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG2_SPEC, u32, u32, 24, O>;
#[doc = "Field `FRAC_DITHER_EN` reader - N/A"]
pub type FRAC_DITHER_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC_DITHER_EN` writer - N/A"]
pub type FRAC_DITHER_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FRAC_EN` reader - Enables fractional division mode. When using fractional division mode, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
pub type FRAC_EN_R = crate::BitReader<bool>;
#[doc = "Field `FRAC_EN` writer - Enables fractional division mode. When using fractional division mode, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
pub type FRAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Control bits for fractional divider. This value is interpreted as a fraction of the PFD frequency, i.e. fPFD * (FRAC_DIV/2^24). This field can be dynamically updated within the 1000ppm control limit. It takes up to 115 AHB cycles to transfer the setting to the PLL, and writes that occur faster may be silently ignored and require the application to write again after the previous update has finished. Reading the register returns the accepted value. The PLL will start targeting the new value, but it may take significant time (milliseconds) to stabilize at the new average value. Do not change the FRAC_DIV setting while the PLL is initially locking."]
    #[inline(always)]
    pub fn frac_div(&self) -> FRAC_DIV_R {
        FRAC_DIV_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 28:30 - N/A"]
    #[inline(always)]
    pub fn frac_dither_en(&self) -> FRAC_DITHER_EN_R {
        FRAC_DITHER_EN_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Enables fractional division mode. When using fractional division mode, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
    #[inline(always)]
    pub fn frac_en(&self) -> FRAC_EN_R {
        FRAC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Control bits for fractional divider. This value is interpreted as a fraction of the PFD frequency, i.e. fPFD * (FRAC_DIV/2^24). This field can be dynamically updated within the 1000ppm control limit. It takes up to 115 AHB cycles to transfer the setting to the PLL, and writes that occur faster may be silently ignored and require the application to write again after the previous update has finished. Reading the register returns the accepted value. The PLL will start targeting the new value, but it may take significant time (milliseconds) to stabilize at the new average value. Do not change the FRAC_DIV setting while the PLL is initially locking."]
    #[inline(always)]
    #[must_use]
    pub fn frac_div(&mut self) -> FRAC_DIV_W<0> {
        FRAC_DIV_W::new(self)
    }
    #[doc = "Bits 28:30 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn frac_dither_en(&mut self) -> FRAC_DITHER_EN_W<28> {
        FRAC_DITHER_EN_W::new(self)
    }
    #[doc = "Bit 31 - Enables fractional division mode. When using fractional division mode, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
    #[inline(always)]
    #[must_use]
    pub fn frac_en(&mut self) -> FRAC_EN_W<31> {
        FRAC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "400MHz Digital PLL Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config2](index.html) module"]
pub struct CONFIG2_SPEC;
impl crate::RegisterSpec for CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config2::R](R) reader structure"]
impl crate::Readable for CONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config2::W](W) writer structure"]
impl crate::Writable for CONFIG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG2 to value 0"]
impl crate::Resettable for CONFIG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
