#[doc = "Register `TRIGGER` reader"]
pub struct R(crate::R<TRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGGER` writer"]
pub struct W(crate::W<TRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGGER_SPEC>;
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
impl From<crate::W<TRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUMPREQ` reader - Next frame is dumped when this field is set. It is reset by HW once dumping of frame data has started. It cannot be reset by SW."]
pub type DUMPREQ_R = crate::BitReader<bool>;
#[doc = "Field `DUMPREQ` writer - Next frame is dumped when this field is set. It is reset by HW once dumping of frame data has started. It cannot be reset by SW."]
pub type DUMPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGGER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Next frame is dumped when this field is set. It is reset by HW once dumping of frame data has started. It cannot be reset by SW."]
    #[inline(always)]
    pub fn dumpreq(&self) -> DUMPREQ_R {
        DUMPREQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Next frame is dumped when this field is set. It is reset by HW once dumping of frame data has started. It cannot be reset by SW."]
    #[inline(always)]
    #[must_use]
    pub fn dumpreq(&mut self) -> DUMPREQ_W<0> {
        DUMPREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](index.html) module"]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigger::R](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigger::W](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
