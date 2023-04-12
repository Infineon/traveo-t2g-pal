#[doc = "Register `CROPSTARTX` reader"]
pub struct R(crate::R<CROPSTARTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CROPSTARTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CROPSTARTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CROPSTARTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CROPSTARTX` writer"]
pub struct W(crate::W<CROPSTARTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CROPSTARTX_SPEC>;
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
impl From<crate::W<CROPSTARTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CROPSTARTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CROPSTARTX` reader - X crop start position"]
pub type CROPSTARTX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CROPSTARTX` writer - X crop start position"]
pub type CROPSTARTX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CROPSTARTX_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - X crop start position"]
    #[inline(always)]
    pub fn cropstartx(&self) -> CROPSTARTX_R {
        CROPSTARTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X crop start position"]
    #[inline(always)]
    #[must_use]
    pub fn cropstartx(&mut self) -> CROPSTARTX_W<0> {
        CROPSTARTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "X crop start position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cropstartx](index.html) module"]
pub struct CROPSTARTX_SPEC;
impl crate::RegisterSpec for CROPSTARTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cropstartx::R](R) reader structure"]
impl crate::Readable for CROPSTARTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cropstartx::W](W) writer structure"]
impl crate::Writable for CROPSTARTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CROPSTARTX to value 0"]
impl crate::Resettable for CROPSTARTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
