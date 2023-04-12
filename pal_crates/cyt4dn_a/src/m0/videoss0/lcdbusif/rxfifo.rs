#[doc = "Register `RXFIFO[%s]` reader"]
pub struct R(crate::R<RXFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFIFO[%s]` writer"]
pub struct W(crate::W<RXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFIFO_SPEC>;
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
impl From<crate::W<RXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXENTRY` reader - Data received from the LCDBus."]
pub type RXENTRY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXENTRY` writer - Data received from the LCDBus."]
pub type RXENTRY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXFIFO_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - Data received from the LCDBus."]
    #[inline(always)]
    pub fn rxentry(&self) -> RXENTRY_R {
        RXENTRY_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Data received from the LCDBus."]
    #[inline(always)]
    #[must_use]
    pub fn rxentry(&mut self) -> RXENTRY_W<0> {
        RXENTRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception Fifo.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo](index.html) module"]
pub struct RXFIFO_SPEC;
impl crate::RegisterSpec for RXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifo::R](R) reader structure"]
impl crate::Readable for RXFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfifo::W](W) writer structure"]
impl crate::Writable for RXFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFIFO[%s]
to value 0"]
impl crate::Resettable for RXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
