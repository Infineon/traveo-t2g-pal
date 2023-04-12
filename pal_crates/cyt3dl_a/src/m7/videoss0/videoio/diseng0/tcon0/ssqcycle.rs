#[doc = "Register `SSQCYCLE` reader"]
pub struct R(crate::R<SSQCYCLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSQCYCLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSQCYCLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSQCYCLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSQCYCLE` writer"]
pub struct W(crate::W<SSQCYCLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSQCYCLE_SPEC>;
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
impl From<crate::W<SSQCYCLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSQCYCLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSQCYCLE` reader - Sequencer cycle length (number -1) of sequencer cycles"]
pub type SSQCYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSQCYCLE` writer - Sequencer cycle length (number -1) of sequencer cycles"]
pub type SSQCYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSQCYCLE_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Sequencer cycle length (number -1) of sequencer cycles"]
    #[inline(always)]
    pub fn ssqcycle(&self) -> SSQCYCLE_R {
        SSQCYCLE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sequencer cycle length (number -1) of sequencer cycles"]
    #[inline(always)]
    #[must_use]
    pub fn ssqcycle(&mut self) -> SSQCYCLE_W<0> {
        SSQCYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This bitfield sets the sequencer cycle length. The value set here -1 is the number of sequencer cycles\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssqcycle](index.html) module"]
pub struct SSQCYCLE_SPEC;
impl crate::RegisterSpec for SSQCYCLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssqcycle::R](R) reader structure"]
impl crate::Readable for SSQCYCLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssqcycle::W](W) writer structure"]
impl crate::Writable for SSQCYCLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSQCYCLE to value 0"]
impl crate::Resettable for SSQCYCLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
