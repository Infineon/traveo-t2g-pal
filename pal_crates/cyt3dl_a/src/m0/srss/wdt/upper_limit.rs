#[doc = "Register `UPPER_LIMIT` reader"]
pub struct R(crate::R<UPPER_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPPER_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPPER_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPPER_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPPER_LIMIT` writer"]
pub struct W(crate::W<UPPER_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPPER_LIMIT_SPEC>;
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
impl From<crate::W<UPPER_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPPER_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPPER_LIMIT` reader - Upper limit for watchdog. See UPPER_ACTION."]
pub type UPPER_LIMIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UPPER_LIMIT` writer - Upper limit for watchdog. See UPPER_ACTION."]
pub type UPPER_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UPPER_LIMIT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Upper limit for watchdog. See UPPER_ACTION."]
    #[inline(always)]
    pub fn upper_limit(&self) -> UPPER_LIMIT_R {
        UPPER_LIMIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Upper limit for watchdog. See UPPER_ACTION."]
    #[inline(always)]
    #[must_use]
    pub fn upper_limit(&mut self) -> UPPER_LIMIT_W<0> {
        UPPER_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Upper Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upper_limit](index.html) module"]
pub struct UPPER_LIMIT_SPEC;
impl crate::RegisterSpec for UPPER_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upper_limit::R](R) reader structure"]
impl crate::Readable for UPPER_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [upper_limit::W](W) writer structure"]
impl crate::Writable for UPPER_LIMIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPPER_LIMIT to value 0x8000"]
impl crate::Resettable for UPPER_LIMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
