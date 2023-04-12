#[doc = "Register `ITUSTS` reader"]
pub struct R(crate::R<ITUSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITUSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITUSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITUSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITUSTS` writer"]
pub struct W(crate::W<ITUSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITUSTS_SPEC>;
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
impl From<crate::W<ITUSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITUSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITUPROTECTIONONEBITERROR` reader - Set if a one bit error in the reference code of the ITU capture input 0 is found and corrected. Clear by writing a 1 to this field."]
pub type ITUPROTECTIONONEBITERROR_R = crate::BitReader<bool>;
#[doc = "Field `ITUPROTECTIONONEBITERROR` writer - Set if a one bit error in the reference code of the ITU capture input 0 is found and corrected. Clear by writing a 1 to this field."]
pub type ITUPROTECTIONONEBITERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ITUSTS_SPEC, bool, O>;
#[doc = "Field `ITUPROTECTIONTWOBITERROR` reader - Set if a two bit error in the reference code of the ITU capture input 0 is found. Two bit errors can only be detected, not corrected. Clear by writing a 1 to this field."]
pub type ITUPROTECTIONTWOBITERROR_R = crate::BitReader<bool>;
#[doc = "Field `ITUPROTECTIONTWOBITERROR` writer - Set if a two bit error in the reference code of the ITU capture input 0 is found. Two bit errors can only be detected, not corrected. Clear by writing a 1 to this field."]
pub type ITUPROTECTIONTWOBITERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ITUSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set if a one bit error in the reference code of the ITU capture input 0 is found and corrected. Clear by writing a 1 to this field."]
    #[inline(always)]
    pub fn ituprotectiononebiterror(&self) -> ITUPROTECTIONONEBITERROR_R {
        ITUPROTECTIONONEBITERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set if a two bit error in the reference code of the ITU capture input 0 is found. Two bit errors can only be detected, not corrected. Clear by writing a 1 to this field."]
    #[inline(always)]
    pub fn ituprotectiontwobiterror(&self) -> ITUPROTECTIONTWOBITERROR_R {
        ITUPROTECTIONTWOBITERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set if a one bit error in the reference code of the ITU capture input 0 is found and corrected. Clear by writing a 1 to this field."]
    #[inline(always)]
    #[must_use]
    pub fn ituprotectiononebiterror(&mut self) -> ITUPROTECTIONONEBITERROR_W<0> {
        ITUPROTECTIONONEBITERROR_W::new(self)
    }
    #[doc = "Bit 1 - Set if a two bit error in the reference code of the ITU capture input 0 is found. Two bit errors can only be detected, not corrected. Clear by writing a 1 to this field."]
    #[inline(always)]
    #[must_use]
    pub fn ituprotectiontwobiterror(&mut self) -> ITUPROTECTIONTWOBITERROR_W<1> {
        ITUPROTECTIONTWOBITERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ITU error status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itusts](index.html) module"]
pub struct ITUSTS_SPEC;
impl crate::RegisterSpec for ITUSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itusts::R](R) reader structure"]
impl crate::Readable for ITUSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itusts::W](W) writer structure"]
impl crate::Writable for ITUSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITUSTS to value 0"]
impl crate::Resettable for ITUSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
