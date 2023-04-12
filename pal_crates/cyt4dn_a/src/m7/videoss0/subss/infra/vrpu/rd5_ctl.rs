#[doc = "Register `RD5_CTL` reader"]
pub struct R(crate::R<RD5_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD5_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD5_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD5_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD5_CTL` writer"]
pub struct W(crate::W<RD5_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD5_CTL_SPEC>;
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
impl From<crate::W<RD5_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD5_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD5_P` reader - Privileged setting ('0': user mode; '1': privileged mode)."]
pub type RD5_P_R = crate::BitReader<bool>;
#[doc = "Field `RD5_P` writer - Privileged setting ('0': user mode; '1': privileged mode)."]
pub type RD5_P_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD5_CTL_SPEC, bool, O>;
#[doc = "Field `RD5_NS` reader - Security setting ('0': secure mode; '1': non-secure mode)."]
pub type RD5_NS_R = crate::BitReader<bool>;
#[doc = "Field `RD5_NS` writer - Security setting ('0': secure mode; '1': non-secure mode)."]
pub type RD5_NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD5_CTL_SPEC, bool, O>;
#[doc = "Field `RD5_PRIO` reader - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
pub type RD5_PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD5_PRIO` writer - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
pub type RD5_PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD5_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RD5_PC_MASK_0` reader - Protection context mask for protection context '0'. This field is a constant '0'."]
pub type RD5_PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `RD5_PC_MASK_15_TO_1` reader - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'."]
pub type RD5_PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RD5_PC_MASK_15_TO_1` writer - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'."]
pub type RD5_PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RD5_CTL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode)."]
    #[inline(always)]
    pub fn rd5_p(&self) -> RD5_P_R {
        RD5_P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode)."]
    #[inline(always)]
    pub fn rd5_ns(&self) -> RD5_NS_R {
        RD5_NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
    #[inline(always)]
    pub fn rd5_prio(&self) -> RD5_PRIO_R {
        RD5_PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Protection context mask for protection context '0'. This field is a constant '0'."]
    #[inline(always)]
    pub fn rd5_pc_mask_0(&self) -> RD5_PC_MASK_0_R {
        RD5_PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'."]
    #[inline(always)]
    pub fn rd5_pc_mask_15_to_1(&self) -> RD5_PC_MASK_15_TO_1_R {
        RD5_PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode)."]
    #[inline(always)]
    #[must_use]
    pub fn rd5_p(&mut self) -> RD5_P_W<0> {
        RD5_P_W::new(self)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode)."]
    #[inline(always)]
    #[must_use]
    pub fn rd5_ns(&mut self) -> RD5_NS_W<1> {
        RD5_NS_W::new(self)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
    #[inline(always)]
    #[must_use]
    pub fn rd5_prio(&mut self) -> RD5_PRIO_W<8> {
        RD5_PRIO_W::new(self)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'."]
    #[inline(always)]
    #[must_use]
    pub fn rd5_pc_mask_15_to_1(&mut self) -> RD5_PC_MASK_15_TO_1_W<17> {
        RD5_PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VRAM Protection for read master with ID=5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd5_ctl](index.html) module"]
pub struct RD5_CTL_SPEC;
impl crate::RegisterSpec for RD5_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd5_ctl::R](R) reader structure"]
impl crate::Readable for RD5_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd5_ctl::W](W) writer structure"]
impl crate::Writable for RD5_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD5_CTL to value 0x0303"]
impl crate::Resettable for RD5_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0303;
}
