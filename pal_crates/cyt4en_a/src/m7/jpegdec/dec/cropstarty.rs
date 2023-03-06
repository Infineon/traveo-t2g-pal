#[doc = "Register `CROPSTARTY` reader"]
pub struct R(crate::R<CROPSTARTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CROPSTARTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CROPSTARTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CROPSTARTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CROPSTARTY` writer"]
pub struct W(crate::W<CROPSTARTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CROPSTARTY_SPEC>;
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
impl From<crate::W<CROPSTARTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CROPSTARTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CROPSTARTY` reader - Y crop start position"]
pub type CROPSTARTY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CROPSTARTY` writer - Y crop start position"]
pub type CROPSTARTY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CROPSTARTY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Y crop start position"]
    #[inline(always)]
    pub fn cropstarty(&self) -> CROPSTARTY_R {
        CROPSTARTY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Y crop start position"]
    #[inline(always)]
    #[must_use]
    pub fn cropstarty(&mut self) -> CROPSTARTY_W<0> {
        CROPSTARTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Y crop start position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cropstarty](index.html) module"]
pub struct CROPSTARTY_SPEC;
impl crate::RegisterSpec for CROPSTARTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cropstarty::R](R) reader structure"]
impl crate::Readable for CROPSTARTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cropstarty::W](W) writer structure"]
impl crate::Writable for CROPSTARTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CROPSTARTY to value 0"]
impl crate::Resettable for CROPSTARTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
