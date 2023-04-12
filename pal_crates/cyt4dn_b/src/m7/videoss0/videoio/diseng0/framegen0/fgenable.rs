#[doc = "Register `FGENABLE` reader"]
pub struct R(crate::R<FGENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGENABLE` writer"]
pub struct W(crate::W<FGENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGENABLE_SPEC>;
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
impl From<crate::W<FGENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGEN` reader - Frame Generator Enable. Note: For loading the shadow registers at start-up of FrameGen unit write the FgSlr.ShdTokGen field immediately before writing this field."]
pub type FGEN_R = crate::BitReader<bool>;
#[doc = "Field `FGEN` writer - Frame Generator Enable. Note: For loading the shadow registers at start-up of FrameGen unit write the FgSlr.ShdTokGen field immediately before writing this field."]
pub type FGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Frame Generator Enable. Note: For loading the shadow registers at start-up of FrameGen unit write the FgSlr.ShdTokGen field immediately before writing this field."]
    #[inline(always)]
    pub fn fgen(&self) -> FGEN_R {
        FGEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Generator Enable. Note: For loading the shadow registers at start-up of FrameGen unit write the FgSlr.ShdTokGen field immediately before writing this field."]
    #[inline(always)]
    #[must_use]
    pub fn fgen(&mut self) -> FGEN_W<0> {
        FGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgenable](index.html) module"]
pub struct FGENABLE_SPEC;
impl crate::RegisterSpec for FGENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgenable::R](R) reader structure"]
impl crate::Readable for FGENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgenable::W](W) writer structure"]
impl crate::Writable for FGENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGENABLE to value 0"]
impl crate::Resettable for FGENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
