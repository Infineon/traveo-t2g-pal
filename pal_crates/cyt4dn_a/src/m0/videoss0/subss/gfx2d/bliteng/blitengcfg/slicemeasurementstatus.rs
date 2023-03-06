#[doc = "Register `SLICEMEASUREMENTSTATUS` reader"]
pub struct R(crate::R<SLICEMEASUREMENTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLICEMEASUREMENTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLICEMEASUREMENTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLICEMEASUREMENTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLICEMEASUREMENTNUMBER` reader - The number of measurements that can be read from SliceMeasurementValues register. Incremented with each slice being finished and decremented after each read from SliceMeasurementValues."]
pub type SLICEMEASUREMENTNUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLICEMEASUREMENTMAXNUMBER` reader - The maximal number of slice measurements that can be stored."]
pub type SLICEMEASUREMENTMAXNUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLICEMEASUREMENTVALID` reader - This bit is one if the results of measurement are valid. If this bit is zero, the mesurement is stopped and has to be restarted via SlicePerfMeasurementEn."]
pub type SLICEMEASUREMENTVALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - The number of measurements that can be read from SliceMeasurementValues register. Incremented with each slice being finished and decremented after each read from SliceMeasurementValues."]
    #[inline(always)]
    pub fn slicemeasurementnumber(&self) -> SLICEMEASUREMENTNUMBER_R {
        SLICEMEASUREMENTNUMBER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The maximal number of slice measurements that can be stored."]
    #[inline(always)]
    pub fn slicemeasurementmaxnumber(&self) -> SLICEMEASUREMENTMAXNUMBER_R {
        SLICEMEASUREMENTMAXNUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is one if the results of measurement are valid. If this bit is zero, the mesurement is stopped and has to be restarted via SlicePerfMeasurementEn."]
    #[inline(always)]
    pub fn slicemeasurementvalid(&self) -> SLICEMEASUREMENTVALID_R {
        SLICEMEASUREMENTVALID_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Performance measurement register for BlitEng containing idle cycles. Measurement is active only if SlicePerfMeasurementEn is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slicemeasurementstatus](index.html) module"]
pub struct SLICEMEASUREMENTSTATUS_SPEC;
impl crate::RegisterSpec for SLICEMEASUREMENTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slicemeasurementstatus::R](R) reader structure"]
impl crate::Readable for SLICEMEASUREMENTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLICEMEASUREMENTSTATUS to value 0"]
impl crate::Resettable for SLICEMEASUREMENTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
