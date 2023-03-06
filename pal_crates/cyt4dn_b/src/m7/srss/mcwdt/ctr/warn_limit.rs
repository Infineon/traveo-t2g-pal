#[doc = "Register `WARN_LIMIT` reader"]
pub struct R(crate::R<WARN_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WARN_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WARN_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WARN_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WARN_LIMIT` writer"]
pub struct W(crate::W<WARN_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WARN_LIMIT_SPEC>;
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
impl From<crate::W<WARN_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WARN_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WARN_LIMIT` reader - Warn limit for this MCWDT subcounter. See WARN_ACTION."]
pub type WARN_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WARN_LIMIT` writer - Warn limit for this MCWDT subcounter. See WARN_ACTION."]
pub type WARN_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WARN_LIMIT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Warn limit for this MCWDT subcounter. See WARN_ACTION."]
    #[inline(always)]
    pub fn warn_limit(&self) -> WARN_LIMIT_R {
        WARN_LIMIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Warn limit for this MCWDT subcounter. See WARN_ACTION."]
    #[inline(always)]
    #[must_use]
    pub fn warn_limit(&mut self) -> WARN_LIMIT_W<0> {
        WARN_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCWDT Subcounter Warn Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [warn_limit](index.html) module"]
pub struct WARN_LIMIT_SPEC;
impl crate::RegisterSpec for WARN_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [warn_limit::R](R) reader structure"]
impl crate::Readable for WARN_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [warn_limit::W](W) writer structure"]
impl crate::Writable for WARN_LIMIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WARN_LIMIT to value 0"]
impl crate::Resettable for WARN_LIMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
