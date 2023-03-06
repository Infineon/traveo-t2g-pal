#[doc = "Register `SYNCIDREQUEST0` reader"]
pub struct R(crate::R<SYNCIDREQUEST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDREQUEST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDREQUEST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDREQUEST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCIDREQUEST0` writer"]
pub struct W(crate::W<SYNCIDREQUEST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCIDREQUEST0_SPEC>;
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
impl From<crate::W<SYNCIDREQUEST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCIDREQUEST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCIDREQUEST0` reader - Sync ID requested by SW. Will be loaded into SyncIdStatus0 if SyncIdTrigger\\[0\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNCIDREQUEST0` writer - Sync ID requested by SW. Will be loaded into SyncIdStatus0 if SyncIdTrigger\\[0\\]
is set and if/once store operation is done."]
pub type SYNCIDREQUEST0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYNCIDREQUEST0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus0 if SyncIdTrigger\\[0\\]
is set and if/once store operation is done."]
    #[inline(always)]
    pub fn syncidrequest0(&self) -> SYNCIDREQUEST0_R {
        SYNCIDREQUEST0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sync ID requested by SW. Will be loaded into SyncIdStatus0 if SyncIdTrigger\\[0\\]
is set and if/once store operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn syncidrequest0(&mut self) -> SYNCIDREQUEST0_W<0> {
        SYNCIDREQUEST0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync ID request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidrequest0](index.html) module"]
pub struct SYNCIDREQUEST0_SPEC;
impl crate::RegisterSpec for SYNCIDREQUEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidrequest0::R](R) reader structure"]
impl crate::Readable for SYNCIDREQUEST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncidrequest0::W](W) writer structure"]
impl crate::Writable for SYNCIDREQUEST0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCIDREQUEST0 to value 0"]
impl crate::Resettable for SYNCIDREQUEST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
