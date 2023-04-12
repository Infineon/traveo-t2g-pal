#[doc = "Register `INECCSTT4` reader"]
pub struct R(crate::R<INECCSTT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECCSTT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECCSTT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECCSTT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INECCSTT4` writer"]
pub struct W(crate::W<INECCSTT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INECCSTT4_SPEC>;
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
impl From<crate::W<INECCSTT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INECCSTT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_NUM` reader - Number of SEC Write to this register to clear the value"]
pub type SEC_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEC_NUM` writer - Number of SEC Write to this register to clear the value"]
pub type SEC_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INECCSTT4_SPEC, u16, u16, 16, O>;
#[doc = "Field `DED_NUM` reader - Number of DED Write to this register to clear the value"]
pub type DED_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DED_NUM` writer - Number of DED Write to this register to clear the value"]
pub type DED_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INECCSTT4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Number of SEC Write to this register to clear the value"]
    #[inline(always)]
    pub fn sec_num(&self) -> SEC_NUM_R {
        SEC_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of DED Write to this register to clear the value"]
    #[inline(always)]
    pub fn ded_num(&self) -> DED_NUM_R {
        DED_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of SEC Write to this register to clear the value"]
    #[inline(always)]
    #[must_use]
    pub fn sec_num(&mut self) -> SEC_NUM_W<0> {
        SEC_NUM_W::new(self)
    }
    #[doc = "Bits 16:31 - Number of DED Write to this register to clear the value"]
    #[inline(always)]
    #[must_use]
    pub fn ded_num(&mut self) -> DED_NUM_W<16> {
        DED_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inline ECC Status Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ineccstt4](index.html) module"]
pub struct INECCSTT4_SPEC;
impl crate::RegisterSpec for INECCSTT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ineccstt4::R](R) reader structure"]
impl crate::Readable for INECCSTT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ineccstt4::W](W) writer structure"]
impl crate::Writable for INECCSTT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INECCSTT4 to value 0"]
impl crate::Resettable for INECCSTT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
