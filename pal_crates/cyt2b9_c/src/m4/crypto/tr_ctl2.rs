#[doc = "Register `TR_CTL2` reader"]
pub struct R(crate::R<TR_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTL2` writer"]
pub struct W(crate::W<TR_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTL2_SPEC>;
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
impl From<crate::W<TR_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE` reader - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_CTL2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<0> {
        SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "True random control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl2](index.html) module"]
pub struct TR_CTL2_SPEC;
impl crate::RegisterSpec for TR_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctl2::R](R) reader structure"]
impl crate::Readable for TR_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctl2::W](W) writer structure"]
impl crate::Writable for TR_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTL2 to value 0"]
impl crate::Resettable for TR_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
