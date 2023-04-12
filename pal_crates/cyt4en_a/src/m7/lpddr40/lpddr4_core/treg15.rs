#[doc = "Register `TREG15` reader"]
pub struct R(crate::R<TREG15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG15` writer"]
pub struct W(crate::W<TREG15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG15_SPEC>;
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
impl From<crate::W<TREG15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_VRCGEN` reader - VREF high current mode enable time = 9'ha3"]
pub type T_VRCGEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_VRCGEN` writer - VREF high current mode enable time = 9'ha3"]
pub type T_VRCGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG15_SPEC, u16, u16, 9, O>;
#[doc = "Field `T_VRCGDIS` reader - VREF high current mode disable time = 8'h53"]
pub type T_VRCGDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_VRCGDIS` writer - VREF high current mode disable time = 8'h53"]
pub type T_VRCGDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG15_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:8 - VREF high current mode enable time = 9'ha3"]
    #[inline(always)]
    pub fn t_vrcgen(&self) -> T_VRCGEN_R {
        T_VRCGEN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - VREF high current mode disable time = 8'h53"]
    #[inline(always)]
    pub fn t_vrcgdis(&self) -> T_VRCGDIS_R {
        T_VRCGDIS_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - VREF high current mode enable time = 9'ha3"]
    #[inline(always)]
    #[must_use]
    pub fn t_vrcgen(&mut self) -> T_VRCGEN_W<0> {
        T_VRCGEN_W::new(self)
    }
    #[doc = "Bits 9:16 - VREF high current mode disable time = 8'h53"]
    #[inline(always)]
    #[must_use]
    pub fn t_vrcgdis(&mut self) -> T_VRCGDIS_W<9> {
        T_VRCGDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg15](index.html) module"]
pub struct TREG15_SPEC;
impl crate::RegisterSpec for TREG15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg15::R](R) reader structure"]
impl crate::Readable for TREG15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg15::W](W) writer structure"]
impl crate::Writable for TREG15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG15 to value 0xa0a0"]
impl crate::Resettable for TREG15_SPEC {
    const RESET_VALUE: Self::Ux = 0xa0a0;
}
