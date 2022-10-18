#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_14` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_14` writer"]
pub struct W(crate::W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>;
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
impl From<crate::W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L` reader - RTCSlow_1 memory low region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L` writer - RTCSlow_1 memory low region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_PIF_PMS_CONSTRAIN_14_SPEC, u8, u8, 3, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H` reader - RTCSlow_1 memory high region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H` writer - RTCSlow_1 memory high region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_PIF_PMS_CONSTRAIN_14_SPEC, u8, u8, 3, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L` reader - RTCSlow_1 memory low region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L` writer - RTCSlow_1 memory low region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_PIF_PMS_CONSTRAIN_14_SPEC, u8, u8, 3, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H` reader - RTCSlow_1 memory high region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H` writer - RTCSlow_1 memory high region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_PIF_PMS_CONSTRAIN_14_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - RTCSlow_1 memory low region permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_l(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - RTCSlow_1 memory high region permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_h(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - RTCSlow_1 memory low region permission in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_l(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - RTCSlow_1 memory high region permission in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_h(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTCSlow_1 memory low region permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_l(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_W<0> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_W::new(self)
    }
    #[doc = "Bits 3:5 - RTCSlow_1 memory high region permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_h(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_W<3> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_W::new(self)
    }
    #[doc = "Bits 6:8 - RTCSlow_1 memory low region permission in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_l(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_W<6> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_W::new(self)
    }
    #[doc = "Bits 9:11 - RTCSlow_1 memory high region permission in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_h(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_W<9> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core1 access peripherals permission configuration register 14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_constrain_14](index.html) module"]
pub struct CORE_1_PIF_PMS_CONSTRAIN_14_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_constrain_14::R](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_pif_pms_constrain_14::W](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_14 to value 0x0fff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}