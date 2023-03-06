#[doc = "Register `ROT_BLK_LUT` reader"]
pub struct R(crate::R<ROT_BLK_LUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROT_BLK_LUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROT_BLK_LUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROT_BLK_LUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROT_BLK_LUT` writer"]
pub struct W(crate::W<ROT_BLK_LUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROT_BLK_LUT_SPEC>;
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
impl From<crate::W<ROT_BLK_LUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROT_BLK_LUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTR0` reader - W/R/NS bits for block 0 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR0` writer - W/R/NS bits for block 0 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATTR1` reader - W/R/NS bits for block 1 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR1` writer - W/R/NS bits for block 1 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATTR2` reader - W/R/NS bits for block 2 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR2` writer - W/R/NS bits for block 2 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATTR3` reader - W/R/NS bits for block 3 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR3` writer - W/R/NS bits for block 3 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATTR4` reader - W/R/NS bits for block 4 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR4` writer - W/R/NS bits for block 4 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATTR5` reader - W/R/NS bits for block 5 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR5` writer - W/R/NS bits for block 5 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATTR6` reader - W/R/NS bits for block 6 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR6` writer - W/R/NS bits for block 6 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATTR7` reader - W/R/NS bits for block 7 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR7` writer - W/R/NS bits for block 7 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
pub type ATTR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_LUT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - W/R/NS bits for block 0 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr0(&self) -> ATTR0_R {
        ATTR0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - W/R/NS bits for block 1 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - W/R/NS bits for block 2 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - W/R/NS bits for block 3 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr3(&self) -> ATTR3_R {
        ATTR3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - W/R/NS bits for block 4 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr4(&self) -> ATTR4_R {
        ATTR4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - W/R/NS bits for block 5 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr5(&self) -> ATTR5_R {
        ATTR5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - W/R/NS bits for block 6 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr6(&self) -> ATTR6_R {
        ATTR6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - W/R/NS bits for block 7 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    pub fn attr7(&self) -> ATTR7_R {
        ATTR7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - W/R/NS bits for block 0 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr0(&mut self) -> ATTR0_W<0> {
        ATTR0_W::new(self)
    }
    #[doc = "Bits 4:6 - W/R/NS bits for block 1 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr1(&mut self) -> ATTR1_W<4> {
        ATTR1_W::new(self)
    }
    #[doc = "Bits 8:10 - W/R/NS bits for block 2 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr2(&mut self) -> ATTR2_W<8> {
        ATTR2_W::new(self)
    }
    #[doc = "Bits 12:14 - W/R/NS bits for block 3 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr3(&mut self) -> ATTR3_W<12> {
        ATTR3_W::new(self)
    }
    #[doc = "Bits 16:18 - W/R/NS bits for block 4 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr4(&mut self) -> ATTR4_W<16> {
        ATTR4_W::new(self)
    }
    #[doc = "Bits 20:22 - W/R/NS bits for block 5 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr5(&mut self) -> ATTR5_W<20> {
        ATTR5_W::new(self)
    }
    #[doc = "Bits 24:26 - W/R/NS bits for block 6 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr6(&mut self) -> ATTR6_W<24> {
        ATTR6_W::new(self)
    }
    #[doc = "Bits 28:30 - W/R/NS bits for block 7 indicated by ROT_BLK_IDX for ROT_BLK_PC PC"]
    #[inline(always)]
    #[must_use]
    pub fn attr7(&mut self) -> ATTR7_W<28> {
        ATTR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "(R,W,NS) bits for 8 blocks at ROT_BLK_IDX for PC=ROT_BKL_PC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rot_blk_lut](index.html) module"]
pub struct ROT_BLK_LUT_SPEC;
impl crate::RegisterSpec for ROT_BLK_LUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rot_blk_lut::R](R) reader structure"]
impl crate::Readable for ROT_BLK_LUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rot_blk_lut::W](W) writer structure"]
impl crate::Writable for ROT_BLK_LUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROT_BLK_LUT to value 0"]
impl crate::Resettable for ROT_BLK_LUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
