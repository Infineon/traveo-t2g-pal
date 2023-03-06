#[doc = "Register `ROT_CFG` reader"]
pub struct R(crate::R<ROT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROT_CFG` writer"]
pub struct W(crate::W<ROT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROT_CFG_SPEC>;
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
impl From<crate::W<ROT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_SIZE` reader - Block size of individually protected blocks (0: 32B, 1: 64B, ...up to 15:1 MB) Block size= (1&lt;&lt;(BLOCK_SIZE+5)) The number and size blocks in an MPC is design time configurable and for external memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
pub type BLOCK_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLOCK_SIZE` writer - Block size of individually protected blocks (0: 32B, 1: 64B, ...up to 15:1 MB) Block size= (1&lt;&lt;(BLOCK_SIZE+5)) The number and size blocks in an MPC is design time configurable and for external memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
pub type BLOCK_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROT_CFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Block size of individually protected blocks (0: 32B, 1: 64B, ...up to 15:1 MB) Block size= (1&lt;&lt;(BLOCK_SIZE+5)) The number and size blocks in an MPC is design time configurable and for external memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Block size of individually protected blocks (0: 32B, 1: 64B, ...up to 15:1 MB) Block size= (1&lt;&lt;(BLOCK_SIZE+5)) The number and size blocks in an MPC is design time configurable and for external memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W<0> {
        BLOCK_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sets block-size to match memory size (external memory only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rot_cfg](index.html) module"]
pub struct ROT_CFG_SPEC;
impl crate::RegisterSpec for ROT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rot_cfg::R](R) reader structure"]
impl crate::Readable for ROT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rot_cfg::W](W) writer structure"]
impl crate::Writable for ROT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROT_CFG to value 0x07"]
impl crate::Resettable for ROT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
