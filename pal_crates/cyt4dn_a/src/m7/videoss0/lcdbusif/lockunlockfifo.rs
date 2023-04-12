#[doc = "Register `LOCKUNLOCKFIFO` reader"]
pub struct R(crate::R<LOCKUNLOCKFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKUNLOCKFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKUNLOCKFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKUNLOCKFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKUNLOCKFIFO` writer"]
pub struct W(crate::W<LOCKUNLOCKFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKUNLOCKFIFO_SPEC>;
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
impl From<crate::W<LOCKUNLOCKFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKUNLOCKFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKUNLOCKFIFO` reader - The protection status is changed by writing one of the following key values to this field:"]
pub type LOCKUNLOCKFIFO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOCKUNLOCKFIFO` writer - The protection status is changed by writing one of the following key values to this field:"]
pub type LOCKUNLOCKFIFO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOCKUNLOCKFIFO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The protection status is changed by writing one of the following key values to this field:"]
    #[inline(always)]
    pub fn lockunlockfifo(&self) -> LOCKUNLOCKFIFO_R {
        LOCKUNLOCKFIFO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The protection status is changed by writing one of the following key values to this field:"]
    #[inline(always)]
    #[must_use]
    pub fn lockunlockfifo(&mut self) -> LOCKUNLOCKFIFO_W<0> {
        LOCKUNLOCKFIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to change the protection status of the fifo address blocks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockunlockfifo](index.html) module"]
pub struct LOCKUNLOCKFIFO_SPEC;
impl crate::RegisterSpec for LOCKUNLOCKFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockunlockfifo::R](R) reader structure"]
impl crate::Readable for LOCKUNLOCKFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockunlockfifo::W](W) writer structure"]
impl crate::Writable for LOCKUNLOCKFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKUNLOCKFIFO to value 0"]
impl crate::Resettable for LOCKUNLOCKFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
