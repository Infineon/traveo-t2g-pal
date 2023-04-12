#[doc = "Register `CRC_CMD` reader"]
pub struct R(crate::R<CRC_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_CMD` writer"]
pub struct W(crate::W<CRC_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CMD_SPEC>;
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
impl From<crate::W<CRC_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - SW sets this field to '1' to start a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using 0xFF as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. A CRC calculation over 64 bits is done by writing the 64 input bits to CRC_INPUT0/1, performing a START command and reading the CRC result from CRC_OUTPUT. Note: An operation can only be started in MMIO_MODE."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - SW sets this field to '1' to start a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using 0xFF as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. A CRC calculation over 64 bits is done by writing the 64 input bits to CRC_INPUT0/1, performing a START command and reading the CRC result from CRC_OUTPUT. Note: An operation can only be started in MMIO_MODE."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_CMD_SPEC, bool, O>;
#[doc = "Field `CONTINUE` reader - SW sets this field to '1' to continue a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using the current CRC output in CRC_OUTPUT as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. This command is used to perform a CRC calculation over more than 64 intput bits. For example a CRC calculation over 128 bits is done by writing the first 64 input bits to CRC_INPUT0/1, performing a START command, writing the second 64 input bits to CRC_INPUT0/1, performing a CONTINUE command and reading the CRC result from CRC_OUTPUT. For a CRC calculation of even more input bits the CONTINUE command has to be used multiple times. Note: An operation can only be started in MMIO_MODE."]
pub type CONTINUE_R = crate::BitReader<bool>;
#[doc = "Field `CONTINUE` writer - SW sets this field to '1' to continue a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using the current CRC output in CRC_OUTPUT as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. This command is used to perform a CRC calculation over more than 64 intput bits. For example a CRC calculation over 128 bits is done by writing the first 64 input bits to CRC_INPUT0/1, performing a START command, writing the second 64 input bits to CRC_INPUT0/1, performing a CONTINUE command and reading the CRC result from CRC_OUTPUT. For a CRC calculation of even more input bits the CONTINUE command has to be used multiple times. Note: An operation can only be started in MMIO_MODE."]
pub type CONTINUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to start a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using 0xFF as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. A CRC calculation over 64 bits is done by writing the 64 input bits to CRC_INPUT0/1, performing a START command and reading the CRC result from CRC_OUTPUT. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SW sets this field to '1' to continue a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using the current CRC output in CRC_OUTPUT as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. This command is used to perform a CRC calculation over more than 64 intput bits. For example a CRC calculation over 128 bits is done by writing the first 64 input bits to CRC_INPUT0/1, performing a START command, writing the second 64 input bits to CRC_INPUT0/1, performing a CONTINUE command and reading the CRC result from CRC_OUTPUT. For a CRC calculation of even more input bits the CONTINUE command has to be used multiple times. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn continue_(&self) -> CONTINUE_R {
        CONTINUE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to start a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using 0xFF as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. A CRC calculation over 64 bits is done by writing the 64 input bits to CRC_INPUT0/1, performing a START command and reading the CRC result from CRC_OUTPUT. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - SW sets this field to '1' to continue a CRC calculation over the 64 CRC input bits provided in CRC_INPUT0 and CRC_INPUT1, using the current CRC output in CRC_OUTPUT as CRC feedback. HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRC_OUTPUT. This command is used to perform a CRC calculation over more than 64 intput bits. For example a CRC calculation over 128 bits is done by writing the first 64 input bits to CRC_INPUT0/1, performing a START command, writing the second 64 input bits to CRC_INPUT0/1, performing a CONTINUE command and reading the CRC result from CRC_OUTPUT. For a CRC calculation of even more input bits the CONTINUE command has to be used multiple times. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    #[must_use]
    pub fn continue_(&mut self) -> CONTINUE_W<1> {
        CONTINUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_cmd](index.html) module"]
pub struct CRC_CMD_SPEC;
impl crate::RegisterSpec for CRC_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_cmd::R](R) reader structure"]
impl crate::Readable for CRC_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_cmd::W](W) writer structure"]
impl crate::Writable for CRC_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_CMD to value 0"]
impl crate::Resettable for CRC_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
