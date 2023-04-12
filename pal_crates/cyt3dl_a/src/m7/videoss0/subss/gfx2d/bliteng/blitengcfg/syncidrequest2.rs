#[doc = "Register `SYNCIDREQUEST2` reader"]
pub struct R(crate::R<SYNCIDREQUEST2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDREQUEST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDREQUEST2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDREQUEST2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCIDREQUEST2` writer"]
pub struct W(crate::W<SYNCIDREQUEST2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCIDREQUEST2_SPEC>;
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
impl From<crate::W<SYNCIDREQUEST2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCIDREQUEST2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCIDREQUEST2` reader - Sync ID requested by SW. Will be loaded into SyncIdStatus2 if SyncIdTrigger\\[2\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNCIDREQUEST2` writer - Sync ID requested by SW. Will be loaded into SyncIdStatus2 if SyncIdTrigger\\[2\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYNCIDREQUEST2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus2 if SyncIdTrigger\\[2\\]
is set and if/once store operation is done."]
    #[inline(always)]
    pub fn syncidrequest2(&self) -> SYNCIDREQUEST2_R {
        SYNCIDREQUEST2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus2 if SyncIdTrigger\\[2\\]
is set and if/once store operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn syncidrequest2(&mut self) -> SYNCIDREQUEST2_W<0> {
        SYNCIDREQUEST2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync ID request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidrequest2](index.html) module"]
pub struct SYNCIDREQUEST2_SPEC;
impl crate::RegisterSpec for SYNCIDREQUEST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidrequest2::R](R) reader structure"]
impl crate::Readable for SYNCIDREQUEST2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncidrequest2::W](W) writer structure"]
impl crate::Writable for SYNCIDREQUEST2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCIDREQUEST2 to value 0"]
impl crate::Resettable for SYNCIDREQUEST2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
