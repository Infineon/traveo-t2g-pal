#[doc = "Register `ADDR5` reader"]
pub struct R(crate::R<ADDR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR5` writer"]
pub struct W(crate::W<ADDR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR5_SPEC>;
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
impl From<crate::W<ADDR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANK_ADDR_MAP_B0` reader - Address mapping for bank bit 0"]
pub type BANK_ADDR_MAP_B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANK_ADDR_MAP_B0` writer - Address mapping for bank bit 0"]
pub type BANK_ADDR_MAP_B0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `BANK_ADDR_MAP_B1` reader - Address mapping for bank bit 1"]
pub type BANK_ADDR_MAP_B1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANK_ADDR_MAP_B1` writer - Address mapping for bank bit 1"]
pub type BANK_ADDR_MAP_B1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `BANK_ADDR_MAP_B2` reader - Address mapping for bank bit 2"]
pub type BANK_ADDR_MAP_B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANK_ADDR_MAP_B2` writer - Address mapping for bank bit 2"]
pub type BANK_ADDR_MAP_B2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHAN_ADDR_MAP_B0` reader - Address mapping for channel bit 0"]
pub type CHAN_ADDR_MAP_B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHAN_ADDR_MAP_B0` writer - Address mapping for channel bit 0"]
pub type CHAN_ADDR_MAP_B0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR5_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Address mapping for bank bit 0"]
    #[inline(always)]
    pub fn bank_addr_map_b0(&self) -> BANK_ADDR_MAP_B0_R {
        BANK_ADDR_MAP_B0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Address mapping for bank bit 1"]
    #[inline(always)]
    pub fn bank_addr_map_b1(&self) -> BANK_ADDR_MAP_B1_R {
        BANK_ADDR_MAP_B1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Address mapping for bank bit 2"]
    #[inline(always)]
    pub fn bank_addr_map_b2(&self) -> BANK_ADDR_MAP_B2_R {
        BANK_ADDR_MAP_B2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Address mapping for channel bit 0"]
    #[inline(always)]
    pub fn chan_addr_map_b0(&self) -> CHAN_ADDR_MAP_B0_R {
        CHAN_ADDR_MAP_B0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address mapping for bank bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bank_addr_map_b0(&mut self) -> BANK_ADDR_MAP_B0_W<0> {
        BANK_ADDR_MAP_B0_W::new(self)
    }
    #[doc = "Bits 5:9 - Address mapping for bank bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bank_addr_map_b1(&mut self) -> BANK_ADDR_MAP_B1_W<5> {
        BANK_ADDR_MAP_B1_W::new(self)
    }
    #[doc = "Bits 10:14 - Address mapping for bank bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bank_addr_map_b2(&mut self) -> BANK_ADDR_MAP_B2_W<10> {
        BANK_ADDR_MAP_B2_W::new(self)
    }
    #[doc = "Bits 15:19 - Address mapping for channel bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn chan_addr_map_b0(&mut self) -> CHAN_ADDR_MAP_B0_W<15> {
        CHAN_ADDR_MAP_B0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DRAM Address Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr5](index.html) module"]
pub struct ADDR5_SPEC;
impl crate::RegisterSpec for ADDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr5::R](R) reader structure"]
impl crate::Readable for ADDR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr5::W](W) writer structure"]
impl crate::Writable for ADDR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR5 to value 0"]
impl crate::Resettable for ADDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
