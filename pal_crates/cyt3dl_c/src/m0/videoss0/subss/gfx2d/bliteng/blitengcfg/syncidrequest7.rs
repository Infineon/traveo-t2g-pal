#[doc = "Register `SYNCIDREQUEST7` reader"]
pub struct R(crate::R<SYNCIDREQUEST7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDREQUEST7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDREQUEST7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDREQUEST7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCIDREQUEST7` writer"]
pub struct W(crate::W<SYNCIDREQUEST7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCIDREQUEST7_SPEC>;
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
impl From<crate::W<SYNCIDREQUEST7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCIDREQUEST7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCIDREQUEST7` reader - Sync ID requested by SW. Will be loaded into SyncIdStatus7 if SyncIdTrigger\\[7\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNCIDREQUEST7` writer - Sync ID requested by SW. Will be loaded into SyncIdStatus7 if SyncIdTrigger\\[7\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYNCIDREQUEST7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus7 if SyncIdTrigger\\[7\\]
is set and if/once store operation is done."]
    #[inline(always)]
    pub fn syncidrequest7(&self) -> SYNCIDREQUEST7_R {
        SYNCIDREQUEST7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus7 if SyncIdTrigger\\[7\\]
is set and if/once store operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn syncidrequest7(&mut self) -> SYNCIDREQUEST7_W<0> {
        SYNCIDREQUEST7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync ID request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidrequest7](index.html) module"]
pub struct SYNCIDREQUEST7_SPEC;
impl crate::RegisterSpec for SYNCIDREQUEST7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidrequest7::R](R) reader structure"]
impl crate::Readable for SYNCIDREQUEST7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncidrequest7::W](W) writer structure"]
impl crate::Writable for SYNCIDREQUEST7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCIDREQUEST7 to value 0"]
impl crate::Resettable for SYNCIDREQUEST7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
