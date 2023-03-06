#[doc = "Register `INTR_DST` reader"]
pub struct R(crate::R<INTR_DST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_DST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_DST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_DST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_DST` writer"]
pub struct W(crate::W<INTR_DST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_DST_SPEC>;
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
impl From<crate::W<INTR_DST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_DST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_TRIGGER` reader - HW sets this field to '1', when a destination trigger is generated."]
pub type FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_TRIGGER` writer - HW sets this field to '1', when a destination trigger is generated."]
pub type FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DST_SPEC, bool, O>;
#[doc = "Field `FIFO_UNDERFLOW` reader - HW sets this field to '1', when reading from an empty destination FIFO (DST_FIFO_STATUS.USED is '0')."]
pub type FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_UNDERFLOW` writer - HW sets this field to '1', when reading from an empty destination FIFO (DST_FIFO_STATUS.USED is '0')."]
pub type FIFO_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DST_SPEC, bool, O>;
#[doc = "Field `FADED_OUT` reader - HW sets this field to '1', when fading out has completed (HW changes DST_FADE_CMD.FADE_OUT from '0' to '1')."]
pub type FADED_OUT_R = crate::BitReader<bool>;
#[doc = "Field `FADED_OUT` writer - HW sets this field to '1', when fading out has completed (HW changes DST_FADE_CMD.FADE_OUT from '0' to '1')."]
pub type FADED_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HW sets this field to '1', when a destination trigger is generated."]
    #[inline(always)]
    pub fn fifo_trigger(&self) -> FIFO_TRIGGER_R {
        FIFO_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - HW sets this field to '1', when reading from an empty destination FIFO (DST_FIFO_STATUS.USED is '0')."]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when fading out has completed (HW changes DST_FADE_CMD.FADE_OUT from '0' to '1')."]
    #[inline(always)]
    pub fn faded_out(&self) -> FADED_OUT_R {
        FADED_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HW sets this field to '1', when a destination trigger is generated."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_trigger(&mut self) -> FIFO_TRIGGER_W<0> {
        FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 2 - HW sets this field to '1', when reading from an empty destination FIFO (DST_FIFO_STATUS.USED is '0')."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underflow(&mut self) -> FIFO_UNDERFLOW_W<2> {
        FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when fading out has completed (HW changes DST_FADE_CMD.FADE_OUT from '0' to '1')."]
    #[inline(always)]
    #[must_use]
    pub fn faded_out(&mut self) -> FADED_OUT_W<8> {
        FADED_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_dst](index.html) module"]
pub struct INTR_DST_SPEC;
impl crate::RegisterSpec for INTR_DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_dst::R](R) reader structure"]
impl crate::Readable for INTR_DST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_dst::W](W) writer structure"]
impl crate::Writable for INTR_DST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_DST to value 0"]
impl crate::Resettable for INTR_DST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
