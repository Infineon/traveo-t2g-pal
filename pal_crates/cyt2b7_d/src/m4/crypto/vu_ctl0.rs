#[doc = "Register `VU_CTL0` reader"]
pub struct R(crate::R<VU_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VU_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VU_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VU_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VU_CTL0` writer"]
pub struct W(crate::W<VU_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VU_CTL0_SPEC>;
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
impl From<crate::W<VU_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VU_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALWAYS_EXECUTE` reader - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
pub type ALWAYS_EXECUTE_R = crate::BitReader<bool>;
#[doc = "Field `ALWAYS_EXECUTE` writer - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
pub type ALWAYS_EXECUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VU_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
    #[inline(always)]
    pub fn always_execute(&self) -> ALWAYS_EXECUTE_R {
        ALWAYS_EXECUTE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
    #[inline(always)]
    #[must_use]
    pub fn always_execute(&mut self) -> ALWAYS_EXECUTE_W<0> {
        ALWAYS_EXECUTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vector unit control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vu_ctl0](index.html) module"]
pub struct VU_CTL0_SPEC;
impl crate::RegisterSpec for VU_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vu_ctl0::R](R) reader structure"]
impl crate::Readable for VU_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vu_ctl0::W](W) writer structure"]
impl crate::Writable for VU_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VU_CTL0 to value 0"]
impl crate::Resettable for VU_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
