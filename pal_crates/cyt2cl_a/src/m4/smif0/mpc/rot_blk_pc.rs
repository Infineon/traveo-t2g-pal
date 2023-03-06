#[doc = "Register `ROT_BLK_PC` reader"]
pub struct R(crate::R<ROT_BLK_PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROT_BLK_PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROT_BLK_PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROT_BLK_PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROT_BLK_PC` writer"]
pub struct W(crate::W<ROT_BLK_PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROT_BLK_PC_SPEC>;
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
impl From<crate::W<ROT_BLK_PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROT_BLK_PC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC` reader - Specify PC values for ROT_BLK_IDX and ROT_BLK_LUT"]
pub type PC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC` writer - Specify PC values for ROT_BLK_IDX and ROT_BLK_LUT"]
pub type PC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_BLK_PC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Specify PC values for ROT_BLK_IDX and ROT_BLK_LUT"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Specify PC values for ROT_BLK_IDX and ROT_BLK_LUT"]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<0> {
        PC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protection context of 8-block group accesses through ROT_BLK_LUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rot_blk_pc](index.html) module"]
pub struct ROT_BLK_PC_SPEC;
impl crate::RegisterSpec for ROT_BLK_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rot_blk_pc::R](R) reader structure"]
impl crate::Readable for ROT_BLK_PC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rot_blk_pc::W](W) writer structure"]
impl crate::Writable for ROT_BLK_PC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROT_BLK_PC to value 0"]
impl crate::Resettable for ROT_BLK_PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
