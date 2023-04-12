#[doc = "Register `FM_ADDR` writer"]
pub struct W(crate::W<FM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_ADDR_SPEC>;
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
impl From<crate::W<FM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FM_ADDR` writer - Code or Work Flash Address to be used during write operations (PGM/ERS)"]
pub type FM_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FM_ADDR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Code or Work Flash Address to be used during write operations (PGM/ERS)"]
    #[inline(always)]
    #[must_use]
    pub fn fm_addr(&mut self) -> FM_ADDR_W<0> {
        FM_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Macro Address\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_addr](index.html) module"]
pub struct FM_ADDR_SPEC;
impl crate::RegisterSpec for FM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fm_addr::W](W) writer structure"]
impl crate::Writable for FM_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_ADDR to value 0"]
impl crate::Resettable for FM_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
