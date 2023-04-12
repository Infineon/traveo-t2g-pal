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
#[doc = "Field `CAPTURE0` reader - SW capture 0 trigger. When written with '1', a capture 0 trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.ENABLED, the field is immediately set to '0'."]
pub type CAPTURE0_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE0` writer - SW capture 0 trigger. When written with '1', a capture 0 trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.ENABLED, the field is immediately set to '0'."]
pub type CAPTURE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CMD_SPEC, bool, O>;
#[doc = "Field `RELOAD` reader - SW reload trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type RELOAD_R = crate::BitReader<bool>;
#[doc = "Field `RELOAD` writer - SW reload trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CMD_SPEC, bool, O>;
#[doc = "Field `STOP` reader - SW stop trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - SW stop trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CMD_SPEC, bool, O>;
#[doc = "Field `START` reader - SW start trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - SW start trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CMD_SPEC, bool, O>;
#[doc = "Field `CAPTURE1` reader - SW capture 1 trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type CAPTURE1_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE1` writer - SW capture 1 trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
pub type CAPTURE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW capture 0 trigger. When written with '1', a capture 0 trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn capture0(&self) -> CAPTURE0_R {
        CAPTURE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SW reload trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SW stop trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SW start trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SW capture 1 trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn capture1(&self) -> CAPTURE1_R {
        CAPTURE1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW capture 0 trigger. When written with '1', a capture 0 trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn capture0(&mut self) -> CAPTURE0_W<0> {
        CAPTURE0_W::new(self)
    }
    #[doc = "Bit 2 - SW reload trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<2> {
        RELOAD_W::new(self)
    }
    #[doc = "Bit 3 - SW stop trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<3> {
        STOP_W::new(self)
    }
    #[doc = "Bit 4 - SW start trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<4> {
        START_W::new(self)
    }
    #[doc = "Bit 5 - SW capture 1 trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    #[must_use]
    pub fn capture1(&mut self) -> CAPTURE1_W<5> {
        CAPTURE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter trigger command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](index.html) module"]
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
