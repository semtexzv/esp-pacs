#[doc = "Register `ETM_TASK_P5_CFG` reader"]
pub struct R(crate::R<ETM_TASK_P5_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETM_TASK_P5_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETM_TASK_P5_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETM_TASK_P5_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETM_TASK_P5_CFG` writer"]
pub struct W(crate::W<ETM_TASK_P5_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETM_TASK_P5_CFG_SPEC>;
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
impl From<crate::W<ETM_TASK_P5_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETM_TASK_P5_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_TASK_GPIO20_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO20_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO20_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO20_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO20_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO20_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO20_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO20_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETM_TASK_GPIO21_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO21_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO21_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO21_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO21_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO21_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO21_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO21_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETM_TASK_GPIO22_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO22_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO22_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO22_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO22_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO22_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO22_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO22_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETM_TASK_GPIO23_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO23_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO23_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO23_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO23_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO23_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO23_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO23_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P5_CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio20_en(&self) -> ETM_TASK_GPIO20_EN_R {
        ETM_TASK_GPIO20_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio20_sel(&self) -> ETM_TASK_GPIO20_SEL_R {
        ETM_TASK_GPIO20_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio21_en(&self) -> ETM_TASK_GPIO21_EN_R {
        ETM_TASK_GPIO21_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio21_sel(&self) -> ETM_TASK_GPIO21_SEL_R {
        ETM_TASK_GPIO21_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio22_en(&self) -> ETM_TASK_GPIO22_EN_R {
        ETM_TASK_GPIO22_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio22_sel(&self) -> ETM_TASK_GPIO22_SEL_R {
        ETM_TASK_GPIO22_SEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio23_en(&self) -> ETM_TASK_GPIO23_EN_R {
        ETM_TASK_GPIO23_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio23_sel(&self) -> ETM_TASK_GPIO23_SEL_R {
        ETM_TASK_GPIO23_SEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio20_en(&mut self) -> ETM_TASK_GPIO20_EN_W<0> {
        ETM_TASK_GPIO20_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio20_sel(&mut self) -> ETM_TASK_GPIO20_SEL_W<1> {
        ETM_TASK_GPIO20_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio21_en(&mut self) -> ETM_TASK_GPIO21_EN_W<8> {
        ETM_TASK_GPIO21_EN_W::new(self)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio21_sel(&mut self) -> ETM_TASK_GPIO21_SEL_W<9> {
        ETM_TASK_GPIO21_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio22_en(&mut self) -> ETM_TASK_GPIO22_EN_W<16> {
        ETM_TASK_GPIO22_EN_W::new(self)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio22_sel(&mut self) -> ETM_TASK_GPIO22_SEL_W<17> {
        ETM_TASK_GPIO22_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio23_en(&mut self) -> ETM_TASK_GPIO23_EN_W<24> {
        ETM_TASK_GPIO23_EN_W::new(self)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio23_sel(&mut self) -> ETM_TASK_GPIO23_SEL_W<25> {
        ETM_TASK_GPIO23_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etm_task_p5_cfg](index.html) module"]
pub struct ETM_TASK_P5_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P5_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etm_task_p5_cfg::R](R) reader structure"]
impl crate::Readable for ETM_TASK_P5_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etm_task_p5_cfg::W](W) writer structure"]
impl crate::Writable for ETM_TASK_P5_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_TASK_P5_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P5_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}