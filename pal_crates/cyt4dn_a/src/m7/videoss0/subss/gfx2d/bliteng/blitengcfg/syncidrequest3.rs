#[doc = "Register `SYNCIDREQUEST3` reader"]
pub struct R(crate::R<SYNCIDREQUEST3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDREQUEST3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDREQUEST3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDREQUEST3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCIDREQUEST3` writer"]
pub struct W(crate::W<SYNCIDREQUEST3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCIDREQUEST3_SPEC>;
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
impl From<crate::W<SYNCIDREQUEST3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCIDREQUEST3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCIDREQUEST3` reader - Sync ID requested by SW. Will be loaded into SyncIdStatus3 if SyncIdTrigger\\[3\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNCIDREQUEST3` writer - Sync ID requested by SW. Will be loaded into SyncIdStatus3 if SyncIdTrigger\\[3\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYNCIDREQUEST3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus3 if SyncIdTrigger\\[3\\]
is set and if/once store operation is done."]
    #[inline(always)]
    pub fn syncidrequest3(&self) -> SYNCIDREQUEST3_R {
        SYNCIDREQUEST3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus3 if SyncIdTrigger\\[3\\]
is set and if/once store operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn syncidrequest3(&mut self) -> SYNCIDREQUEST3_W<0> {
        SYNCIDREQUEST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync ID request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidrequest3](index.html) module"]
pub struct SYNCIDREQUEST3_SPEC;
impl crate::RegisterSpec for SYNCIDREQUEST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidrequest3::R](R) reader structure"]
impl crate::Readable for SYNCIDREQUEST3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncidrequest3::W](W) writer structure"]
impl crate::Writable for SYNCIDREQUEST3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCIDREQUEST3 to value 0"]
impl crate::Resettable for SYNCIDREQUEST3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
