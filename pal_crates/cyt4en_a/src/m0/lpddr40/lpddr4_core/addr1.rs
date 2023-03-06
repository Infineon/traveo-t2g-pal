#[doc = "Register `ADDR1` reader"]
pub struct R(crate::R<ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR1` writer"]
pub struct W(crate::W<ADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR1_SPEC>;
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
impl From<crate::W<ADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COL_ADDR_MAP_B6` reader - Address mapping for column bit 6"]
pub type COL_ADDR_MAP_B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COL_ADDR_MAP_B6` writer - Address mapping for column bit 6"]
pub type COL_ADDR_MAP_B6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COL_ADDR_MAP_B7` reader - Address mapping for column bit 7"]
pub type COL_ADDR_MAP_B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COL_ADDR_MAP_B7` writer - Address mapping for column bit 7"]
pub type COL_ADDR_MAP_B7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COL_ADDR_MAP_B8` reader - Address mapping for column bit 8"]
pub type COL_ADDR_MAP_B8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COL_ADDR_MAP_B8` writer - Address mapping for column bit 8"]
pub type COL_ADDR_MAP_B8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COL_ADDR_MAP_B9` reader - Address mapping for column bit 9"]
pub type COL_ADDR_MAP_B9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COL_ADDR_MAP_B9` writer - Address mapping for column bit 9"]
pub type COL_ADDR_MAP_B9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Address mapping for column bit 6"]
    #[inline(always)]
    pub fn col_addr_map_b6(&self) -> COL_ADDR_MAP_B6_R {
        COL_ADDR_MAP_B6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Address mapping for column bit 7"]
    #[inline(always)]
    pub fn col_addr_map_b7(&self) -> COL_ADDR_MAP_B7_R {
        COL_ADDR_MAP_B7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Address mapping for column bit 8"]
    #[inline(always)]
    pub fn col_addr_map_b8(&self) -> COL_ADDR_MAP_B8_R {
        COL_ADDR_MAP_B8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Address mapping for column bit 9"]
    #[inline(always)]
    pub fn col_addr_map_b9(&self) -> COL_ADDR_MAP_B9_R {
        COL_ADDR_MAP_B9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address mapping for column bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn col_addr_map_b6(&mut self) -> COL_ADDR_MAP_B6_W<0> {
        COL_ADDR_MAP_B6_W::new(self)
    }
    #[doc = "Bits 5:9 - Address mapping for column bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn col_addr_map_b7(&mut self) -> COL_ADDR_MAP_B7_W<5> {
        COL_ADDR_MAP_B7_W::new(self)
    }
    #[doc = "Bits 10:14 - Address mapping for column bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn col_addr_map_b8(&mut self) -> COL_ADDR_MAP_B8_W<10> {
        COL_ADDR_MAP_B8_W::new(self)
    }
    #[doc = "Bits 15:19 - Address mapping for column bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn col_addr_map_b9(&mut self) -> COL_ADDR_MAP_B9_W<15> {
        COL_ADDR_MAP_B9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DRAM Address Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](index.html) module"]
pub struct ADDR1_SPEC;
impl crate::RegisterSpec for ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr1::R](R) reader structure"]
impl crate::Readable for ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr1::W](W) writer structure"]
impl crate::Writable for ADDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for ADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
