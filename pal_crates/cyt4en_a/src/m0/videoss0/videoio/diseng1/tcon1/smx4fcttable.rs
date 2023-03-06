#[doc = "Register `SMX4FCTTABLE` reader"]
pub struct R(crate::R<SMX4FCTTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMX4FCTTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMX4FCTTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMX4FCTTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMX4FCTTABLE` writer"]
pub struct W(crate::W<SMX4FCTTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMX4FCTTABLE_SPEC>;
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
impl From<crate::W<SMX4FCTTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMX4FCTTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMXFCT4` reader - Sync mixer 4 function table"]
pub type SMXFCT4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SMXFCT4` writer - Sync mixer 4 function table"]
pub type SMXFCT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMX4FCTTABLE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Sync mixer 4 function table"]
    #[inline(always)]
    pub fn smxfct4(&self) -> SMXFCT4_R {
        SMXFCT4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sync mixer 4 function table"]
    #[inline(always)]
    #[must_use]
    pub fn smxfct4(&mut self) -> SMXFCT4_W<0> {
        SMXFCT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The sync mixer output is the result of the function table a=s4*2**4+s3*2**3+s2*2**2+s1*2**1+s0*2**0 whereby a is bit number and s result of sync mixer input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smx4fcttable](index.html) module"]
pub struct SMX4FCTTABLE_SPEC;
impl crate::RegisterSpec for SMX4FCTTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smx4fcttable::R](R) reader structure"]
impl crate::Readable for SMX4FCTTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smx4fcttable::W](W) writer structure"]
impl crate::Writable for SMX4FCTTABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMX4FCTTABLE to value 0"]
impl crate::Resettable for SMX4FCTTABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
