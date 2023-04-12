#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE` reader - Activate functionality (1 bit for each channel): '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
pub type ACTIVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTIVE` writer - Activate functionality (1 bit for each channel): '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
pub type ACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Activate functionality (1 bit for each channel): '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Activate functionality (1 bit for each channel): '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<0> {
        ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
