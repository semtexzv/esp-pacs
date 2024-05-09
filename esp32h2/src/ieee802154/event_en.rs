#[doc = "Register `EVENT_EN` reader"]
pub type R = crate::R<EVENT_EN_SPEC>;
#[doc = "Register `EVENT_EN` writer"]
pub type W = crate::W<EVENT_EN_SPEC>;
#[doc = "Field `EVENT_EN` reader - "]
pub type EVENT_EN_R = crate::FieldReader<u16>;
#[doc = "Field `EVENT_EN` writer - "]
pub type EVENT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn event_en(&self) -> EVENT_EN_R {
        EVENT_EN_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENT_EN")
            .field("event_en", &format_args!("{}", self.event_en().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVENT_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn event_en(&mut self) -> EVENT_EN_W<EVENT_EN_SPEC> {
        EVENT_EN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVENT_EN_SPEC;
impl crate::RegisterSpec for EVENT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_en::R`](R) reader structure"]
impl crate::Readable for EVENT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`event_en::W`](W) writer structure"]
impl crate::Writable for EVENT_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENT_EN to value 0"]
impl crate::Resettable for EVENT_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
