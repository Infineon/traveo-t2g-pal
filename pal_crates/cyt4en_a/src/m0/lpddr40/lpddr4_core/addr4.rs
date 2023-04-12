#[doc = "Register `ADDR4` reader"]
pub struct R(crate::R<ADDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR4` writer"]
pub struct W(crate::W<ADDR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR4_SPEC>;
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
impl From<crate::W<ADDR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROW_ADDR_MAP_B12` reader - Address mapping for row bit 12"]
pub type ROW_ADDR_MAP_B12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B12` writer - Address mapping for row bit 12"]
pub type ROW_ADDR_MAP_B12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `ROW_ADDR_MAP_B13` reader - Address mapping for row bit 13"]
pub type ROW_ADDR_MAP_B13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B13` writer - Address mapping for row bit 13"]
pub type ROW_ADDR_MAP_B13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `ROW_ADDR_MAP_B14` reader - Address mapping for row bit 14"]
pub type ROW_ADDR_MAP_B14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B14` writer - Address mapping for row bit 14"]
pub type ROW_ADDR_MAP_B14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Address mapping for row bit 12"]
    #[inline(always)]
    pub fn row_addr_map_b12(&self) -> ROW_ADDR_MAP_B12_R {
        ROW_ADDR_MAP_B12_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Address mapping for row bit 13"]
    #[inline(always)]
    pub fn row_addr_map_b13(&self) -> ROW_ADDR_MAP_B13_R {
        ROW_ADDR_MAP_B13_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Address mapping for row bit 14"]
    #[inline(always)]
    pub fn row_addr_map_b14(&self) -> ROW_ADDR_MAP_B14_R {
        ROW_ADDR_MAP_B14_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address mapping for row bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b12(&mut self) -> ROW_ADDR_MAP_B12_W<0> {
        ROW_ADDR_MAP_B12_W::new(self)
    }
    #[doc = "Bits 5:9 - Address mapping for row bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b13(&mut self) -> ROW_ADDR_MAP_B13_W<5> {
        ROW_ADDR_MAP_B13_W::new(self)
    }
    #[doc = "Bits 10:14 - Address mapping for row bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b14(&mut self) -> ROW_ADDR_MAP_B14_W<10> {
        ROW_ADDR_MAP_B14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DRAM Address Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr4](index.html) module"]
pub struct ADDR4_SPEC;
impl crate::RegisterSpec for ADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr4::R](R) reader structure"]
impl crate::Readable for ADDR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr4::W](W) writer structure"]
impl crate::Writable for ADDR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR4 to value 0"]
impl crate::Resettable for ADDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
