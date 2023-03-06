#[doc = "Register `INTR_INFO2` reader"]
pub struct R(crate::R<INTR_INFO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_INFO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_INFO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_INFO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HMASTER` reader - The master ID of the master that made the access causing the violation (taken from the AHB HMASTER signal)"]
pub type HMASTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HNONSEC` reader - The security status of the access address causing the violation (taken from the AHB5 HNONSEC signal)."]
pub type HNONSEC_R = crate::BitReader<bool>;
#[doc = "Field `CFG_NS` reader - The secure/non-secure configuration of the block access attempt causing the violation."]
pub type CFG_NS_R = crate::BitReader<bool>;
#[doc = "Field `HWRITE` reader - The R/W status from which the violating access was made."]
pub type HWRITE_R = crate::BitReader<bool>;
#[doc = "Field `HAUSER` reader - The protection context from which the violating access was made (taken from the AHB5 HAUSER signal)."]
pub type HAUSER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY_VIOLATION` reader - This bit is set when a secure access was done to a non-secure block of memory, or a non-secure access was done to a secure block of memory."]
pub type SECURITY_VIOLATION_R = crate::BitReader<bool>;
#[doc = "Field `ACCESS_VIOLATION` reader - This bit is set when a read or write transaction was done from a protection context that does not have access to this block of memory."]
pub type ACCESS_VIOLATION_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - The master ID of the master that made the access causing the violation (taken from the AHB HMASTER signal)"]
    #[inline(always)]
    pub fn hmaster(&self) -> HMASTER_R {
        HMASTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - The security status of the access address causing the violation (taken from the AHB5 HNONSEC signal)."]
    #[inline(always)]
    pub fn hnonsec(&self) -> HNONSEC_R {
        HNONSEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The secure/non-secure configuration of the block access attempt causing the violation."]
    #[inline(always)]
    pub fn cfg_ns(&self) -> CFG_NS_R {
        CFG_NS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The R/W status from which the violating access was made."]
    #[inline(always)]
    pub fn hwrite(&self) -> HWRITE_R {
        HWRITE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:27 - The protection context from which the violating access was made (taken from the AHB5 HAUSER signal)."]
    #[inline(always)]
    pub fn hauser(&self) -> HAUSER_R {
        HAUSER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - This bit is set when a secure access was done to a non-secure block of memory, or a non-secure access was done to a secure block of memory."]
    #[inline(always)]
    pub fn security_violation(&self) -> SECURITY_VIOLATION_R {
        SECURITY_VIOLATION_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set when a read or write transaction was done from a protection context that does not have access to this block of memory."]
    #[inline(always)]
    pub fn access_violation(&self) -> ACCESS_VIOLATION_R {
        ACCESS_VIOLATION_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Infor about violation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_info2](index.html) module"]
pub struct INTR_INFO2_SPEC;
impl crate::RegisterSpec for INTR_INFO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_info2::R](R) reader structure"]
impl crate::Readable for INTR_INFO2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_INFO2 to value 0"]
impl crate::Resettable for INTR_INFO2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
