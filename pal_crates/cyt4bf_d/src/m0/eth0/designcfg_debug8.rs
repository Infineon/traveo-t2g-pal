#[doc = "Register `DESIGNCFG_DEBUG8` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_SCR2_COMPARE_REGS` reader - Takes the value of the `num_scr2_compare_regs DEFINE"]
pub type NUM_SCR2_COMPARE_REGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_SCR2_ETHTYPE_REGS` reader - Takes the value of the `num_scr2_ethtype_regs DEFINE"]
pub type NUM_SCR2_ETHTYPE_REGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_TYPE2_SCREENERS` reader - Takes the value of the `num_type2_screeners DEFINE"]
pub type NUM_TYPE2_SCREENERS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_TYPE1_SCREENERS` reader - Takes the value of the `num_type1_screeners DEFINE"]
pub type NUM_TYPE1_SCREENERS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Takes the value of the `num_scr2_compare_regs DEFINE"]
    #[inline(always)]
    pub fn num_scr2_compare_regs(&self) -> NUM_SCR2_COMPARE_REGS_R {
        NUM_SCR2_COMPARE_REGS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Takes the value of the `num_scr2_ethtype_regs DEFINE"]
    #[inline(always)]
    pub fn num_scr2_ethtype_regs(&self) -> NUM_SCR2_ETHTYPE_REGS_R {
        NUM_SCR2_ETHTYPE_REGS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Takes the value of the `num_type2_screeners DEFINE"]
    #[inline(always)]
    pub fn num_type2_screeners(&self) -> NUM_TYPE2_SCREENERS_R {
        NUM_TYPE2_SCREENERS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Takes the value of the `num_type1_screeners DEFINE"]
    #[inline(always)]
    pub fn num_type1_screeners(&self) -> NUM_TYPE1_SCREENERS_R {
        NUM_TYPE1_SCREENERS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Design Configuration Register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug8](index.html) module"]
pub struct DESIGNCFG_DEBUG8_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug8::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG8 to value 0x1010_0820"]
impl crate::Resettable for DESIGNCFG_DEBUG8_SPEC {
    const RESET_VALUE: Self::Ux = 0x1010_0820;
}
