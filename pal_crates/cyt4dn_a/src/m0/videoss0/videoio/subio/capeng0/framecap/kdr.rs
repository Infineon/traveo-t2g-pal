#[doc = "Register `KDR` reader"]
pub struct R(crate::R<KDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KDR` writer"]
pub struct W(crate::W<KDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KDR_SPEC>;
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
impl From<crate::W<KDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KICKDEL` reader - Delay after receiving last active pixel in frame (in video clock cycles) until the kick pulse is generated. In any case the kick pulse is generated with first active pixel of next frame latest."]
pub type KICKDEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KICKDEL` writer - Delay after receiving last active pixel in frame (in video clock cycles) until the kick pulse is generated. In any case the kick pulse is generated with first active pixel of next frame latest."]
pub type KICKDEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KDR_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Delay after receiving last active pixel in frame (in video clock cycles) until the kick pulse is generated. In any case the kick pulse is generated with first active pixel of next frame latest."]
    #[inline(always)]
    pub fn kickdel(&self) -> KICKDEL_R {
        KICKDEL_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Delay after receiving last active pixel in frame (in video clock cycles) until the kick pulse is generated. In any case the kick pulse is generated with first active pixel of next frame latest."]
    #[inline(always)]
    #[must_use]
    pub fn kickdel(&mut self) -> KICKDEL_W<0> {
        KICKDEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap kick delay configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kdr](index.html) module"]
pub struct KDR_SPEC;
impl crate::RegisterSpec for KDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kdr::R](R) reader structure"]
impl crate::Readable for KDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kdr::W](W) writer structure"]
impl crate::Writable for KDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KDR to value 0"]
impl crate::Resettable for KDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
