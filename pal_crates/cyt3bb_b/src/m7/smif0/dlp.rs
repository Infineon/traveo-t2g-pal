#[doc = "Register `DLP` reader"]
pub struct R(crate::R<DLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLP` writer"]
pub struct W(crate::W<DLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLP_SPEC>;
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
impl From<crate::W<DLP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLP` reader - Data Learning Pattern"]
pub type DLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLP` writer - Data Learning Pattern"]
pub type DLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data Learning Pattern"]
    #[inline(always)]
    pub fn dlp(&self) -> DLP_R {
        DLP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Learning Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn dlp(&mut self) -> DLP_W<0> {
        DLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Learning Pattern\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlp](index.html) module"]
pub struct DLP_SPEC;
impl crate::RegisterSpec for DLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlp::R](R) reader structure"]
impl crate::Readable for DLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlp::W](W) writer structure"]
impl crate::Writable for DLP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLP to value 0"]
impl crate::Resettable for DLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
