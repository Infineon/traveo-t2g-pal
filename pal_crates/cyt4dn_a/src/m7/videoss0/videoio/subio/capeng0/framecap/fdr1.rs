#[doc = "Register `FDR1` reader"]
pub struct R(crate::R<FDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDR1` writer"]
pub struct W(crate::W<FDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDR1_SPEC>;
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
impl From<crate::W<FDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HEIGHT1` reader - Expected frame heigth of video fields with id 1 (value is Heigth - 1). It could be configured after mode detection."]
pub type HEIGHT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HEIGHT1` writer - Expected frame heigth of video fields with id 1 (value is Heigth - 1). It could be configured after mode detection."]
pub type HEIGHT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 16:29 - Expected frame heigth of video fields with id 1 (value is Heigth - 1). It could be configured after mode detection."]
    #[inline(always)]
    pub fn height1(&self) -> HEIGHT1_R {
        HEIGHT1_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - Expected frame heigth of video fields with id 1 (value is Heigth - 1). It could be configured after mode detection."]
    #[inline(always)]
    #[must_use]
    pub fn height1(&mut self) -> HEIGHT1_W<16> {
        HEIGHT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame dimension (field 1).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr1](index.html) module"]
pub struct FDR1_SPEC;
impl crate::RegisterSpec for FDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdr1::R](R) reader structure"]
impl crate::Readable for FDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdr1::W](W) writer structure"]
impl crate::Writable for FDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDR1 to value 0x00ef_0000"]
impl crate::Resettable for FDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ef_0000;
}
