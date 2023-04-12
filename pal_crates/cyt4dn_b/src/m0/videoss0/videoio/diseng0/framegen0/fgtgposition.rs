#[doc = "Register `FGTGPOSITION` reader"]
pub struct R(crate::R<FGTGPOSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGTGPOSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGTGPOSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGTGPOSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRTGX` reader - Coordinated pixel position starting from HACT inside a total line generated in timing generator in display clock cycles. (For debugging purposes only)"]
pub type FRTGX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRTGY` reader - Coordinated line position starting from VACT inside a total frame generated in timing generator in display clock cycles. (For debugging purposes only)"]
pub type FRTGY_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Coordinated pixel position starting from HACT inside a total line generated in timing generator in display clock cycles. (For debugging purposes only)"]
    #[inline(always)]
    pub fn frtgx(&self) -> FRTGX_R {
        FRTGX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27 - Coordinated line position starting from VACT inside a total frame generated in timing generator in display clock cycles. (For debugging purposes only)"]
    #[inline(always)]
    pub fn frtgy(&self) -> FRTGY_R {
        FRTGY_R::new(((self.bits >> 14) & 0x3fff) as u16)
    }
}
#[doc = "Current (x,y) positions of the timing generator Register in FrameGen timing generator. Please don't use it due to lack of verification.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgtgposition](index.html) module"]
pub struct FGTGPOSITION_SPEC;
impl crate::RegisterSpec for FGTGPOSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgtgposition::R](R) reader structure"]
impl crate::Readable for FGTGPOSITION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGTGPOSITION to value 0x003f_0000"]
impl crate::Resettable for FGTGPOSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_0000;
}
