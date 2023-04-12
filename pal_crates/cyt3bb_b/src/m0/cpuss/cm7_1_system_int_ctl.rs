#[doc = "Register `CM7_1_SYSTEM_INT_CTL[%s]` reader"]
pub struct R(crate::R<CM7_1_SYSTEM_INT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_1_SYSTEM_INT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_1_SYSTEM_INT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_1_SYSTEM_INT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_1_SYSTEM_INT_CTL[%s]` writer"]
pub struct W(crate::W<CM7_1_SYSTEM_INT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_1_SYSTEM_INT_CTL_SPEC>;
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
impl From<crate::W<CM7_1_SYSTEM_INT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_1_SYSTEM_INT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_INT_IDX` reader - Refer CM7_0_SYSTEM_INT_CTL description."]
pub type CPU_INT_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_INT_IDX` writer - Refer CM7_0_SYSTEM_INT_CTL description."]
pub type CPU_INT_IDX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM7_1_SYSTEM_INT_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPU_INT_VALID` reader - Refer CM7_0_SYSTEM_INT_CTL description."]
pub type CPU_INT_VALID_R = crate::BitReader<bool>;
#[doc = "Field `CPU_INT_VALID` writer - Refer CM7_0_SYSTEM_INT_CTL description."]
pub type CPU_INT_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CM7_1_SYSTEM_INT_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Refer CM7_0_SYSTEM_INT_CTL description."]
    #[inline(always)]
    pub fn cpu_int_idx(&self) -> CPU_INT_IDX_R {
        CPU_INT_IDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Refer CM7_0_SYSTEM_INT_CTL description."]
    #[inline(always)]
    pub fn cpu_int_valid(&self) -> CPU_INT_VALID_R {
        CPU_INT_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Refer CM7_0_SYSTEM_INT_CTL description."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_idx(&mut self) -> CPU_INT_IDX_W<0> {
        CPU_INT_IDX_W::new(self)
    }
    #[doc = "Bit 31 - Refer CM7_0_SYSTEM_INT_CTL description."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_valid(&mut self) -> CPU_INT_VALID_W<31> {
        CPU_INT_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM7 1 system interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_1_system_int_ctl](index.html) module"]
pub struct CM7_1_SYSTEM_INT_CTL_SPEC;
impl crate::RegisterSpec for CM7_1_SYSTEM_INT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_1_system_int_ctl::R](R) reader structure"]
impl crate::Readable for CM7_1_SYSTEM_INT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_1_system_int_ctl::W](W) writer structure"]
impl crate::Writable for CM7_1_SYSTEM_INT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_1_SYSTEM_INT_CTL[%s]
to value 0"]
impl crate::Resettable for CM7_1_SYSTEM_INT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
