#[doc = "Register `SCREENING_TYPE_2_REGISTER_3` reader"]
pub struct R(crate::R<SCREENING_TYPE_2_REGISTER_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCREENING_TYPE_2_REGISTER_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCREENING_TYPE_2_REGISTER_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCREENING_TYPE_2_REGISTER_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCREENING_TYPE_2_REGISTER_3` writer"]
pub struct W(crate::W<SCREENING_TYPE_2_REGISTER_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCREENING_TYPE_2_REGISTER_3_SPEC>;
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
impl From<crate::W<SCREENING_TYPE_2_REGISTER_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCREENING_TYPE_2_REGISTER_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUEUE_NUMBER` reader - 'Queue Number (0 to 15)'"]
pub type QUEUE_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QUEUE_NUMBER` writer - 'Queue Number (0 to 15)'"]
pub type QUEUE_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, u8, u8, 4, O>;
#[doc = "Field `VLAN_PRIORITY` reader - 'VLAN Priority'"]
pub type VLAN_PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLAN_PRIORITY` writer - 'VLAN Priority'"]
pub type VLAN_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSVD_7` reader - N/A"]
pub type RSVD_7_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_7` writer - N/A"]
pub type RSVD_7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, bool, O>;
#[doc = "Field `VLAN_ENABLE` reader - 'VLAN Enable'"]
pub type VLAN_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `VLAN_ENABLE` writer - 'VLAN Enable'"]
pub type VLAN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, bool, O>;
#[doc = "Field `INDEX` reader - 'Index to screener type 2 EtherType register'"]
pub type INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INDEX` writer - 'Index to screener type 2 EtherType register'"]
pub type INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETHERTYPE_ENABLE` reader - 'EtherType Enable'"]
pub type ETHERTYPE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ETHERTYPE_ENABLE` writer - 'EtherType Enable'"]
pub type ETHERTYPE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, bool, O>;
#[doc = "Field `COMPARE_A` reader - 'Compare A - Index to screener type 2 Compare register '"]
pub type COMPARE_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPARE_A` writer - 'Compare A - Index to screener type 2 Compare register '"]
pub type COMPARE_A_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPARE_A_ENABLE` reader - 'Compare A Enable'"]
pub type COMPARE_A_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `COMPARE_A_ENABLE` writer - 'Compare A Enable'"]
pub type COMPARE_A_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, bool, O>;
#[doc = "Field `COMPARE_B` reader - 'Compare B - Index to screener type 2 Compare register'"]
pub type COMPARE_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPARE_B` writer - 'Compare B - Index to screener type 2 Compare register'"]
pub type COMPARE_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPARE_B_ENABLE` reader - 'Compare B Enable'"]
pub type COMPARE_B_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `COMPARE_B_ENABLE` writer - 'Compare B Enable'"]
pub type COMPARE_B_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, bool, O>;
#[doc = "Field `COMPARE_C` reader - 'Compare C - Index to screener type 2 Compare register'"]
pub type COMPARE_C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPARE_C` writer - 'Compare C - Index to screener type 2 Compare register'"]
pub type COMPARE_C_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPARE_C_ENABLE` reader - 'Compare C Enable'"]
pub type COMPARE_C_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `COMPARE_C_ENABLE` writer - 'Compare C Enable'"]
pub type COMPARE_C_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_2_REGISTER_3_SPEC, bool, O>;
#[doc = "Field `RSVD_31` reader - N/A"]
pub type RSVD_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - 'Queue Number (0 to 15)'"]
    #[inline(always)]
    pub fn queue_number(&self) -> QUEUE_NUMBER_R {
        QUEUE_NUMBER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 'VLAN Priority'"]
    #[inline(always)]
    pub fn vlan_priority(&self) -> VLAN_PRIORITY_R {
        VLAN_PRIORITY_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 'VLAN Enable'"]
    #[inline(always)]
    pub fn vlan_enable(&self) -> VLAN_ENABLE_R {
        VLAN_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 'Index to screener type 2 EtherType register'"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - 'EtherType Enable'"]
    #[inline(always)]
    pub fn ethertype_enable(&self) -> ETHERTYPE_ENABLE_R {
        ETHERTYPE_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - 'Compare A - Index to screener type 2 Compare register '"]
    #[inline(always)]
    pub fn compare_a(&self) -> COMPARE_A_R {
        COMPARE_A_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - 'Compare A Enable'"]
    #[inline(always)]
    pub fn compare_a_enable(&self) -> COMPARE_A_ENABLE_R {
        COMPARE_A_ENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 'Compare B - Index to screener type 2 Compare register'"]
    #[inline(always)]
    pub fn compare_b(&self) -> COMPARE_B_R {
        COMPARE_B_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 'Compare B Enable'"]
    #[inline(always)]
    pub fn compare_b_enable(&self) -> COMPARE_B_ENABLE_R {
        COMPARE_B_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - 'Compare C - Index to screener type 2 Compare register'"]
    #[inline(always)]
    pub fn compare_c(&self) -> COMPARE_C_R {
        COMPARE_C_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - 'Compare C Enable'"]
    #[inline(always)]
    pub fn compare_c_enable(&self) -> COMPARE_C_ENABLE_R {
        COMPARE_C_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn rsvd_31(&self) -> RSVD_31_R {
        RSVD_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 'Queue Number (0 to 15)'"]
    #[inline(always)]
    #[must_use]
    pub fn queue_number(&mut self) -> QUEUE_NUMBER_W<0> {
        QUEUE_NUMBER_W::new(self)
    }
    #[doc = "Bits 4:6 - 'VLAN Priority'"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_priority(&mut self) -> VLAN_PRIORITY_W<4> {
        VLAN_PRIORITY_W::new(self)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_7(&mut self) -> RSVD_7_W<7> {
        RSVD_7_W::new(self)
    }
    #[doc = "Bit 8 - 'VLAN Enable'"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_enable(&mut self) -> VLAN_ENABLE_W<8> {
        VLAN_ENABLE_W::new(self)
    }
    #[doc = "Bits 9:11 - 'Index to screener type 2 EtherType register'"]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<9> {
        INDEX_W::new(self)
    }
    #[doc = "Bit 12 - 'EtherType Enable'"]
    #[inline(always)]
    #[must_use]
    pub fn ethertype_enable(&mut self) -> ETHERTYPE_ENABLE_W<12> {
        ETHERTYPE_ENABLE_W::new(self)
    }
    #[doc = "Bits 13:17 - 'Compare A - Index to screener type 2 Compare register '"]
    #[inline(always)]
    #[must_use]
    pub fn compare_a(&mut self) -> COMPARE_A_W<13> {
        COMPARE_A_W::new(self)
    }
    #[doc = "Bit 18 - 'Compare A Enable'"]
    #[inline(always)]
    #[must_use]
    pub fn compare_a_enable(&mut self) -> COMPARE_A_ENABLE_W<18> {
        COMPARE_A_ENABLE_W::new(self)
    }
    #[doc = "Bits 19:23 - 'Compare B - Index to screener type 2 Compare register'"]
    #[inline(always)]
    #[must_use]
    pub fn compare_b(&mut self) -> COMPARE_B_W<19> {
        COMPARE_B_W::new(self)
    }
    #[doc = "Bit 24 - 'Compare B Enable'"]
    #[inline(always)]
    #[must_use]
    pub fn compare_b_enable(&mut self) -> COMPARE_B_ENABLE_W<24> {
        COMPARE_B_ENABLE_W::new(self)
    }
    #[doc = "Bits 25:29 - 'Compare C - Index to screener type 2 Compare register'"]
    #[inline(always)]
    #[must_use]
    pub fn compare_c(&mut self) -> COMPARE_C_W<25> {
        COMPARE_C_W::new(self)
    }
    #[doc = "Bit 30 - 'Compare C Enable'"]
    #[inline(always)]
    #[must_use]
    pub fn compare_c_enable(&mut self) -> COMPARE_C_ENABLE_W<30> {
        COMPARE_C_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "screening type 2 register 3, same as screening_type_2_register_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [screening_type_2_register_3](index.html) module"]
pub struct SCREENING_TYPE_2_REGISTER_3_SPEC;
impl crate::RegisterSpec for SCREENING_TYPE_2_REGISTER_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [screening_type_2_register_3::R](R) reader structure"]
impl crate::Readable for SCREENING_TYPE_2_REGISTER_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [screening_type_2_register_3::W](W) writer structure"]
impl crate::Writable for SCREENING_TYPE_2_REGISTER_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCREENING_TYPE_2_REGISTER_3 to value 0"]
impl crate::Resettable for SCREENING_TYPE_2_REGISTER_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
