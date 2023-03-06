#[doc = "Register `INTR_SRC` reader"]
pub struct R(crate::R<INTR_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SRC` writer"]
pub struct W(crate::W<INTR_SRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SRC_SPEC>;
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
impl From<crate::W<INTR_SRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_TRIGGER` reader - HW sets this field to '1', when a source trigger is generated."]
pub type FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_TRIGGER` writer - HW sets this field to '1', when a source trigger is generated."]
pub type FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SRC_SPEC, bool, O>;
#[doc = "Field `FIFO_OVERFLOW` reader - HW sets this field to '1', when writing to a full source FIFO (SRC_FIFO_STATUS.USED is '128')."]
pub type FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_OVERFLOW` writer - HW sets this field to '1', when writing to a full source FIFO (SRC_FIFO_STATUS.USED is '128')."]
pub type FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SRC_SPEC, bool, O>;
#[doc = "Field `FADED_OUT` reader - HW sets this field to '1', when fade out has completed (HW changes SRC_FADE_CMD.FADE_OUT from '0' to '1')."]
pub type FADED_OUT_R = crate::BitReader<bool>;
#[doc = "Field `FADED_OUT` writer - HW sets this field to '1', when fade out has completed (HW changes SRC_FADE_CMD.FADE_OUT from '0' to '1')."]
pub type FADED_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SRC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HW sets this field to '1', when a source trigger is generated."]
    #[inline(always)]
    pub fn fifo_trigger(&self) -> FIFO_TRIGGER_R {
        FIFO_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HW sets this field to '1', when writing to a full source FIFO (SRC_FIFO_STATUS.USED is '128')."]
    #[inline(always)]
    pub fn fifo_overflow(&self) -> FIFO_OVERFLOW_R {
        FIFO_OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when fade out has completed (HW changes SRC_FADE_CMD.FADE_OUT from '0' to '1')."]
    #[inline(always)]
    pub fn faded_out(&self) -> FADED_OUT_R {
        FADED_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HW sets this field to '1', when a source trigger is generated."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_trigger(&mut self) -> FIFO_TRIGGER_W<0> {
        FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 1 - HW sets this field to '1', when writing to a full source FIFO (SRC_FIFO_STATUS.USED is '128')."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow(&mut self) -> FIFO_OVERFLOW_W<1> {
        FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when fade out has completed (HW changes SRC_FADE_CMD.FADE_OUT from '0' to '1')."]
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
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_src](index.html) module"]
pub struct INTR_SRC_SPEC;
impl crate::RegisterSpec for INTR_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_src::R](R) reader structure"]
impl crate::Readable for INTR_SRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_src::W](W) writer structure"]
impl crate::Writable for INTR_SRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SRC to value 0"]
impl crate::Resettable for INTR_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
