#[doc = "Register `TR_MON_CMD` reader"]
pub struct R(crate::R<TR_MON_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_MON_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_MON_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_MON_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_MON_CMD` writer"]
pub struct W(crate::W<TR_MON_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_MON_CMD_SPEC>;
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
impl From<crate::W<TR_MON_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_MON_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_AP` reader - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
pub type START_AP_R = crate::BitReader<bool>;
#[doc = "Field `START_AP` writer - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
pub type START_AP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_MON_CMD_SPEC, bool, O>;
#[doc = "Field `START_RC` reader - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
pub type START_RC_R = crate::BitReader<bool>;
#[doc = "Field `START_RC` writer - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
pub type START_RC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_MON_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
    #[inline(always)]
    pub fn start_ap(&self) -> START_AP_R {
        START_AP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
    #[inline(always)]
    pub fn start_rc(&self) -> START_RC_R {
        START_RC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
    #[inline(always)]
    #[must_use]
    pub fn start_ap(&mut self) -> START_AP_W<0> {
        START_AP_W::new(self)
    }
    #[doc = "Bit 1 - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
    #[inline(always)]
    #[must_use]
    pub fn start_rc(&mut self) -> START_RC_W<1> {
        START_RC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "True random monitor command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_mon_cmd](index.html) module"]
pub struct TR_MON_CMD_SPEC;
impl crate::RegisterSpec for TR_MON_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_mon_cmd::R](R) reader structure"]
impl crate::Readable for TR_MON_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_mon_cmd::W](W) writer structure"]
impl crate::Writable for TR_MON_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_MON_CMD to value 0"]
impl crate::Resettable for TR_MON_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
