#[doc = "Register `FCL` reader"]
pub struct R(crate::R<FCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCL` writer"]
pub struct W(crate::W<FCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCL_SPEC>;
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
impl From<crate::W<FCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CL` reader - Critical Level When the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level configured by CL\\[7:0\\], the receive FIFO critical level flag FSR.RFCL is set. If CL\\[7:0\\]
is programmed to values > 128, bit FSR.RFCL is never set. When FSR.RFCL changes from '0' to 1' bit SIR.RFCL is set to '1', and if enabled, an interrupt is generated."]
pub type CL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL` writer - Critical Level When the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level configured by CL\\[7:0\\], the receive FIFO critical level flag FSR.RFCL is set. If CL\\[7:0\\]
is programmed to values > 128, bit FSR.RFCL is never set. When FSR.RFCL changes from '0' to 1' bit SIR.RFCL is set to '1', and if enabled, an interrupt is generated."]
pub type CL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Critical Level When the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level configured by CL\\[7:0\\], the receive FIFO critical level flag FSR.RFCL is set. If CL\\[7:0\\]
is programmed to values > 128, bit FSR.RFCL is never set. When FSR.RFCL changes from '0' to 1' bit SIR.RFCL is set to '1', and if enabled, an interrupt is generated."]
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Critical Level When the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level configured by CL\\[7:0\\], the receive FIFO critical level flag FSR.RFCL is set. If CL\\[7:0\\]
is programmed to values > 128, bit FSR.RFCL is never set. When FSR.RFCL changes from '0' to 1' bit SIR.RFCL is set to '1', and if enabled, an interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cl(&mut self) -> CL_W<0> {
        CL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Critical Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcl](index.html) module"]
pub struct FCL_SPEC;
impl crate::RegisterSpec for FCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcl::R](R) reader structure"]
impl crate::Readable for FCL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcl::W](W) writer structure"]
impl crate::Writable for FCL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCL to value 0x80"]
impl crate::Resettable for FCL_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
