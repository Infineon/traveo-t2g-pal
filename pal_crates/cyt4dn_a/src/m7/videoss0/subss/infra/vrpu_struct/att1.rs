#[doc = "Register `ATT1` reader"]
pub struct R(crate::R<ATT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATT1` writer"]
pub struct W(crate::W<ATT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATT1_SPEC>;
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
impl From<crate::W<ATT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATT1_UR` reader - See CPUSS."]
pub type ATT1_UR_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_UW` reader - See CPUSS."]
pub type ATT1_UW_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_UW` writer - See CPUSS."]
pub type ATT1_UW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT1_SPEC, bool, O>;
#[doc = "Field `ATT1_UX` reader - See CPUSS."]
pub type ATT1_UX_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_PR` reader - See CPUSS."]
pub type ATT1_PR_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_PW` reader - See CPUSS."]
pub type ATT1_PW_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_PW` writer - See CPUSS."]
pub type ATT1_PW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT1_SPEC, bool, O>;
#[doc = "Field `ATT1_PX` reader - See CPUSS."]
pub type ATT1_PX_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_NS` reader - See CPUSS."]
pub type ATT1_NS_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_NS` writer - See CPUSS."]
pub type ATT1_NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT1_SPEC, bool, O>;
#[doc = "Field `ATT1_PC_MASK_0` reader - See CPUSS."]
pub type ATT1_PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_PC_MASK_15_TO_1` reader - See CPUSS."]
pub type ATT1_PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ATT1_PC_MASK_15_TO_1` writer - See CPUSS."]
pub type ATT1_PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ATT1_SPEC, u16, u16, 15, O>;
#[doc = "Field `ATT1_REGION_SIZE` reader - See CPUSS."]
pub type ATT1_REGION_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATT1_PC_MATCH` reader - See CPUSS."]
pub type ATT1_PC_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_PC_MATCH` writer - See CPUSS."]
pub type ATT1_PC_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT1_SPEC, bool, O>;
#[doc = "Field `ATT1_ENABLED` reader - See CPUSS."]
pub type ATT1_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ATT1_ENABLED` writer - See CPUSS."]
pub type ATT1_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See CPUSS."]
    #[inline(always)]
    pub fn att1_ur(&self) -> ATT1_UR_R {
        ATT1_UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See CPUSS."]
    #[inline(always)]
    pub fn att1_uw(&self) -> ATT1_UW_R {
        ATT1_UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - See CPUSS."]
    #[inline(always)]
    pub fn att1_ux(&self) -> ATT1_UX_R {
        ATT1_UX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - See CPUSS."]
    #[inline(always)]
    pub fn att1_pr(&self) -> ATT1_PR_R {
        ATT1_PR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See CPUSS."]
    #[inline(always)]
    pub fn att1_pw(&self) -> ATT1_PW_R {
        ATT1_PW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See CPUSS."]
    #[inline(always)]
    pub fn att1_px(&self) -> ATT1_PX_R {
        ATT1_PX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See CPUSS."]
    #[inline(always)]
    pub fn att1_ns(&self) -> ATT1_NS_R {
        ATT1_NS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - See CPUSS."]
    #[inline(always)]
    pub fn att1_pc_mask_0(&self) -> ATT1_PC_MASK_0_R {
        ATT1_PC_MASK_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:23 - See CPUSS."]
    #[inline(always)]
    pub fn att1_pc_mask_15_to_1(&self) -> ATT1_PC_MASK_15_TO_1_R {
        ATT1_PC_MASK_15_TO_1_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - See CPUSS."]
    #[inline(always)]
    pub fn att1_region_size(&self) -> ATT1_REGION_SIZE_R {
        ATT1_REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - See CPUSS."]
    #[inline(always)]
    pub fn att1_pc_match(&self) -> ATT1_PC_MATCH_R {
        ATT1_PC_MATCH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See CPUSS."]
    #[inline(always)]
    pub fn att1_enabled(&self) -> ATT1_ENABLED_R {
        ATT1_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att1_uw(&mut self) -> ATT1_UW_W<1> {
        ATT1_UW_W::new(self)
    }
    #[doc = "Bit 4 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att1_pw(&mut self) -> ATT1_PW_W<4> {
        ATT1_PW_W::new(self)
    }
    #[doc = "Bit 6 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att1_ns(&mut self) -> ATT1_NS_W<6> {
        ATT1_NS_W::new(self)
    }
    #[doc = "Bits 9:23 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att1_pc_mask_15_to_1(&mut self) -> ATT1_PC_MASK_15_TO_1_W<9> {
        ATT1_PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Bit 30 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att1_pc_match(&mut self) -> ATT1_PC_MATCH_W<30> {
        ATT1_PC_MATCH_W::new(self)
    }
    #[doc = "Bit 31 - See CPUSS."]
    #[inline(always)]
    #[must_use]
    pub fn att1_enabled(&mut self) -> ATT1_ENABLED_W<31> {
        ATT1_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ATT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att1](index.html) module"]
pub struct ATT1_SPEC;
impl crate::RegisterSpec for ATT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [att1::R](R) reader structure"]
impl crate::Readable for ATT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [att1::W](W) writer structure"]
impl crate::Writable for ATT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATT1 to value 0x0700_0100"]
impl crate::Resettable for ATT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700_0100;
}
