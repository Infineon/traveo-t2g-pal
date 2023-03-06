#[doc = "Register `USER_IO_REGISTER` reader"]
pub struct R(crate::R<USER_IO_REGISTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_IO_REGISTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_IO_REGISTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_IO_REGISTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSVD_31_0` reader - Write ignore, read 0"]
pub type RSVD_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_31_0(&self) -> RSVD_31_0_R {
        RSVD_31_0_R::new(self.bits)
    }
}
#[doc = "Not presents. Access to the register will return AHB error.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user_io_register](index.html) module"]
pub struct USER_IO_REGISTER_SPEC;
impl crate::RegisterSpec for USER_IO_REGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user_io_register::R](R) reader structure"]
impl crate::Readable for USER_IO_REGISTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USER_IO_REGISTER to value 0"]
impl crate::Resettable for USER_IO_REGISTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
