#[doc = "Register `INTR_CMDSEQ` reader"]
pub struct R(crate::R<INTR_CMDSEQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CMDSEQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CMDSEQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CMDSEQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_CMDSEQ` writer"]
pub struct W(crate::W<INTR_CMDSEQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_CMDSEQ_SPEC>;
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
impl From<crate::W<INTR_CMDSEQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_CMDSEQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_CMDSEQ` reader - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_CMDSEQ_SET is written. By writing 1 the corresponding bit is cleared."]
pub type INTR_CMDSEQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTR_CMDSEQ` writer - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_CMDSEQ_SET is written. By writing 1 the corresponding bit is cleared."]
pub type INTR_CMDSEQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_CMDSEQ_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_CMDSEQ_SET is written. By writing 1 the corresponding bit is cleared."]
    #[inline(always)]
    pub fn intr_cmdseq(&self) -> INTR_CMDSEQ_R {
        INTR_CMDSEQ_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_CMDSEQ_SET is written. By writing 1 the corresponding bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn intr_cmdseq(&mut self) -> INTR_CMDSEQ_W<0> {
        INTR_CMDSEQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt cause register for CmdSeq\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cmdseq](index.html) module"]
pub struct INTR_CMDSEQ_SPEC;
impl crate::RegisterSpec for INTR_CMDSEQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cmdseq::R](R) reader structure"]
impl crate::Readable for INTR_CMDSEQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_cmdseq::W](W) writer structure"]
impl crate::Writable for INTR_CMDSEQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_CMDSEQ to value 0"]
impl crate::Resettable for INTR_CMDSEQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
