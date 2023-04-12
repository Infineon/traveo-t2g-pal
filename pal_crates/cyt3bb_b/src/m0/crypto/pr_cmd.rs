#[doc = "Register `PR_CMD` reader"]
pub struct R(crate::R<PR_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR_CMD` writer"]
pub struct W(crate::W<PR_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_CMD_SPEC>;
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
impl From<crate::W<PR_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pseudo random command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr_cmd](index.html) module"]
pub struct PR_CMD_SPEC;
impl crate::RegisterSpec for PR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr_cmd::R](R) reader structure"]
impl crate::Readable for PR_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr_cmd::W](W) writer structure"]
impl crate::Writable for PR_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR_CMD to value 0"]
impl crate::Resettable for PR_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
