#[doc = "Register `ADDR2` reader"]
pub struct R(crate::R<ADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR2` writer"]
pub struct W(crate::W<ADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR2_SPEC>;
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
impl From<crate::W<ADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROW_ADDR_MAP_B0` reader - Address mapping for row bit 0"]
pub type ROW_ADDR_MAP_B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B0` writer - Address mapping for row bit 0"]
pub type ROW_ADDR_MAP_B0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ROW_ADDR_MAP_B1` reader - Address mapping for row bit 1"]
pub type ROW_ADDR_MAP_B1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B1` writer - Address mapping for row bit 1"]
pub type ROW_ADDR_MAP_B1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ROW_ADDR_MAP_B2` reader - Address mapping for row bit 2"]
pub type ROW_ADDR_MAP_B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B2` writer - Address mapping for row bit 2"]
pub type ROW_ADDR_MAP_B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ROW_ADDR_MAP_B3` reader - Address mapping for row bit 3"]
pub type ROW_ADDR_MAP_B3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B3` writer - Address mapping for row bit 3"]
pub type ROW_ADDR_MAP_B3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ROW_ADDR_MAP_B4` reader - Address mapping for row bit 4"]
pub type ROW_ADDR_MAP_B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B4` writer - Address mapping for row bit 4"]
pub type ROW_ADDR_MAP_B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ROW_ADDR_MAP_B5` reader - Address mapping for row bit 5"]
pub type ROW_ADDR_MAP_B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_ADDR_MAP_B5` writer - Address mapping for row bit 5"]
pub type ROW_ADDR_MAP_B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Address mapping for row bit 0"]
    #[inline(always)]
    pub fn row_addr_map_b0(&self) -> ROW_ADDR_MAP_B0_R {
        ROW_ADDR_MAP_B0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Address mapping for row bit 1"]
    #[inline(always)]
    pub fn row_addr_map_b1(&self) -> ROW_ADDR_MAP_B1_R {
        ROW_ADDR_MAP_B1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Address mapping for row bit 2"]
    #[inline(always)]
    pub fn row_addr_map_b2(&self) -> ROW_ADDR_MAP_B2_R {
        ROW_ADDR_MAP_B2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Address mapping for row bit 3"]
    #[inline(always)]
    pub fn row_addr_map_b3(&self) -> ROW_ADDR_MAP_B3_R {
        ROW_ADDR_MAP_B3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Address mapping for row bit 4"]
    #[inline(always)]
    pub fn row_addr_map_b4(&self) -> ROW_ADDR_MAP_B4_R {
        ROW_ADDR_MAP_B4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Address mapping for row bit 5"]
    #[inline(always)]
    pub fn row_addr_map_b5(&self) -> ROW_ADDR_MAP_B5_R {
        ROW_ADDR_MAP_B5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address mapping for row bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b0(&mut self) -> ROW_ADDR_MAP_B0_W<0> {
        ROW_ADDR_MAP_B0_W::new(self)
    }
    #[doc = "Bits 5:9 - Address mapping for row bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b1(&mut self) -> ROW_ADDR_MAP_B1_W<5> {
        ROW_ADDR_MAP_B1_W::new(self)
    }
    #[doc = "Bits 10:14 - Address mapping for row bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b2(&mut self) -> ROW_ADDR_MAP_B2_W<10> {
        ROW_ADDR_MAP_B2_W::new(self)
    }
    #[doc = "Bits 15:19 - Address mapping for row bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b3(&mut self) -> ROW_ADDR_MAP_B3_W<15> {
        ROW_ADDR_MAP_B3_W::new(self)
    }
    #[doc = "Bits 20:24 - Address mapping for row bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b4(&mut self) -> ROW_ADDR_MAP_B4_W<20> {
        ROW_ADDR_MAP_B4_W::new(self)
    }
    #[doc = "Bits 25:29 - Address mapping for row bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn row_addr_map_b5(&mut self) -> ROW_ADDR_MAP_B5_W<25> {
        ROW_ADDR_MAP_B5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DRAM Address Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr2](index.html) module"]
pub struct ADDR2_SPEC;
impl crate::RegisterSpec for ADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr2::R](R) reader structure"]
impl crate::Readable for ADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr2::W](W) writer structure"]
impl crate::Writable for ADDR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR2 to value 0"]
impl crate::Resettable for ADDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
