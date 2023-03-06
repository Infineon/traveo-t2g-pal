#[doc = "Register `OUTD0` reader"]
pub struct R(crate::R<OUTD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTD0` writer"]
pub struct W(crate::W<OUTD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTD0_SPEC>;
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
impl From<crate::W<OUTD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CA` reader - Output Data Input for CA bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type CA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CA` writer - Output Data Input for CA bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTD0_SPEC, u16, u16, 12, O>;
#[doc = "Field `CKE` reader - Output Data Input for CKE bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type CKE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKE` writer - Output Data Input for CKE bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type CKE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTD0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CS` reader - Output Data Input for CS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS` writer - Output Data Input for CS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTD0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ODT` reader - Output Data Input for ODT bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type ODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODT` writer - Output Data Input for ODT bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type ODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTD0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESET_N` reader - Output Data Input for RESET_N bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type RESET_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESET_N` writer - Output Data Input for RESET_N bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type RESET_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTD0_SPEC, u8, u8, 2, O>;
#[doc = "Field `DM` reader - Output Data Input for DM bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DM` writer - Output Data Input for DM bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type DM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTD0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DQS` reader - Output Data Input for DQS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type DQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQS` writer - Output Data Input for DQS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type DQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTD0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:11 - Output Data Input for CA bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn ca(&self) -> CA_R {
        CA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Output Data Input for CKE bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Output Data Input for CS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Output Data Input for ODT bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn odt(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Output Data Input for RESET_N bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - Output Data Input for DM bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Output Data Input for DQS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn dqs(&self) -> DQS_R {
        DQS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Output Data Input for CA bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CA_W<0> {
        CA_W::new(self)
    }
    #[doc = "Bits 12:13 - Output Data Input for CKE bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<12> {
        CKE_W::new(self)
    }
    #[doc = "Bits 14:15 - Output Data Input for CS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<14> {
        CS_W::new(self)
    }
    #[doc = "Bits 16:17 - Output Data Input for ODT bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn odt(&mut self) -> ODT_W<16> {
        ODT_W::new(self)
    }
    #[doc = "Bits 18:19 - Output Data Input for RESET_N bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> RESET_N_W<18> {
        RESET_N_W::new(self)
    }
    #[doc = "Bits 20:23 - Output Data Input for DM bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<20> {
        DM_W::new(self)
    }
    #[doc = "Bits 24:27 - Output Data Input for DQS bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn dqs(&mut self) -> DQS_W<24> {
        DQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Data In Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outd0](index.html) module"]
pub struct OUTD0_SPEC;
impl crate::RegisterSpec for OUTD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outd0::R](R) reader structure"]
impl crate::Readable for OUTD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outd0::W](W) writer structure"]
impl crate::Writable for OUTD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTD0 to value 0"]
impl crate::Resettable for OUTD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
