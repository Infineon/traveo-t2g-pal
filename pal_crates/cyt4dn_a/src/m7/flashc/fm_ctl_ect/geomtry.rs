#[doc = "Register `GEOMTRY` reader"]
pub struct R(crate::R<GEOMTRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEOMTRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEOMTRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEOMTRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CODE_FLASH_DENSITY` reader - N/A"]
pub type CODE_FLASH_DENSITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WORK_FLASH_DENSITY` reader - N/A"]
pub type WORK_FLASH_DENSITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE_FLASH_SMS_NUMBER` reader - 0: 16 sectors 1: 32 sectors"]
pub type CODE_FLASH_SMS_NUMBER_R = crate::BitReader<bool>;
#[doc = "Field `OTP_SIZE_KB` reader - 0: 32KB 1: 64KB"]
pub type OTP_SIZE_KB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn code_flash_density(&self) -> CODE_FLASH_DENSITY_R {
        CODE_FLASH_DENSITY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - N/A"]
    #[inline(always)]
    pub fn work_flash_density(&self) -> WORK_FLASH_DENSITY_R {
        WORK_FLASH_DENSITY_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 0: 16 sectors 1: 32 sectors"]
    #[inline(always)]
    pub fn code_flash_sms_number(&self) -> CODE_FLASH_SMS_NUMBER_R {
        CODE_FLASH_SMS_NUMBER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: 32KB 1: 64KB"]
    #[inline(always)]
    pub fn otp_size_kb(&self) -> OTP_SIZE_KB_R {
        OTP_SIZE_KB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Flash Density Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geomtry](index.html) module"]
pub struct GEOMTRY_SPEC;
impl crate::RegisterSpec for GEOMTRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [geomtry::R](R) reader structure"]
impl crate::Readable for GEOMTRY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GEOMTRY to value 0x13"]
impl crate::Resettable for GEOMTRY_SPEC {
    const RESET_VALUE: Self::Ux = 0x13;
}
