#[doc = "Register `DECODEINTRMASK` reader"]
pub struct R(crate::R<DECODEINTRMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECODEINTRMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECODEINTRMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECODEINTRMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECODEINTRMASK` writer"]
pub struct W(crate::W<DECODEINTRMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECODEINTRMASK_SPEC>;
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
impl From<crate::W<DECODEINTRMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECODEINTRMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECODEINTRMASK` reader - Setting this bit to '1' propagates decoding completion status (INTR_DEC.NORMALEND/INTR_DEC.CORRECTEDEND ORed) to INTR.JPEG_CORE."]
pub type DECODEINTRMASK_R = crate::BitReader<bool>;
#[doc = "Field `DECODEINTRMASK` writer - Setting this bit to '1' propagates decoding completion status (INTR_DEC.NORMALEND/INTR_DEC.CORRECTEDEND ORed) to INTR.JPEG_CORE."]
pub type DECODEINTRMASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DECODEINTRMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit to '1' propagates decoding completion status (INTR_DEC.NORMALEND/INTR_DEC.CORRECTEDEND ORed) to INTR.JPEG_CORE."]
    #[inline(always)]
    pub fn decodeintrmask(&self) -> DECODEINTRMASK_R {
        DECODEINTRMASK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit to '1' propagates decoding completion status (INTR_DEC.NORMALEND/INTR_DEC.CORRECTEDEND ORed) to INTR.JPEG_CORE."]
    #[inline(always)]
    #[must_use]
    pub fn decodeintrmask(&mut self) -> DECODEINTRMASK_W<0> {
        DECODEINTRMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decoder core decoding end interrupt mask.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decodeintrmask](index.html) module"]
pub struct DECODEINTRMASK_SPEC;
impl crate::RegisterSpec for DECODEINTRMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decodeintrmask::R](R) reader structure"]
impl crate::Readable for DECODEINTRMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decodeintrmask::W](W) writer structure"]
impl crate::Writable for DECODEINTRMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECODEINTRMASK to value 0"]
impl crate::Resettable for DECODEINTRMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
