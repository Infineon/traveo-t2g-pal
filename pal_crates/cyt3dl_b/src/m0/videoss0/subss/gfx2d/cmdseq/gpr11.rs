#[doc = "Register `GPR11` reader"]
pub struct R(crate::R<GPR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER11` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER11_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister11(&self) -> GENERALPURPOSEREGISTER11_R {
        GENERALPURPOSEREGISTER11_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 11)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr11](index.html) module"]
pub struct GPR11_SPEC;
impl crate::RegisterSpec for GPR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr11::R](R) reader structure"]
impl crate::Readable for GPR11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR11 to value 0"]
impl crate::Resettable for GPR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
