#[doc = "Register `BURSTBUFFERMANAGEMENT` reader"]
pub struct R(crate::R<BURSTBUFFERMANAGEMENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BURSTBUFFERMANAGEMENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BURSTBUFFERMANAGEMENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BURSTBUFFERMANAGEMENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BURSTBUFFERMANAGEMENT` writer"]
pub struct W(crate::W<BURSTBUFFERMANAGEMENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BURSTBUFFERMANAGEMENT_SPEC>;
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
impl From<crate::W<BURSTBUFFERMANAGEMENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BURSTBUFFERMANAGEMENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETNUMBUFFERS` reader - Set this to the number of bursts that should be buffered. SetNumBuffers has to be smaller or equal to ManagedBurstBuffers and SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers. Must be a power of 2. The minimum allowed settings is 4 except for the fetcheco derivate which has a minimum allowed setting of 2."]
pub type SETNUMBUFFERS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETNUMBUFFERS` writer - Set this to the number of bursts that should be buffered. SetNumBuffers has to be smaller or equal to ManagedBurstBuffers and SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers. Must be a power of 2. The minimum allowed settings is 4 except for the fetcheco derivate which has a minimum allowed setting of 2."]
pub type SETNUMBUFFERS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BURSTBUFFERMANAGEMENT_SPEC, u8, u8, 8, O>;
#[doc = "Field `SETBURSTLENGTH` reader - Set this to the burst length that should be used on the AXI interface. SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers and that bursts larger than 16 are not possible on the axi interface. Only a power of two may be specified as burst length."]
pub type SETBURSTLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETBURSTLENGTH` writer - Set this to the burst length that should be used on the AXI interface. SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers and that bursts larger than 16 are not possible on the axi interface. Only a power of two may be specified as burst length."]
pub type SETBURSTLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BURSTBUFFERMANAGEMENT_SPEC, u8, u8, 5, O>;
#[doc = "Field `LINEMODE` reader - Fetch buffer cache control."]
pub type LINEMODE_R = crate::BitReader<LINEMODE_A>;
#[doc = "Fetch buffer cache control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEMODE_A {
    #[doc = "0: Mandatory setting for operation in the Display Controller. Works also for Blit Engine with marginal performance impact."]
    DISPLAY = 0,
    #[doc = "1: Recommended setting for operation in the Blit Engine."]
    BLIT = 1,
}
impl From<LINEMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LINEMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEMODE_A {
        match self.bits {
            false => LINEMODE_A::DISPLAY,
            true => LINEMODE_A::BLIT,
        }
    }
    #[doc = "Checks if the value of the field is `DISPLAY`"]
    #[inline(always)]
    pub fn is_display(&self) -> bool {
        *self == LINEMODE_A::DISPLAY
    }
    #[doc = "Checks if the value of the field is `BLIT`"]
    #[inline(always)]
    pub fn is_blit(&self) -> bool {
        *self == LINEMODE_A::BLIT
    }
}
#[doc = "Field `LINEMODE` writer - Fetch buffer cache control."]
pub type LINEMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BURSTBUFFERMANAGEMENT_SPEC, LINEMODE_A, O>;
impl<'a, const O: u8> LINEMODE_W<'a, O> {
    #[doc = "Mandatory setting for operation in the Display Controller. Works also for Blit Engine with marginal performance impact."]
    #[inline(always)]
    pub fn display(self) -> &'a mut W {
        self.variant(LINEMODE_A::DISPLAY)
    }
    #[doc = "Recommended setting for operation in the Blit Engine."]
    #[inline(always)]
    pub fn blit(self) -> &'a mut W {
        self.variant(LINEMODE_A::BLIT)
    }
}
impl R {
    #[doc = "Bits 0:7 - Set this to the number of bursts that should be buffered. SetNumBuffers has to be smaller or equal to ManagedBurstBuffers and SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers. Must be a power of 2. The minimum allowed settings is 4 except for the fetcheco derivate which has a minimum allowed setting of 2."]
    #[inline(always)]
    pub fn setnumbuffers(&self) -> SETNUMBUFFERS_R {
        SETNUMBUFFERS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Set this to the burst length that should be used on the AXI interface. SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers and that bursts larger than 16 are not possible on the axi interface. Only a power of two may be specified as burst length."]
    #[inline(always)]
    pub fn setburstlength(&self) -> SETBURSTLENGTH_R {
        SETBURSTLENGTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Fetch buffer cache control."]
    #[inline(always)]
    pub fn linemode(&self) -> LINEMODE_R {
        LINEMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set this to the number of bursts that should be buffered. SetNumBuffers has to be smaller or equal to ManagedBurstBuffers and SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers. Must be a power of 2. The minimum allowed settings is 4 except for the fetcheco derivate which has a minimum allowed setting of 2."]
    #[inline(always)]
    #[must_use]
    pub fn setnumbuffers(&mut self) -> SETNUMBUFFERS_W<0> {
        SETNUMBUFFERS_W::new(self)
    }
    #[doc = "Bits 8:12 - Set this to the burst length that should be used on the AXI interface. SetNumBuffers * SetBurstLength has to be smaller or equal to ManagedBurstBuffers * BurstLengthForMaxBuffers and that bursts larger than 16 are not possible on the axi interface. Only a power of two may be specified as burst length."]
    #[inline(always)]
    #[must_use]
    pub fn setburstlength(&mut self) -> SETBURSTLENGTH_W<8> {
        SETBURSTLENGTH_W::new(self)
    }
    #[doc = "Bit 31 - Fetch buffer cache control."]
    #[inline(always)]
    #[must_use]
    pub fn linemode(&mut self) -> LINEMODE_W<31> {
        LINEMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interface buffer management register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstbuffermanagement](index.html) module"]
pub struct BURSTBUFFERMANAGEMENT_SPEC;
impl crate::RegisterSpec for BURSTBUFFERMANAGEMENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [burstbuffermanagement::R](R) reader structure"]
impl crate::Readable for BURSTBUFFERMANAGEMENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [burstbuffermanagement::W](W) writer structure"]
impl crate::Writable for BURSTBUFFERMANAGEMENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BURSTBUFFERMANAGEMENT to value 0x0204"]
impl crate::Resettable for BURSTBUFFERMANAGEMENT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0204;
}
