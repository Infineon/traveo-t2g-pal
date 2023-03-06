#[doc = "Register `SMX3FCTTABLE` reader"]
pub struct R(crate::R<SMX3FCTTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMX3FCTTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMX3FCTTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMX3FCTTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMX3FCTTABLE` writer"]
pub struct W(crate::W<SMX3FCTTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMX3FCTTABLE_SPEC>;
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
impl From<crate::W<SMX3FCTTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMX3FCTTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMXFCT3` reader - Sync mixer 3 function table"]
pub type SMXFCT3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SMXFCT3` writer - Sync mixer 3 function table"]
pub type SMXFCT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMX3FCTTABLE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Sync mixer 3 function table"]
    #[inline(always)]
    pub fn smxfct3(&self) -> SMXFCT3_R {
        SMXFCT3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sync mixer 3 function table"]
    #[inline(always)]
    #[must_use]
    pub fn smxfct3(&mut self) -> SMXFCT3_W<0> {
        SMXFCT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smx3fcttable](index.html) module"]
pub struct SMX3FCTTABLE_SPEC;
impl crate::RegisterSpec for SMX3FCTTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smx3fcttable::R](R) reader structure"]
impl crate::Readable for SMX3FCTTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smx3fcttable::W](W) writer structure"]
impl crate::Writable for SMX3FCTTABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMX3FCTTABLE to value 0"]
impl crate::Resettable for SMX3FCTTABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
