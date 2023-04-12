#[doc = "Register `DEV_KEY_STATUS` reader"]
pub struct R(crate::R<DEV_KEY_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_KEY_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_KEY_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_KEY_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOADED` reader - Specifies if a device key is present in the IP register buffer blocks 4 and 5. HW sets this field to '1' on successful completion of a LOAD_DEV_KEY instruction. HW clears this field to '0' when a CLEAR instruction is executed (the CLEAR instruction also sets the IP register buffer to '0')."]
pub type LOADED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Specifies if a device key is present in the IP register buffer blocks 4 and 5. HW sets this field to '1' on successful completion of a LOAD_DEV_KEY instruction. HW clears this field to '0' when a CLEAR instruction is executed (the CLEAR instruction also sets the IP register buffer to '0')."]
    #[inline(always)]
    pub fn loaded(&self) -> LOADED_R {
        LOADED_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Device key status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_key_status](index.html) module"]
pub struct DEV_KEY_STATUS_SPEC;
impl crate::RegisterSpec for DEV_KEY_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_key_status::R](R) reader structure"]
impl crate::Readable for DEV_KEY_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEV_KEY_STATUS to value 0"]
impl crate::Resettable for DEV_KEY_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
