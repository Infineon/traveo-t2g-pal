#[doc = "Register `ERROR` reader"]
pub struct R(crate::R<ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FETCHERROR` reader - Indicates if an bus error occurred while fetching the command list."]
pub type FETCHERROR_R = crate::BitReader<bool>;
#[doc = "Field `STOREERROR` reader - Indicates if storing the alpha bitmap to memory failed."]
pub type STOREERROR_R = crate::BitReader<bool>;
#[doc = "Field `INSTRERROR` reader - Indicates if the command list contained an illegal or unexpected instruction."]
pub type INSTRERROR_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOWERROR` reader - Indicates if an overflow occurred when truncating to internal data width."]
pub type OVERFLOWERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates if an bus error occurred while fetching the command list."]
    #[inline(always)]
    pub fn fetcherror(&self) -> FETCHERROR_R {
        FETCHERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if storing the alpha bitmap to memory failed."]
    #[inline(always)]
    pub fn storeerror(&self) -> STOREERROR_R {
        STOREERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if the command list contained an illegal or unexpected instruction."]
    #[inline(always)]
    pub fn instrerror(&self) -> INSTRERROR_R {
        INSTRERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if an overflow occurred when truncating to internal data width."]
    #[inline(always)]
    pub fn overflowerror(&self) -> OVERFLOWERROR_R {
        OVERFLOWERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Error status of finished operation. If register is unequal to zero the resulting raster image is most likely corrupted.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error](index.html) module"]
pub struct ERROR_SPEC;
impl crate::RegisterSpec for ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error::R](R) reader structure"]
impl crate::Readable for ERROR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERROR to value 0"]
impl crate::Resettable for ERROR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
