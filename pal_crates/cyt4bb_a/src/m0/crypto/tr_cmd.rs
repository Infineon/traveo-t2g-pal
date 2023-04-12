#[doc = "Register `TR_CMD` reader"]
pub struct R(crate::R<TR_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CMD` writer"]
pub struct W(crate::W<TR_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CMD_SPEC>;
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
impl From<crate::W<TR_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
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
#[doc = "True random command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](index.html) module"]
pub struct TR_CMD_SPEC;
impl crate::RegisterSpec for TR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_cmd::R](R) reader structure"]
impl crate::Readable for TR_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_cmd::W](W) writer structure"]
impl crate::Writable for TR_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TR_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
