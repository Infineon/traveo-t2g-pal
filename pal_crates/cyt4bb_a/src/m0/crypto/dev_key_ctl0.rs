#[doc = "Register `DEV_KEY_CTL0` reader"]
pub struct R(crate::R<DEV_KEY_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_KEY_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_KEY_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_KEY_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_KEY_CTL0` writer"]
pub struct W(crate::W<DEV_KEY_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_KEY_CTL0_SPEC>;
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
impl From<crate::W<DEV_KEY_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_KEY_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOWED` reader - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
pub type ALLOWED_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWED` writer - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
pub type ALLOWED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEV_KEY_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
    #[inline(always)]
    pub fn allowed(&self) -> ALLOWED_R {
        ALLOWED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
    #[inline(always)]
    #[must_use]
    pub fn allowed(&mut self) -> ALLOWED_W<0> {
        ALLOWED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device key control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_key_ctl0](index.html) module"]
pub struct DEV_KEY_CTL0_SPEC;
impl crate::RegisterSpec for DEV_KEY_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_key_ctl0::R](R) reader structure"]
impl crate::Readable for DEV_KEY_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_key_ctl0::W](W) writer structure"]
impl crate::Writable for DEV_KEY_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEV_KEY_CTL0 to value 0"]
impl crate::Resettable for DEV_KEY_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
