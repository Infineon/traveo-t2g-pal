#[doc = "Register `TRIM_RAM350_CTL` reader"]
pub struct R(crate::R<TRIM_RAM350_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_RAM350_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_RAM350_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_RAM350_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_RAM350_CTL` writer"]
pub struct W(crate::W<TRIM_RAM350_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_RAM350_CTL_SPEC>;
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
impl From<crate::W<TRIM_RAM350_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_RAM350_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - See TRIM_RAM_CTL for description. CM7 Cache and TCM memories are connected to this register. All CM7 cache and TCM SRAM macros are controlled using bias control power switches, so bits \\[7:5\\]
of this register controls the bias pins of the CM7 cache and TCM SRAM power switches."]
pub type TRIM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TRIM` writer - See TRIM_RAM_CTL for description. CM7 Cache and TCM memories are connected to this register. All CM7 cache and TCM SRAM macros are controlled using bias control power switches, so bits \\[7:5\\]
of this register controls the bias pins of the CM7 cache and TCM SRAM power switches."]
pub type TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIM_RAM350_CTL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See TRIM_RAM_CTL for description. CM7 Cache and TCM memories are connected to this register. All CM7 cache and TCM SRAM macros are controlled using bias control power switches, so bits \\[7:5\\]
of this register controls the bias pins of the CM7 cache and TCM SRAM power switches."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See TRIM_RAM_CTL for description. CM7 Cache and TCM memories are connected to this register. All CM7 cache and TCM SRAM macros are controlled using bias control power switches, so bits \\[7:5\\]
of this register controls the bias pins of the CM7 cache and TCM SRAM power switches."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM trim control for more than 200MHz SRAMs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ram350_ctl](index.html) module"]
pub struct TRIM_RAM350_CTL_SPEC;
impl crate::RegisterSpec for TRIM_RAM350_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ram350_ctl::R](R) reader structure"]
impl crate::Readable for TRIM_RAM350_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ram350_ctl::W](W) writer structure"]
impl crate::Writable for TRIM_RAM350_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIM_RAM350_CTL to value 0"]
impl crate::Resettable for TRIM_RAM350_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
