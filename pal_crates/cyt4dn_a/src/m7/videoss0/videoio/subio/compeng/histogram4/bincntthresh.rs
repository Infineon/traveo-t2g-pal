#[doc = "Register `BINCNTTHRESH` reader"]
pub struct R(crate::R<BINCNTTHRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BINCNTTHRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BINCNTTHRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BINCNTTHRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BINCNTTHRESH` writer"]
pub struct W(crate::W<BINCNTTHRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BINCNTTHRESH_SPEC>;
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
impl From<crate::W<BINCNTTHRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BINCNTTHRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BINCNT_THRESH` reader - Bin counters below this threshold will be reported as 0. This threshold value will also affect the BINCNT_ACCU mode where 0 will be added for all bin counters below the threshold."]
pub type BINCNT_THRESH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BINCNT_THRESH` writer - Bin counters below this threshold will be reported as 0. This threshold value will also affect the BINCNT_ACCU mode where 0 will be added for all bin counters below the threshold."]
pub type BINCNT_THRESH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BINCNTTHRESH_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - Bin counters below this threshold will be reported as 0. This threshold value will also affect the BINCNT_ACCU mode where 0 will be added for all bin counters below the threshold."]
    #[inline(always)]
    pub fn bincnt_thresh(&self) -> BINCNT_THRESH_R {
        BINCNT_THRESH_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Bin counters below this threshold will be reported as 0. This threshold value will also affect the BINCNT_ACCU mode where 0 will be added for all bin counters below the threshold."]
    #[inline(always)]
    #[must_use]
    pub fn bincnt_thresh(&mut self) -> BINCNT_THRESH_W<0> {
        BINCNT_THRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bin counter threshold value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bincntthresh](index.html) module"]
pub struct BINCNTTHRESH_SPEC;
impl crate::RegisterSpec for BINCNTTHRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bincntthresh::R](R) reader structure"]
impl crate::Readable for BINCNTTHRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bincntthresh::W](W) writer structure"]
impl crate::Writable for BINCNTTHRESH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BINCNTTHRESH to value 0"]
impl crate::Resettable for BINCNTTHRESH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
