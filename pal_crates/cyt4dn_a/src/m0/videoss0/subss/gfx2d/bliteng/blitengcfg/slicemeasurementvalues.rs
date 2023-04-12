#[doc = "Register `SLICEMEASUREMENTVALUES` reader"]
pub struct R(crate::R<SLICEMEASUREMENTVALUES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLICEMEASUREMENTVALUES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLICEMEASUREMENTVALUES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLICEMEASUREMENTVALUES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLICEMEASUREMENTVALUES` writer"]
pub struct W(crate::W<SLICEMEASUREMENTVALUES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLICEMEASUREMENTVALUES_SPEC>;
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
impl From<crate::W<SLICEMEASUREMENTVALUES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLICEMEASUREMENTVALUES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLICEMEASUREMENTTB` reader - The id of taskbuffer to which this measurement corresponds."]
pub type SLICEMEASUREMENTTB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLICEMEASUREMENTTB` writer - The id of taskbuffer to which this measurement corresponds."]
pub type SLICEMEASUREMENTTB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLICEMEASUREMENTVALUES_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLICEMEASUREMENTOVERRUN` reader - The measured slice duration is not valid because of the overrun. The measurement itself can continue, all other slice durations are still valid."]
pub type SLICEMEASUREMENTOVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `SLICEMEASUREMENTOVERRUN` writer - The measured slice duration is not valid because of the overrun. The measurement itself can continue, all other slice durations are still valid."]
pub type SLICEMEASUREMENTOVERRUN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLICEMEASUREMENTVALUES_SPEC, bool, O>;
#[doc = "Field `SLICEMEASUREMENTDURATION` reader - The measured duration of the slice in clock cylces"]
pub type SLICEMEASUREMENTDURATION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLICEMEASUREMENTDURATION` writer - The measured duration of the slice in clock cylces"]
pub type SLICEMEASUREMENTDURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLICEMEASUREMENTVALUES_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:2 - The id of taskbuffer to which this measurement corresponds."]
    #[inline(always)]
    pub fn slicemeasurementtb(&self) -> SLICEMEASUREMENTTB_R {
        SLICEMEASUREMENTTB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - The measured slice duration is not valid because of the overrun. The measurement itself can continue, all other slice durations are still valid."]
    #[inline(always)]
    pub fn slicemeasurementoverrun(&self) -> SLICEMEASUREMENTOVERRUN_R {
        SLICEMEASUREMENTOVERRUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - The measured duration of the slice in clock cylces"]
    #[inline(always)]
    pub fn slicemeasurementduration(&self) -> SLICEMEASUREMENTDURATION_R {
        SLICEMEASUREMENTDURATION_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - The id of taskbuffer to which this measurement corresponds."]
    #[inline(always)]
    #[must_use]
    pub fn slicemeasurementtb(&mut self) -> SLICEMEASUREMENTTB_W<0> {
        SLICEMEASUREMENTTB_W::new(self)
    }
    #[doc = "Bit 3 - The measured slice duration is not valid because of the overrun. The measurement itself can continue, all other slice durations are still valid."]
    #[inline(always)]
    #[must_use]
    pub fn slicemeasurementoverrun(&mut self) -> SLICEMEASUREMENTOVERRUN_W<3> {
        SLICEMEASUREMENTOVERRUN_W::new(self)
    }
    #[doc = "Bits 4:31 - The measured duration of the slice in clock cylces"]
    #[inline(always)]
    #[must_use]
    pub fn slicemeasurementduration(&mut self) -> SLICEMEASUREMENTDURATION_W<4> {
        SLICEMEASUREMENTDURATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Performance measurement for each slice. Measurement is active only if SlicePerfMeasurementEn is set. This register can be always read,\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slicemeasurementvalues](index.html) module"]
pub struct SLICEMEASUREMENTVALUES_SPEC;
impl crate::RegisterSpec for SLICEMEASUREMENTVALUES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slicemeasurementvalues::R](R) reader structure"]
impl crate::Readable for SLICEMEASUREMENTVALUES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slicemeasurementvalues::W](W) writer structure"]
impl crate::Writable for SLICEMEASUREMENTVALUES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLICEMEASUREMENTVALUES to value 0"]
impl crate::Resettable for SLICEMEASUREMENTVALUES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
