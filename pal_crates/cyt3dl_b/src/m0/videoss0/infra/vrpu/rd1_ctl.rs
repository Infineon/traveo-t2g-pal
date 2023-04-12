#[doc = "Register `RD1_CTL` reader"]
pub struct R(crate::R<RD1_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD1_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD1_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD1_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD1_CTL` writer"]
pub struct W(crate::W<RD1_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD1_CTL_SPEC>;
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
impl From<crate::W<RD1_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD1_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P` reader - Privileged setting ('0': user mode; '1': privileged mode)."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - Privileged setting ('0': user mode; '1': privileged mode)."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD1_CTL_SPEC, bool, O>;
#[doc = "Field `NS` reader - Security setting ('0': secure mode; '1': non-secure mode)."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `NS` writer - Security setting ('0': secure mode; '1': non-secure mode)."]
pub type NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD1_CTL_SPEC, bool, O>;
#[doc = "Field `PRIO` reader - See RD0_CTL.PRIO"]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - See RD0_CTL.PRIO"]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD1_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PC_MASK_0` reader - Protection context mask for protection context '0'. This field is a constant '0'."]
pub type PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `PC_MASK_15_TO_1` reader - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies CPUSS_PROT_MPU0 MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies GFX_MPU_RDx RD_CTL.PC\\[3:0\\])."]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies CPUSS_PROT_MPU0 MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies GFX_MPU_RDx RD_CTL.PC\\[3:0\\])."]
pub type PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RD1_CTL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode)."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode)."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - See RD0_CTL.PRIO"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Protection context mask for protection context '0'. This field is a constant '0'."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies CPUSS_PROT_MPU0 MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies GFX_MPU_RDx RD_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode)."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<0> {
        P_W::new(self)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode)."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<1> {
        NS_W::new(self)
    }
    #[doc = "Bits 8:9 - See RD0_CTL.PRIO"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<8> {
        PRIO_W::new(self)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': GFX_MPU_RDx RD_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies CPUSS_PROT_MPU0 MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies GFX_MPU_RDx RD_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W<17> {
        PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VRAM Protection for read master with ID=1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd1_ctl](index.html) module"]
pub struct RD1_CTL_SPEC;
impl crate::RegisterSpec for RD1_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd1_ctl::R](R) reader structure"]
impl crate::Readable for RD1_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd1_ctl::W](W) writer structure"]
impl crate::Writable for RD1_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD1_CTL to value 0x0302"]
impl crate::Resettable for RD1_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302;
}
