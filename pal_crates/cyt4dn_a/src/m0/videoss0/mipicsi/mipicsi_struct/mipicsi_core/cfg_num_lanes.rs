#[doc = "Register `CFG_NUM_LANES` reader"]
pub struct R(crate::R<CFG_NUM_LANES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_NUM_LANES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_NUM_LANES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_NUM_LANES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_NUM_LANES` writer"]
pub struct W(crate::W<CFG_NUM_LANES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_NUM_LANES_SPEC>;
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
impl From<crate::W<CFG_NUM_LANES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_NUM_LANES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG_NUM_LANES` reader - N/A"]
pub type CFG_NUM_LANES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG_NUM_LANES` writer - N/A"]
pub type CFG_NUM_LANES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_NUM_LANES_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn cfg_num_lanes(&self) -> CFG_NUM_LANES_R {
        CFG_NUM_LANES_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_num_lanes(&mut self) -> CFG_NUM_LANES_W<0> {
        CFG_NUM_LANES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFG_NUM_LANES is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_num_lanes](index.html) module"]
pub struct CFG_NUM_LANES_SPEC;
impl crate::RegisterSpec for CFG_NUM_LANES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_num_lanes::R](R) reader structure"]
impl crate::Readable for CFG_NUM_LANES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_num_lanes::W](W) writer structure"]
impl crate::Writable for CFG_NUM_LANES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_NUM_LANES to value 0"]
impl crate::Resettable for CFG_NUM_LANES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
