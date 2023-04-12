#[doc = "Register `CAL_UPD_CMD` reader"]
pub struct R(crate::R<CAL_UPD_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_UPD_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_UPD_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_UPD_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_UPD_CMD` writer"]
pub struct W(crate::W<CAL_UPD_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_UPD_CMD_SPEC>;
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
impl From<crate::W<CAL_UPD_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_UPD_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATE` reader - Calibration update command: coherently copy values from alternate calibration regs to current calibration regs. Software sets this bit when the alternate calibration values have been set with the new values. Hardware will do the calibration update as soon as the ADC is idle or a 'continuous' triggered group completes. This ensures that all acquisitions within a group scan (even if preempted) are done with the same calibration values. This bit is cleared at the same time the calibration update is done. By clearing this bit software can cancel a requested update. Note: if the ADC is always busy with acquisitions for non continuously triggered groups/channels then the calibration update will remain pending forever. In such a case the software can either do a non coherent update by writing directly to the current calibration registers, or software can force the ADC to idle by disabling some or all channels. Software can check/poll this bit to see if the calibration update has taken effect."]
pub type UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE` writer - Calibration update command: coherently copy values from alternate calibration regs to current calibration regs. Software sets this bit when the alternate calibration values have been set with the new values. Hardware will do the calibration update as soon as the ADC is idle or a 'continuous' triggered group completes. This ensures that all acquisitions within a group scan (even if preempted) are done with the same calibration values. This bit is cleared at the same time the calibration update is done. By clearing this bit software can cancel a requested update. Note: if the ADC is always busy with acquisitions for non continuously triggered groups/channels then the calibration update will remain pending forever. In such a case the software can either do a non coherent update by writing directly to the current calibration registers, or software can force the ADC to idle by disabling some or all channels. Software can check/poll this bit to see if the calibration update has taken effect."]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_UPD_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Calibration update command: coherently copy values from alternate calibration regs to current calibration regs. Software sets this bit when the alternate calibration values have been set with the new values. Hardware will do the calibration update as soon as the ADC is idle or a 'continuous' triggered group completes. This ensures that all acquisitions within a group scan (even if preempted) are done with the same calibration values. This bit is cleared at the same time the calibration update is done. By clearing this bit software can cancel a requested update. Note: if the ADC is always busy with acquisitions for non continuously triggered groups/channels then the calibration update will remain pending forever. In such a case the software can either do a non coherent update by writing directly to the current calibration registers, or software can force the ADC to idle by disabling some or all channels. Software can check/poll this bit to see if the calibration update has taken effect."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration update command: coherently copy values from alternate calibration regs to current calibration regs. Software sets this bit when the alternate calibration values have been set with the new values. Hardware will do the calibration update as soon as the ADC is idle or a 'continuous' triggered group completes. This ensures that all acquisitions within a group scan (even if preempted) are done with the same calibration values. This bit is cleared at the same time the calibration update is done. By clearing this bit software can cancel a requested update. Note: if the ADC is always busy with acquisitions for non continuously triggered groups/channels then the calibration update will remain pending forever. In such a case the software can either do a non coherent update by writing directly to the current calibration registers, or software can force the ADC to idle by disabling some or all channels. Software can check/poll this bit to see if the calibration update has taken effect."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<0> {
        UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration update command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_upd_cmd](index.html) module"]
pub struct CAL_UPD_CMD_SPEC;
impl crate::RegisterSpec for CAL_UPD_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_upd_cmd::R](R) reader structure"]
impl crate::Readable for CAL_UPD_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_upd_cmd::W](W) writer structure"]
impl crate::Writable for CAL_UPD_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_UPD_CMD to value 0"]
impl crate::Resettable for CAL_UPD_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
