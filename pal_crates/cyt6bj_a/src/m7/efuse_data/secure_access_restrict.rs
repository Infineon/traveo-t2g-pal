#[doc = "Register `SECURE_ACCESS_RESTRICT` reader"]
pub struct R(crate::R<SECURE_ACCESS_RESTRICT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_ACCESS_RESTRICT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_ACCESS_RESTRICT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_ACCESS_RESTRICT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_ACCESS_RESTRICT` writer"]
pub struct W(crate::W<SECURE_ACCESS_RESTRICT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_ACCESS_RESTRICT_SPEC>;
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
impl From<crate::W<SECURE_ACCESS_RESTRICT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_ACCESS_RESTRICT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP_CTL_CM0_DISABLE` reader - goes to CPUSS.AP_CTL"]
pub type AP_CTL_CM0_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP_CTL_CM0_DISABLE` writer - goes to CPUSS.AP_CTL"]
pub type AP_CTL_CM0_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 2, O>;
#[doc = "Field `AP_CTL_CMX_DISABLE` reader - goes to CPUSS.AP_CTL"]
pub type AP_CTL_CMX_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP_CTL_CMX_DISABLE` writer - goes to CPUSS.AP_CTL"]
pub type AP_CTL_CMX_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 2, O>;
#[doc = "Field `AP_CTL_SYS_DISABLE` reader - goes to CPUSS.AP_CTL"]
pub type AP_CTL_SYS_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP_CTL_SYS_DISABLE` writer - goes to CPUSS.AP_CTL"]
pub type AP_CTL_SYS_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 2, O>;
#[doc = "Field `SYS_AP_MPU_ENABLE` reader - goes to SYS DAP MPU"]
pub type SYS_AP_MPU_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SYS_AP_MPU_ENABLE` writer - goes to SYS DAP MPU"]
pub type SYS_AP_MPU_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, bool, O>;
#[doc = "Field `DIRECT_EXECUTE_DISABLE` reader - used by ROM boot"]
pub type DIRECT_EXECUTE_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `DIRECT_EXECUTE_DISABLE` writer - used by ROM boot"]
pub type DIRECT_EXECUTE_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, bool, O>;
#[doc = "Field `FLASH_ALLOWED` reader - goes to SYS DAP MPU"]
pub type FLASH_ALLOWED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_ALLOWED` writer - goes to SYS DAP MPU"]
pub type FLASH_ALLOWED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SRAM_ALLOWED` reader - goes to SYS DAP MPU"]
pub type SRAM_ALLOWED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRAM_ALLOWED` writer - goes to SYS DAP MPU"]
pub type SRAM_ALLOWED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 3, O>;
#[doc = "Field `WORK_FLASH_ALLOWED` reader - goes to SYS DAP MPU"]
pub type WORK_FLASH_ALLOWED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WORK_FLASH_ALLOWED` writer - goes to SYS DAP MPU"]
pub type WORK_FLASH_ALLOWED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 2, O>;
#[doc = "Field `SFLASH_ALLOWED` reader - goes to SYS DAP MPU"]
pub type SFLASH_ALLOWED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFLASH_ALLOWED` writer - goes to SYS DAP MPU"]
pub type SFLASH_ALLOWED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 2, O>;
#[doc = "Field `MMIO_ALLOWED` reader - goes to SYS DAP MPU"]
pub type MMIO_ALLOWED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMIO_ALLOWED` writer - goes to SYS DAP MPU"]
pub type MMIO_ALLOWED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u8, u8, 2, O>;
#[doc = "Field `SMIF_XIP_ENABLE` reader - goes to SYS DAP MPU"]
pub type SMIF_XIP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SMIF_XIP_ENABLE` writer - goes to SYS DAP MPU"]
pub type SMIF_XIP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, bool, O>;
#[doc = "Field `RESEREVED` reader - N/A"]
pub type RESEREVED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESEREVED` writer - N/A"]
pub type RESEREVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_ACCESS_RESTRICT_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:1 - goes to CPUSS.AP_CTL"]
    #[inline(always)]
    pub fn ap_ctl_cm0_disable(&self) -> AP_CTL_CM0_DISABLE_R {
        AP_CTL_CM0_DISABLE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - goes to CPUSS.AP_CTL"]
    #[inline(always)]
    pub fn ap_ctl_cmx_disable(&self) -> AP_CTL_CMX_DISABLE_R {
        AP_CTL_CMX_DISABLE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - goes to CPUSS.AP_CTL"]
    #[inline(always)]
    pub fn ap_ctl_sys_disable(&self) -> AP_CTL_SYS_DISABLE_R {
        AP_CTL_SYS_DISABLE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - goes to SYS DAP MPU"]
    #[inline(always)]
    pub fn sys_ap_mpu_enable(&self) -> SYS_AP_MPU_ENABLE_R {
        SYS_AP_MPU_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - used by ROM boot"]
    #[inline(always)]
    pub fn direct_execute_disable(&self) -> DIRECT_EXECUTE_DISABLE_R {
        DIRECT_EXECUTE_DISABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - goes to SYS DAP MPU"]
    #[inline(always)]
    pub fn flash_allowed(&self) -> FLASH_ALLOWED_R {
        FLASH_ALLOWED_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - goes to SYS DAP MPU"]
    #[inline(always)]
    pub fn sram_allowed(&self) -> SRAM_ALLOWED_R {
        SRAM_ALLOWED_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - goes to SYS DAP MPU"]
    #[inline(always)]
    pub fn work_flash_allowed(&self) -> WORK_FLASH_ALLOWED_R {
        WORK_FLASH_ALLOWED_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - goes to SYS DAP MPU"]
    #[inline(always)]
    pub fn sflash_allowed(&self) -> SFLASH_ALLOWED_R {
        SFLASH_ALLOWED_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - goes to SYS DAP MPU"]
    #[inline(always)]
    pub fn mmio_allowed(&self) -> MMIO_ALLOWED_R {
        MMIO_ALLOWED_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - goes to SYS DAP MPU"]
    #[inline(always)]
    pub fn smif_xip_enable(&self) -> SMIF_XIP_ENABLE_R {
        SMIF_XIP_ENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - N/A"]
    #[inline(always)]
    pub fn resereved(&self) -> RESEREVED_R {
        RESEREVED_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - goes to CPUSS.AP_CTL"]
    #[inline(always)]
    #[must_use]
    pub fn ap_ctl_cm0_disable(&mut self) -> AP_CTL_CM0_DISABLE_W<0> {
        AP_CTL_CM0_DISABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - goes to CPUSS.AP_CTL"]
    #[inline(always)]
    #[must_use]
    pub fn ap_ctl_cmx_disable(&mut self) -> AP_CTL_CMX_DISABLE_W<2> {
        AP_CTL_CMX_DISABLE_W::new(self)
    }
    #[doc = "Bits 4:5 - goes to CPUSS.AP_CTL"]
    #[inline(always)]
    #[must_use]
    pub fn ap_ctl_sys_disable(&mut self) -> AP_CTL_SYS_DISABLE_W<4> {
        AP_CTL_SYS_DISABLE_W::new(self)
    }
    #[doc = "Bit 6 - goes to SYS DAP MPU"]
    #[inline(always)]
    #[must_use]
    pub fn sys_ap_mpu_enable(&mut self) -> SYS_AP_MPU_ENABLE_W<6> {
        SYS_AP_MPU_ENABLE_W::new(self)
    }
    #[doc = "Bit 7 - used by ROM boot"]
    #[inline(always)]
    #[must_use]
    pub fn direct_execute_disable(&mut self) -> DIRECT_EXECUTE_DISABLE_W<7> {
        DIRECT_EXECUTE_DISABLE_W::new(self)
    }
    #[doc = "Bits 8:10 - goes to SYS DAP MPU"]
    #[inline(always)]
    #[must_use]
    pub fn flash_allowed(&mut self) -> FLASH_ALLOWED_W<8> {
        FLASH_ALLOWED_W::new(self)
    }
    #[doc = "Bits 11:13 - goes to SYS DAP MPU"]
    #[inline(always)]
    #[must_use]
    pub fn sram_allowed(&mut self) -> SRAM_ALLOWED_W<11> {
        SRAM_ALLOWED_W::new(self)
    }
    #[doc = "Bits 14:15 - goes to SYS DAP MPU"]
    #[inline(always)]
    #[must_use]
    pub fn work_flash_allowed(&mut self) -> WORK_FLASH_ALLOWED_W<14> {
        WORK_FLASH_ALLOWED_W::new(self)
    }
    #[doc = "Bits 16:17 - goes to SYS DAP MPU"]
    #[inline(always)]
    #[must_use]
    pub fn sflash_allowed(&mut self) -> SFLASH_ALLOWED_W<16> {
        SFLASH_ALLOWED_W::new(self)
    }
    #[doc = "Bits 18:19 - goes to SYS DAP MPU"]
    #[inline(always)]
    #[must_use]
    pub fn mmio_allowed(&mut self) -> MMIO_ALLOWED_W<18> {
        MMIO_ALLOWED_W::new(self)
    }
    #[doc = "Bit 20 - goes to SYS DAP MPU"]
    #[inline(always)]
    #[must_use]
    pub fn smif_xip_enable(&mut self) -> SMIF_XIP_ENABLE_W<20> {
        SMIF_XIP_ENABLE_W::new(self)
    }
    #[doc = "Bits 21:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn resereved(&mut self) -> RESEREVED_W<21> {
        RESEREVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access restrictions for SECURE protection state in SECURE lifecycle stage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_access_restrict](index.html) module"]
pub struct SECURE_ACCESS_RESTRICT_SPEC;
impl crate::RegisterSpec for SECURE_ACCESS_RESTRICT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_access_restrict::R](R) reader structure"]
impl crate::Readable for SECURE_ACCESS_RESTRICT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_access_restrict::W](W) writer structure"]
impl crate::Writable for SECURE_ACCESS_RESTRICT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECURE_ACCESS_RESTRICT to value 0"]
impl crate::Resettable for SECURE_ACCESS_RESTRICT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
