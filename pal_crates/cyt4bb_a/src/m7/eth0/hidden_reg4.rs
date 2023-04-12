#[doc = "Register `HIDDEN_REG4` reader"]
pub struct R(crate::R<HIDDEN_REG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIDDEN_REG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIDDEN_REG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIDDEN_REG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIDDEN_REG4` writer"]
pub struct W(crate::W<HIDDEN_REG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIDDEN_REG4_SPEC>;
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
impl From<crate::W<HIDDEN_REG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIDDEN_REG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIDDEN4_FIELD_L` reader - The valid bits is defined by `gem_tx_pbuf_addr."]
pub type HIDDEN4_FIELD_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIDDEN4_FIELD_L` writer - The valid bits is defined by `gem_tx_pbuf_addr."]
pub type HIDDEN4_FIELD_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HIDDEN_REG4_SPEC, u16, u16, 9, O>;
#[doc = "Field `HIDDEN4_FIELD_H` reader - The valid bits is defined by `gem_tx_pbuf_addr."]
pub type HIDDEN4_FIELD_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIDDEN4_FIELD_H` writer - The valid bits is defined by `gem_tx_pbuf_addr."]
pub type HIDDEN4_FIELD_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HIDDEN_REG4_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - The valid bits is defined by `gem_tx_pbuf_addr."]
    #[inline(always)]
    pub fn hidden4_field_l(&self) -> HIDDEN4_FIELD_L_R {
        HIDDEN4_FIELD_L_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - The valid bits is defined by `gem_tx_pbuf_addr."]
    #[inline(always)]
    pub fn hidden4_field_h(&self) -> HIDDEN4_FIELD_H_R {
        HIDDEN4_FIELD_H_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - The valid bits is defined by `gem_tx_pbuf_addr."]
    #[inline(always)]
    #[must_use]
    pub fn hidden4_field_l(&mut self) -> HIDDEN4_FIELD_L_W<0> {
        HIDDEN4_FIELD_L_W::new(self)
    }
    #[doc = "Bits 16:24 - The valid bits is defined by `gem_tx_pbuf_addr."]
    #[inline(always)]
    #[must_use]
    pub fn hidden4_field_h(&mut self) -> HIDDEN4_FIELD_H_W<16> {
        HIDDEN4_FIELD_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hidden registers defined in edma_defs.v '`define gem_axi_tx_full_threshold0 12'h4f8 // AXI full threshold setting *** HIDDEN Register ***'. Note. When using AXI mode with a single port ram ( gem_spram == 1) mode and a 32b dma bus width ( gem_dma_bus_width == 32 or bits 22 to 21 of the network_config register are set to 0) the AXI hidden registers (0x14F8 and 0x14FC) need to be updated (these registers are used for fine tuning AXI dma bursts and normally should not be touched).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hidden_reg4](index.html) module"]
pub struct HIDDEN_REG4_SPEC;
impl crate::RegisterSpec for HIDDEN_REG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hidden_reg4::R](R) reader structure"]
impl crate::Readable for HIDDEN_REG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hidden_reg4::W](W) writer structure"]
impl crate::Writable for HIDDEN_REG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIDDEN_REG4 to value 0x0006_0008"]
impl crate::Resettable for HIDDEN_REG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_0008;
}
