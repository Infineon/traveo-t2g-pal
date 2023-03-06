#[doc = "Register `TREG2` reader"]
pub struct R(crate::R<TREG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG2` writer"]
pub struct W(crate::W<TREG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG2_SPEC>;
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
impl From<crate::W<TREG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RC` reader - ACT to ACT or REF command period = tRC - 8"]
pub type T_RC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RC` writer - ACT to ACT or REF command period = tRC - 8"]
pub type T_RC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG2_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_RAS` reader - ACTIVE to PRECHARGE command period = tRAS_min - 4"]
pub type T_RAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RAS` writer - ACTIVE to PRECHARGE command period = tRAS_min - 4"]
pub type T_RAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG2_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_RDPDEN` reader - Timing of RD/RDA command to Power Down entry = RL + BL/2 +1"]
pub type T_RDPDEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RDPDEN` writer - Timing of RD/RDA command to Power Down entry = RL + BL/2 +1"]
pub type T_RDPDEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG2_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_DLLRST` reader - DLL reset time. The number of DTI_CLOCKs for which the DLL_RESET must remain asserted when reset is triggered though the POM register. Always write 10."]
pub type T_DLLRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_DLLRST` writer - DLL reset time. The number of DTI_CLOCKs for which the DLL_RESET must remain asserted when reset is triggered though the POM register. Always write 10."]
pub type T_DLLRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ACT to ACT or REF command period = tRC - 8"]
    #[inline(always)]
    pub fn t_rc(&self) -> T_RC_R {
        T_RC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ACTIVE to PRECHARGE command period = tRAS_min - 4"]
    #[inline(always)]
    pub fn t_ras(&self) -> T_RAS_R {
        T_RAS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timing of RD/RDA command to Power Down entry = RL + BL/2 +1"]
    #[inline(always)]
    pub fn t_rdpden(&self) -> T_RDPDEN_R {
        T_RDPDEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DLL reset time. The number of DTI_CLOCKs for which the DLL_RESET must remain asserted when reset is triggered though the POM register. Always write 10."]
    #[inline(always)]
    pub fn t_dllrst(&self) -> T_DLLRST_R {
        T_DLLRST_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ACT to ACT or REF command period = tRC - 8"]
    #[inline(always)]
    #[must_use]
    pub fn t_rc(&mut self) -> T_RC_W<0> {
        T_RC_W::new(self)
    }
    #[doc = "Bits 8:15 - ACTIVE to PRECHARGE command period = tRAS_min - 4"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras(&mut self) -> T_RAS_W<8> {
        T_RAS_W::new(self)
    }
    #[doc = "Bits 16:23 - Timing of RD/RDA command to Power Down entry = RL + BL/2 +1"]
    #[inline(always)]
    #[must_use]
    pub fn t_rdpden(&mut self) -> T_RDPDEN_W<16> {
        T_RDPDEN_W::new(self)
    }
    #[doc = "Bits 24:31 - DLL reset time. The number of DTI_CLOCKs for which the DLL_RESET must remain asserted when reset is triggered though the POM register. Always write 10."]
    #[inline(always)]
    #[must_use]
    pub fn t_dllrst(&mut self) -> T_DLLRST_W<24> {
        T_DLLRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg2](index.html) module"]
pub struct TREG2_SPEC;
impl crate::RegisterSpec for TREG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg2::R](R) reader structure"]
impl crate::Readable for TREG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg2::W](W) writer structure"]
impl crate::Writable for TREG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG2 to value 0x0a00_0000"]
impl crate::Resettable for TREG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00_0000;
}
