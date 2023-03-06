#[doc = "Register `TST_XRES_SECURE` reader"]
pub struct R(crate::R<TST_XRES_SECURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_XRES_SECURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_XRES_SECURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_XRES_SECURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_XRES_SECURE` writer"]
pub struct W(crate::W<TST_XRES_SECURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_XRES_SECURE_SPEC>;
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
impl From<crate::W<TST_XRES_SECURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_XRES_SECURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA8` reader - Data byte to be set into either SECURE TEST or FIRMWARE TEST key. Must not be changed in the same write that is toggling any of the *_WR bits below,"]
pub type DATA8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA8` writer - Data byte to be set into either SECURE TEST or FIRMWARE TEST key. Must not be changed in the same write that is toggling any of the *_WR bits below,"]
pub type DATA8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TST_XRES_SECURE_SPEC, u8, u8, 8, O>;
#[doc = "Field `FW_WR` reader - Latch enables for each of the 4 bytes in the 32-bit FIRMWARE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
pub type FW_WR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FW_WR` writer - Latch enables for each of the 4 bytes in the 32-bit FIRMWARE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
pub type FW_WR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TST_XRES_SECURE_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECURE_WR` reader - Latch enables for each of the 4 bytes in the 32-bit SECURE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
pub type SECURE_WR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURE_WR` writer - Latch enables for each of the 4 bytes in the 32-bit SECURE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
pub type SECURE_WR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TST_XRES_SECURE_SPEC, u8, u8, 4, O>;
#[doc = "Field `FW_KEY_OK` reader - Indicates that the 32-bit FIRMWARE TEST key is observing the correct key. Firmware key is reset by (A)XRES and STRUCT_XRES."]
pub type FW_KEY_OK_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_KEY_OK` reader - Indicates that the 32-bit SECURE TEST key is observing the correct key. Secure key is not reset, but it will establish low after a deep power cycle that causes it to lose its written state."]
pub type SECURE_KEY_OK_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_DISABLE` reader - Disables the SECURE TEST key entry capability until next reset. Must not be set in the same write when any of the above *_WR bits are set or toggling."]
pub type SECURE_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_DISABLE` writer - Disables the SECURE TEST key entry capability until next reset. Must not be set in the same write when any of the above *_WR bits are set or toggling."]
pub type SECURE_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TST_XRES_SECURE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte to be set into either SECURE TEST or FIRMWARE TEST key. Must not be changed in the same write that is toggling any of the *_WR bits below,"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Latch enables for each of the 4 bytes in the 32-bit FIRMWARE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
    #[inline(always)]
    pub fn fw_wr(&self) -> FW_WR_R {
        FW_WR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Latch enables for each of the 4 bytes in the 32-bit SECURE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
    #[inline(always)]
    pub fn secure_wr(&self) -> SECURE_WR_R {
        SECURE_WR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Indicates that the 32-bit FIRMWARE TEST key is observing the correct key. Firmware key is reset by (A)XRES and STRUCT_XRES."]
    #[inline(always)]
    pub fn fw_key_ok(&self) -> FW_KEY_OK_R {
        FW_KEY_OK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicates that the 32-bit SECURE TEST key is observing the correct key. Secure key is not reset, but it will establish low after a deep power cycle that causes it to lose its written state."]
    #[inline(always)]
    pub fn secure_key_ok(&self) -> SECURE_KEY_OK_R {
        SECURE_KEY_OK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disables the SECURE TEST key entry capability until next reset. Must not be set in the same write when any of the above *_WR bits are set or toggling."]
    #[inline(always)]
    pub fn secure_disable(&self) -> SECURE_DISABLE_R {
        SECURE_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte to be set into either SECURE TEST or FIRMWARE TEST key. Must not be changed in the same write that is toggling any of the *_WR bits below,"]
    #[inline(always)]
    #[must_use]
    pub fn data8(&mut self) -> DATA8_W<0> {
        DATA8_W::new(self)
    }
    #[doc = "Bits 8:11 - Latch enables for each of the 4 bytes in the 32-bit FIRMWARE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
    #[inline(always)]
    #[must_use]
    pub fn fw_wr(&mut self) -> FW_WR_W<8> {
        FW_WR_W::new(self)
    }
    #[doc = "Bits 16:19 - Latch enables for each of the 4 bytes in the 32-bit SECURE TEST key. Must be toggled high and then low while keeping DATA8 to the correct value."]
    #[inline(always)]
    #[must_use]
    pub fn secure_wr(&mut self) -> SECURE_WR_W<16> {
        SECURE_WR_W::new(self)
    }
    #[doc = "Bit 31 - Disables the SECURE TEST key entry capability until next reset. Must not be set in the same write when any of the above *_WR bits are set or toggling."]
    #[inline(always)]
    #[must_use]
    pub fn secure_disable(&mut self) -> SECURE_DISABLE_W<31> {
        SECURE_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SECURE TEST and FIRMWARE TEST Key control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_xres_secure](index.html) module"]
pub struct TST_XRES_SECURE_SPEC;
impl crate::RegisterSpec for TST_XRES_SECURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst_xres_secure::R](R) reader structure"]
impl crate::Readable for TST_XRES_SECURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_xres_secure::W](W) writer structure"]
impl crate::Writable for TST_XRES_SECURE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST_XRES_SECURE to value 0"]
impl crate::Resettable for TST_XRES_SECURE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
