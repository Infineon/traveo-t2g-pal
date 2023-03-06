#[doc = "Register `TREG10` reader"]
pub struct R(crate::R<TREG10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG10` writer"]
pub struct W(crate::W<TREG10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG10_SPEC>;
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
impl From<crate::W<TREG10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RFC` reader - Refresh Cycle Time (All Banks) = tRFCab - 4"]
pub type T_RFC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_RFC` writer - Refresh Cycle Time (All Banks) = tRFCab - 4"]
pub type T_RFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG10_SPEC, u16, u16, 10, O>;
#[doc = "Field `T_MRS2ACT` reader - MRW leveling enable to load pulse = tWLMRD + 10"]
pub type T_MRS2ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MRS2ACT` writer - MRW leveling enable to load pulse = tWLMRD + 10"]
pub type T_MRS2ACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG10_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_LVLLOAD` reader - Delay settling time. Specifies the minimum number of DFI clock cycles from when the delays are loaded on the DTI_*_DLY signal to when the DTI_*_LOAD signal may be asserted. = 8"]
pub type T_LVLLOAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_LVLLOAD` writer - Delay settling time. Specifies the minimum number of DFI clock cycles from when the delays are loaded on the DTI_*_DLY signal to when the DTI_*_LOAD signal may be asserted. = 8"]
pub type T_LVLLOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG10_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_LVLDLL` reader - DLL delay. Specifies the minimum number of DFI clock cycles from when DTI_*_LOAD signal updates the DLL delay in the appropriate DTI_*_DLY to when the PHY is ready for the next read or mode register read command. = 15"]
pub type T_LVLDLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_LVLDLL` writer - DLL delay. Specifies the minimum number of DFI clock cycles from when DTI_*_LOAD signal updates the DLL delay in the appropriate DTI_*_DLY to when the PHY is ready for the next read or mode register read command. = 15"]
pub type T_LVLDLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG10_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_LVLRESP` reader - Leveling response latency. Specifies the maximum number of DFI clock cycles from the assertion of a read or mode register read command to the guaranteed validity of the response signal. = 90"]
pub type T_LVLRESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_LVLRESP` writer - Leveling response latency. Specifies the maximum number of DFI clock cycles from the assertion of a read or mode register read command to the guaranteed validity of the response signal. = 90"]
pub type T_LVLRESP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG10_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:9 - Refresh Cycle Time (All Banks) = tRFCab - 4"]
    #[inline(always)]
    pub fn t_rfc(&self) -> T_RFC_R {
        T_RFC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - MRW leveling enable to load pulse = tWLMRD + 10"]
    #[inline(always)]
    pub fn t_mrs2act(&self) -> T_MRS2ACT_R {
        T_MRS2ACT_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Delay settling time. Specifies the minimum number of DFI clock cycles from when the delays are loaded on the DTI_*_DLY signal to when the DTI_*_LOAD signal may be asserted. = 8"]
    #[inline(always)]
    pub fn t_lvlload(&self) -> T_LVLLOAD_R {
        T_LVLLOAD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DLL delay. Specifies the minimum number of DFI clock cycles from when DTI_*_LOAD signal updates the DLL delay in the appropriate DTI_*_DLY to when the PHY is ready for the next read or mode register read command. = 15"]
    #[inline(always)]
    pub fn t_lvldll(&self) -> T_LVLDLL_R {
        T_LVLDLL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Leveling response latency. Specifies the maximum number of DFI clock cycles from the assertion of a read or mode register read command to the guaranteed validity of the response signal. = 90"]
    #[inline(always)]
    pub fn t_lvlresp(&self) -> T_LVLRESP_R {
        T_LVLRESP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Refresh Cycle Time (All Banks) = tRFCab - 4"]
    #[inline(always)]
    #[must_use]
    pub fn t_rfc(&mut self) -> T_RFC_W<0> {
        T_RFC_W::new(self)
    }
    #[doc = "Bits 10:15 - MRW leveling enable to load pulse = tWLMRD + 10"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrs2act(&mut self) -> T_MRS2ACT_W<10> {
        T_MRS2ACT_W::new(self)
    }
    #[doc = "Bits 16:19 - Delay settling time. Specifies the minimum number of DFI clock cycles from when the delays are loaded on the DTI_*_DLY signal to when the DTI_*_LOAD signal may be asserted. = 8"]
    #[inline(always)]
    #[must_use]
    pub fn t_lvlload(&mut self) -> T_LVLLOAD_W<16> {
        T_LVLLOAD_W::new(self)
    }
    #[doc = "Bits 20:23 - DLL delay. Specifies the minimum number of DFI clock cycles from when DTI_*_LOAD signal updates the DLL delay in the appropriate DTI_*_DLY to when the PHY is ready for the next read or mode register read command. = 15"]
    #[inline(always)]
    #[must_use]
    pub fn t_lvldll(&mut self) -> T_LVLDLL_W<20> {
        T_LVLDLL_W::new(self)
    }
    #[doc = "Bits 24:31 - Leveling response latency. Specifies the maximum number of DFI clock cycles from the assertion of a read or mode register read command to the guaranteed validity of the response signal. = 90"]
    #[inline(always)]
    #[must_use]
    pub fn t_lvlresp(&mut self) -> T_LVLRESP_W<24> {
        T_LVLRESP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg10](index.html) module"]
pub struct TREG10_SPEC;
impl crate::RegisterSpec for TREG10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg10::R](R) reader structure"]
impl crate::Readable for TREG10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg10::W](W) writer structure"]
impl crate::Writable for TREG10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG10 to value 0x3244_c000"]
impl crate::Resettable for TREG10_SPEC {
    const RESET_VALUE: Self::Ux = 0x3244_c000;
}
