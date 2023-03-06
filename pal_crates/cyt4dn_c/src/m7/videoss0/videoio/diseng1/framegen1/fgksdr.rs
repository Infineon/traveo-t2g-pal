#[doc = "Register `FGKSDR` reader"]
pub struct R(crate::R<FGKSDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGKSDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGKSDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGKSDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGKSDR` writer"]
pub struct W(crate::W<FGKSDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGKSDR_SPEC>;
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
impl From<crate::W<FGKSDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGKSDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCNTCPLMAX` reader - Maximum Value for ppendcnt_cpl_s complementary primary kick counter. Do not change!"]
pub type PCNTCPLMAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCNTCPLMAX` writer - Maximum Value for ppendcnt_cpl_s complementary primary kick counter. Do not change!"]
pub type PCNTCPLMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGKSDR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SCNTCPLMAX` reader - Maximum Value for spendcnt_cpl_s complementary secondary kick counter. Do not change!"]
pub type SCNTCPLMAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCNTCPLMAX` writer - Maximum Value for spendcnt_cpl_s complementary secondary kick counter. Do not change!"]
pub type SCNTCPLMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGKSDR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Maximum Value for ppendcnt_cpl_s complementary primary kick counter. Do not change!"]
    #[inline(always)]
    pub fn pcntcplmax(&self) -> PCNTCPLMAX_R {
        PCNTCPLMAX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Maximum Value for spendcnt_cpl_s complementary secondary kick counter. Do not change!"]
    #[inline(always)]
    pub fn scntcplmax(&self) -> SCNTCPLMAX_R {
        SCNTCPLMAX_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Maximum Value for ppendcnt_cpl_s complementary primary kick counter. Do not change!"]
    #[inline(always)]
    #[must_use]
    pub fn pcntcplmax(&mut self) -> PCNTCPLMAX_W<0> {
        PCNTCPLMAX_W::new(self)
    }
    #[doc = "Bits 16:18 - Maximum Value for spendcnt_cpl_s complementary secondary kick counter. Do not change!"]
    #[inline(always)]
    #[must_use]
    pub fn scntcplmax(&mut self) -> SCNTCPLMAX_W<16> {
        SCNTCPLMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Kick System Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgksdr](index.html) module"]
pub struct FGKSDR_SPEC;
impl crate::RegisterSpec for FGKSDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgksdr::R](R) reader structure"]
impl crate::Readable for FGKSDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgksdr::W](W) writer structure"]
impl crate::Writable for FGKSDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGKSDR to value 0x0002_0002"]
impl crate::Resettable for FGKSDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
