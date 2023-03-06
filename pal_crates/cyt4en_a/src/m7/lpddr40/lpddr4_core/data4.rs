#[doc = "Register `DATA4` reader"]
pub struct R(crate::R<DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDDATA` reader - Read Data \\[159:128\\]
for Loopback test mode"]
pub type RDDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data \\[159:128\\]
for Loopback test mode"]
    #[inline(always)]
    pub fn rddata(&self) -> RDDATA_R {
        RDDATA_R::new(self.bits)
    }
}
#[doc = "Read Data Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data4](index.html) module"]
pub struct DATA4_SPEC;
impl crate::RegisterSpec for DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data4::R](R) reader structure"]
impl crate::Readable for DATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA4 to value 0"]
impl crate::Resettable for DATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
