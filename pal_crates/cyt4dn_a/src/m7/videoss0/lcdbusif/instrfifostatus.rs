#[doc = "Register `INSTRFIFOSTATUS` reader"]
pub struct R(crate::R<INSTRFIFOSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTRFIFOSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTRFIFOSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTRFIFOSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INSTRSPACE` reader - Available space in InstructionregistFifo in number of entries."]
pub type INSTRSPACE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTREMPTY` reader - Indicates if InstructionFifo is empty."]
pub type INSTREMPTY_R = crate::BitReader<bool>;
#[doc = "Field `INSTRFULL` reader - Indicates if InstructionFifo is full."]
pub type INSTRFULL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Available space in InstructionregistFifo in number of entries."]
    #[inline(always)]
    pub fn instrspace(&self) -> INSTRSPACE_R {
        INSTRSPACE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Indicates if InstructionFifo is empty."]
    #[inline(always)]
    pub fn instrempty(&self) -> INSTREMPTY_R {
        INSTREMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates if InstructionFifo is full."]
    #[inline(always)]
    pub fn instrfull(&self) -> INSTRFULL_R {
        INSTRFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Instruction fifo status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instrfifostatus](index.html) module"]
pub struct INSTRFIFOSTATUS_SPEC;
impl crate::RegisterSpec for INSTRFIFOSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [instrfifostatus::R](R) reader structure"]
impl crate::Readable for INSTRFIFOSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INSTRFIFOSTATUS to value 0x0120"]
impl crate::Resettable for INSTRFIFOSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0120;
}
