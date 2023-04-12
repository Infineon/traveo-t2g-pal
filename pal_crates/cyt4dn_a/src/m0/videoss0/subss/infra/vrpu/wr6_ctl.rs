#[doc = "Register `WR6_CTL` reader"]
pub struct R(crate::R<WR6_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR6_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR6_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR6_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR6_CTL` writer"]
pub struct W(crate::W<WR6_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR6_CTL_SPEC>;
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
impl From<crate::W<WR6_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR6_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR6_P` reader - Privileged setting ('0': user mode; '1': privileged mode)."]
pub type WR6_P_R = crate::BitReader<bool>;
#[doc = "Field `WR6_P` writer - Privileged setting ('0': user mode; '1': privileged mode)."]
pub type WR6_P_W<'a, const O: u8> = crate::BitWriter<'a, u32, WR6_CTL_SPEC, bool, O>;
#[doc = "Field `WR6_NS` reader - Security setting ('0': secure mode; '1': non-secure mode)."]
pub type WR6_NS_R = crate::BitReader<bool>;
#[doc = "Field `WR6_NS` writer - Security setting ('0': secure mode; '1': non-secure mode)."]
pub type WR6_NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WR6_CTL_SPEC, bool, O>;
#[doc = "Field `WR6_PRIO` reader - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
pub type WR6_PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR6_PRIO` writer - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
pub type WR6_PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR6_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WR6_PC_MASK_0` reader - Protection context mask for protection context '0'. This field is a constant '0'."]
pub type WR6_PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `WR6_PC_MASK_15_TO_1` reader - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'."]
pub type WR6_PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WR6_PC_MASK_15_TO_1` writer - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'."]
pub type WR6_PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WR6_CTL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode)."]
    #[inline(always)]
    pub fn wr6_p(&self) -> WR6_P_R {
        WR6_P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode)."]
    #[inline(always)]
    pub fn wr6_ns(&self) -> WR6_NS_R {
        WR6_NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
    #[inline(always)]
    pub fn wr6_prio(&self) -> WR6_PRIO_R {
        WR6_PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Protection context mask for protection context '0'. This field is a constant '0'."]
    #[inline(always)]
    pub fn wr6_pc_mask_0(&self) -> WR6_PC_MASK_0_R {
        WR6_PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
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
    pub fn wr6_pc_mask_15_to_1(&self) -> WR6_PC_MASK_15_TO_1_R {
        WR6_PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode)."]
    #[inline(always)]
    #[must_use]
    pub fn wr6_p(&mut self) -> WR6_P_W<0> {
        WR6_P_W::new(self)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode)."]
    #[inline(always)]
    #[must_use]
    pub fn wr6_ns(&mut self) -> WR6_NS_W<1> {
        WR6_NS_W::new(self)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
    #[inline(always)]
    #[must_use]
    pub fn wr6_prio(&mut self) -> WR6_PRIO_W<8> {
        WR6_PRIO_W::new(self)
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
    pub fn wr6_pc_mask_15_to_1(&mut self) -> WR6_PC_MASK_15_TO_1_W<17> {
        WR6_PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VRAM Protection for write master with ID=6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr6_ctl](index.html) module"]
pub struct WR6_CTL_SPEC;
impl crate::RegisterSpec for WR6_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr6_ctl::R](R) reader structure"]
impl crate::Readable for WR6_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr6_ctl::W](W) writer structure"]
impl crate::Writable for WR6_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR6_CTL to value 0x0303"]
impl crate::Resettable for WR6_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0303;
}
