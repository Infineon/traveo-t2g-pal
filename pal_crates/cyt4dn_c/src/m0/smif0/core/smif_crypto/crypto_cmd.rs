#[doc = "Register `CRYPTO_CMD` reader"]
pub struct R(crate::R<CRYPTO_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_CMD` writer"]
pub struct W(crate::W<CRYPTO_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_CMD_SPEC>;
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
impl From<crate::W<CRYPTO_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - SW sets this field to '1' to start an AES-128 forward block cipher operation (on CRYPTO_INPUT0-3). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_OUTPUT0, ..., CRYPTO_OUTPUT3. The operation takes roughly 13 clk_mem clock cycles. This register is only applicable for MMIO accesses and only in MMIO_MODE. XIP accesses are encrypted on-the-fly in ARB_MODE and thus do not use this register. Because XIP has exclusive access to the crypto engine in ARB_MODE, MMIO encryption is not available in that mode. As such if encryption is needed for MMIO accesses in ARB_MODE that encryption must be done by other means (i.e., chip level crypto)."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - SW sets this field to '1' to start an AES-128 forward block cipher operation (on CRYPTO_INPUT0-3). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_OUTPUT0, ..., CRYPTO_OUTPUT3. The operation takes roughly 13 clk_mem clock cycles. This register is only applicable for MMIO accesses and only in MMIO_MODE. XIP accesses are encrypted on-the-fly in ARB_MODE and thus do not use this register. Because XIP has exclusive access to the crypto engine in ARB_MODE, MMIO encryption is not available in that mode. As such if encryption is needed for MMIO accesses in ARB_MODE that encryption must be done by other means (i.e., chip level crypto)."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYPTO_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to start an AES-128 forward block cipher operation (on CRYPTO_INPUT0-3). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_OUTPUT0, ..., CRYPTO_OUTPUT3. The operation takes roughly 13 clk_mem clock cycles. This register is only applicable for MMIO accesses and only in MMIO_MODE. XIP accesses are encrypted on-the-fly in ARB_MODE and thus do not use this register. Because XIP has exclusive access to the crypto engine in ARB_MODE, MMIO encryption is not available in that mode. As such if encryption is needed for MMIO accesses in ARB_MODE that encryption must be done by other means (i.e., chip level crypto)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to start an AES-128 forward block cipher operation (on CRYPTO_INPUT0-3). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_OUTPUT0, ..., CRYPTO_OUTPUT3. The operation takes roughly 13 clk_mem clock cycles. This register is only applicable for MMIO accesses and only in MMIO_MODE. XIP accesses are encrypted on-the-fly in ARB_MODE and thus do not use this register. Because XIP has exclusive access to the crypto engine in ARB_MODE, MMIO encryption is not available in that mode. As such if encryption is needed for MMIO accesses in ARB_MODE that encryption must be done by other means (i.e., chip level crypto)."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_cmd](index.html) module"]
pub struct CRYPTO_CMD_SPEC;
impl crate::RegisterSpec for CRYPTO_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_cmd::R](R) reader structure"]
impl crate::Readable for CRYPTO_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_cmd::W](W) writer structure"]
impl crate::Writable for CRYPTO_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_CMD to value 0"]
impl crate::Resettable for CRYPTO_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
