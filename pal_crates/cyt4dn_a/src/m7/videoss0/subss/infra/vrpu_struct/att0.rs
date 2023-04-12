#[doc = "Register `ATT0` reader"]
pub struct R(crate::R<ATT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATT0` writer"]
pub struct W(crate::W<ATT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATT0_SPEC>;
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
impl From<crate::W<ATT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATT0_UR` reader - See CPUSS."]
pub type ATT0_UR_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_UR` writer - See CPUSS."]
pub type ATT0_UR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_UW` reader - See CPUSS."]
pub type ATT0_UW_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_UW` writer - See CPUSS."]
pub type ATT0_UW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_UX` reader - See CPUSS."]
pub type ATT0_UX_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_UX` writer - See CPUSS."]
pub type ATT0_UX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_PR` reader - See CPUSS."]
pub type ATT0_PR_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_PR` writer - See CPUSS."]
pub type ATT0_PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_PW` reader - See CPUSS."]
pub type ATT0_PW_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_PW` writer - See CPUSS."]
pub type ATT0_PW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_PX` reader - See CPUSS."]
pub type ATT0_PX_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_PX` writer - See CPUSS."]
pub type ATT0_PX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_NS` reader - See CPUSS."]
pub type ATT0_NS_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_NS` writer - See CPUSS."]
pub type ATT0_NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_PC_MASK_0` reader - See CPUSS."]
pub type ATT0_PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_PC_MASK_15_TO_1` reader - See CPUSS."]
pub type ATT0_PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ATT0_PC_MASK_15_TO_1` writer - See CPUSS."]
pub type ATT0_PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ATT0_SPEC, u16, u16, 15, O>;
#[doc = "Field `ATT0_REGION_SIZE` reader - See CPUSS."]
pub type ATT0_REGION_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATT0_REGION_SIZE` writer - See CPUSS."]
pub type ATT0_REGION_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `ATT0_PC_MATCH` reader - See CPUSS."]
pub type ATT0_PC_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_PC_MATCH` writer - See CPUSS."]
pub type ATT0_PC_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ATT0_ENABLED` reader - See CPUSS."]
pub type ATT0_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ATT0_ENABLED` writer - See CPUSS."]
pub type ATT0_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See CPUSS."]
    #[inline(always)]
    pub fn att0_ur(&self) -> ATT0_UR_R {
        ATT0_UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See CPUSS."]
    #[inline(always)]
    pub fn att0_uw(&self) -> ATT0_UW_R {
        ATT0_UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - See CPUSS."]
    #[inline(always)]
    pub fn att0_ux(&self) -> ATT0_UX_R {
        ATT0_UX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - See CPUSS."]
    #[inline(always)]
    pub fn att0_pr(&self) -> ATT0_PR_R {
        ATT0_PR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See CPUSS."]
    #[inline(always)]
    pub fn att0_pw(&self) -> ATT0_PW_R {
        ATT0_PW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See CPUSS."]
    #[inline(always)]
    pub fn att0_px(&self) -> ATT0_PX_R {
        ATT0_PX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See CPUSS."]
    #[inline(always)]
    pub fn att0_ns(&self) -> ATT0_NS_R {
        ATT0_NS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - See CPUSS."]
    #[inline(always)]
    pub fn att0_pc_mask_0(&self) -> ATT0_PC_MASK_0_R {
        ATT0_PC_MASK_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:23 - See CPUSS."]
    #[inline(always)]
    pub fn att0_pc_mask_15_to_1(&self) -> ATT0_PC_MASK_15_TO_1_R {
        ATT0_PC_MASK_15_TO_1_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - See CPUSS."]
    #[inline(always)]
    pub fn att0_region_size(&self) -> ATT0_REGION_SIZE_R {
        ATT0_REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - See CPUSS."]
    #[inline(always)]
    pub fn att0_pc_match(&self) -> ATT0_PC_MATCH_R {
        ATT0_PC_MATCH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See CPUSS."]
    #[inline(always)]
    pub fn att0_enabled(&self) -> ATT0_ENABLED_R {
        ATT0_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_ur(&mut self) -> ATT0_UR_W<0> {
        ATT0_UR_W::new(self)
    }
    #[doc = "Bit 1 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_uw(&mut self) -> ATT0_UW_W<1> {
        ATT0_UW_W::new(self)
    }
    #[doc = "Bit 2 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_ux(&mut self) -> ATT0_UX_W<2> {
        ATT0_UX_W::new(self)
    }
    #[doc = "Bit 3 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_pr(&mut self) -> ATT0_PR_W<3> {
        ATT0_PR_W::new(self)
    }
    #[doc = "Bit 4 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_pw(&mut self) -> ATT0_PW_W<4> {
        ATT0_PW_W::new(self)
    }
    #[doc = "Bit 5 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_px(&mut self) -> ATT0_PX_W<5> {
        ATT0_PX_W::new(self)
    }
    #[doc = "Bit 6 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_ns(&mut self) -> ATT0_NS_W<6> {
        ATT0_NS_W::new(self)
    }
    #[doc = "Bits 9:23 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_pc_mask_15_to_1(&mut self) -> ATT0_PC_MASK_15_TO_1_W<9> {
        ATT0_PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Bits 24:28 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_region_size(&mut self) -> ATT0_REGION_SIZE_W<24> {
        ATT0_REGION_SIZE_W::new(self)
    }
    #[doc = "Bit 30 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_pc_match(&mut self) -> ATT0_PC_MATCH_W<30> {
        ATT0_PC_MATCH_W::new(self)
    }
    #[doc = "Bit 31 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att0_enabled(&mut self) -> ATT0_ENABLED_W<31> {
        ATT0_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ATT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att0](index.html) module"]
pub struct ATT0_SPEC;
impl crate::RegisterSpec for ATT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [att0::R](R) reader structure"]
impl crate::Readable for ATT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [att0::W](W) writer structure"]
impl crate::Writable for ATT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATT0 to value 0x0100"]
impl crate::Resettable for ATT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
