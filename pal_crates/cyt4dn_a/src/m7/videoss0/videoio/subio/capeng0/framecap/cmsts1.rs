#[doc = "Register `CMSTS1` reader"]
pub struct R(crate::R<CMSTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMSTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMSTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMSTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMSTS1` writer"]
pub struct W(crate::W<CMSTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMSTS1_SPEC>;
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
impl From<crate::W<CMSTS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMSTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTAL_TIME` reader - clk_axi cycle number of a frame starting from vsync to another vsync."]
pub type TOTAL_TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOTAL_TIME` writer - clk_axi cycle number of a frame starting from vsync to another vsync."]
pub type TOTAL_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMSTS1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - clk_axi cycle number of a frame starting from vsync to another vsync."]
    #[inline(always)]
    pub fn total_time(&self) -> TOTAL_TIME_R {
        TOTAL_TIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clk_axi cycle number of a frame starting from vsync to another vsync."]
    #[inline(always)]
    #[must_use]
    pub fn total_time(&mut self) -> TOTAL_TIME_W<0> {
        TOTAL_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_axi cycle number of a frame. (bit locked when MdrCmrDone=1).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmsts1](index.html) module"]
pub struct CMSTS1_SPEC;
impl crate::RegisterSpec for CMSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmsts1::R](R) reader structure"]
impl crate::Readable for CMSTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmsts1::W](W) writer structure"]
impl crate::Writable for CMSTS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMSTS1 to value 0"]
impl crate::Resettable for CMSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
