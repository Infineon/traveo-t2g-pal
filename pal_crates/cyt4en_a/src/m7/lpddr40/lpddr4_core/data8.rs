#[doc = "Register `DATA8` reader"]
pub struct R(crate::R<DATA8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDVALID` reader - Read Data Valid, active HIGH. When this field is HIGH: - User can read out RDDATA from data registers DATA0 ... DATA8 - This field will be cleared automatically after user read it out"]
pub type RDVALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read Data Valid, active HIGH. When this field is HIGH: - User can read out RDDATA from data registers DATA0 ... DATA8 - This field will be cleared automatically after user read it out"]
    #[inline(always)]
    pub fn rdvalid(&self) -> RDVALID_R {
        RDVALID_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Read Data Register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data8](index.html) module"]
pub struct DATA8_SPEC;
impl crate::RegisterSpec for DATA8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data8::R](R) reader structure"]
impl crate::Readable for DATA8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA8 to value 0"]
impl crate::Resettable for DATA8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
