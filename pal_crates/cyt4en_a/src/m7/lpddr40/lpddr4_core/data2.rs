#[doc = "Register `DATA2` reader"]
pub struct R(crate::R<DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDDATA` reader - Read Data \\[95:64\\]
for Loopback test mode"]
pub type RDDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data \\[95:64\\]
for Loopback test mode"]
    #[inline(always)]
    pub fn rddata(&self) -> RDDATA_R {
        RDDATA_R::new(self.bits)
    }
}
#[doc = "Read Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data2](index.html) module"]
pub struct DATA2_SPEC;
impl crate::RegisterSpec for DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data2::R](R) reader structure"]
impl crate::Readable for DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA2 to value 0"]
impl crate::Resettable for DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
