#[doc = "Register `TMR_CTL` reader"]
pub struct R(crate::R<TMR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR_CTL` writer"]
pub struct W(crate::W<TMR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_CTL_SPEC>;
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
impl From<crate::W<TMR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR` reader - 28-bit timer value. The duration of the measurement window is (2**PSC)*TMR AXI clock cyles of the AXI port selected by PORT_SELECT.SELECT. When TMR is 0, the timer is off, and the measurement window is started by TMR_CMD.START and stopped by TMR_CMD.STOP (in this case, the duration of the measurement window needs to be measured by software)."]
pub type TMR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMR` writer - 28-bit timer value. The duration of the measurement window is (2**PSC)*TMR AXI clock cyles of the AXI port selected by PORT_SELECT.SELECT. When TMR is 0, the timer is off, and the measurement window is started by TMR_CMD.START and stopped by TMR_CMD.STOP (in this case, the duration of the measurement window needs to be measured by software)."]
pub type TMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMR_CTL_SPEC, u32, u32, 28, O>;
#[doc = "Field `PSC` reader - 4-bit prescaler value. The clock going to the timer is divided by 2**PSC."]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - 4-bit prescaler value. The clock going to the timer is divided by 2**PSC."]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMR_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:27 - 28-bit timer value. The duration of the measurement window is (2**PSC)*TMR AXI clock cyles of the AXI port selected by PORT_SELECT.SELECT. When TMR is 0, the timer is off, and the measurement window is started by TMR_CMD.START and stopped by TMR_CMD.STOP (in this case, the duration of the measurement window needs to be measured by software)."]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - 4-bit prescaler value. The clock going to the timer is divided by 2**PSC."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - 28-bit timer value. The duration of the measurement window is (2**PSC)*TMR AXI clock cyles of the AXI port selected by PORT_SELECT.SELECT. When TMR is 0, the timer is off, and the measurement window is started by TMR_CMD.START and stopped by TMR_CMD.STOP (in this case, the duration of the measurement window needs to be measured by software)."]
    #[inline(always)]
    #[must_use]
    pub fn tmr(&mut self) -> TMR_W<0> {
        TMR_W::new(self)
    }
    #[doc = "Bits 28:31 - 4-bit prescaler value. The clock going to the timer is divided by 2**PSC."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<28> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr_ctl](index.html) module"]
pub struct TMR_CTL_SPEC;
impl crate::RegisterSpec for TMR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr_ctl::R](R) reader structure"]
impl crate::Readable for TMR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr_ctl::W](W) writer structure"]
impl crate::Writable for TMR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR_CTL to value 0"]
impl crate::Resettable for TMR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
