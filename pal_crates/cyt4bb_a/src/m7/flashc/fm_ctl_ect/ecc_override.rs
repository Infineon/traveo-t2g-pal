#[doc = "Register `ECC_OVERRIDE` writer"]
pub struct W(crate::W<ECC_OVERRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_OVERRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ECC_OVERRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_OVERRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC_OVERRIDE_SYNDROME` writer - The override syndrome itself to be used in case one of the enables are set. It will take \\[7:0\\]
in the case of Code flash and \\[6:0\\]
in the case of work flash, to bypass the internal generated syndrome"]
pub type ECC_OVERRIDE_SYNDROME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECC_OVERRIDE_SPEC, u8, u8, 8, O>;
#[doc = "Field `ECC_OVERRIDE_WORK` writer - 0: no override. Using internal ECC engine to calculate the ECC of the Work Flash"]
pub type ECC_OVERRIDE_WORK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ECC_OVERRIDE_SPEC, bool, O>;
#[doc = "Field `ECC_OVERRIDE_CODE` writer - 0: no override. Using internal ECC engine to calculate the ECC of the Code Flash"]
pub type ECC_OVERRIDE_CODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ECC_OVERRIDE_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:7 - The override syndrome itself to be used in case one of the enables are set. It will take \\[7:0\\]
in the case of Code flash and \\[6:0\\]
in the case of work flash, to bypass the internal generated syndrome"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_override_syndrome(&mut self) -> ECC_OVERRIDE_SYNDROME_W<0> {
        ECC_OVERRIDE_SYNDROME_W::new(self)
    }
    #[doc = "Bit 30 - 0: no override. Using internal ECC engine to calculate the ECC of the Work Flash"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_override_work(&mut self) -> ECC_OVERRIDE_WORK_W<30> {
        ECC_OVERRIDE_WORK_W::new(self)
    }
    #[doc = "Bit 31 - 0: no override. Using internal ECC engine to calculate the ECC of the Code Flash"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_override_code(&mut self) -> ECC_OVERRIDE_CODE_W<31> {
        ECC_OVERRIDE_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Data In override information and control bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_override](index.html) module"]
pub struct ECC_OVERRIDE_SPEC;
impl crate::RegisterSpec for ECC_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ecc_override::W](W) writer structure"]
impl crate::Writable for ECC_OVERRIDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_OVERRIDE to value 0"]
impl crate::Resettable for ECC_OVERRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
