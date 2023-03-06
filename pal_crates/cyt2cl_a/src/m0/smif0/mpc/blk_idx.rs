#[doc = "Register `BLK_IDX` reader"]
pub struct R(crate::R<BLK_IDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_IDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_IDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_IDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK_IDX` writer"]
pub struct W(crate::W<BLK_IDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK_IDX_SPEC>;
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
impl From<crate::W<BLK_IDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK_IDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Index value for accessing block-based lookup table using BLK_LUT. Programming out of LUT range is an user error and it loops back to '0' once overflow occurs."]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Index value for accessing block-based lookup table using BLK_LUT. Programming out of LUT range is an user error and it loops back to '0' once overflow occurs."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLK_IDX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Index value for accessing block-based lookup table using BLK_LUT. Programming out of LUT range is an user error and it loops back to '0' once overflow occurs."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Index value for accessing block-based lookup table using BLK_LUT. Programming out of LUT range is an user error and it loops back to '0' once overflow occurs."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Index of 32-block group accessed through BLK_LUT (Separate IDX for each PC depending on access_pc)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_idx](index.html) module"]
pub struct BLK_IDX_SPEC;
impl crate::RegisterSpec for BLK_IDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_idx::R](R) reader structure"]
impl crate::Readable for BLK_IDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk_idx::W](W) writer structure"]
impl crate::Writable for BLK_IDX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK_IDX to value 0"]
impl crate::Resettable for BLK_IDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
