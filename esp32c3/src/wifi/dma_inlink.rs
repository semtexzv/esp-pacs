#[doc = "Register `DMA_INLINK` reader"]
pub type R = crate::R<DMA_INLINK_SPEC>;
#[doc = "Register `DMA_INLINK` writer"]
pub type W = crate::W<DMA_INLINK_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_inlink::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_inlink::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INLINK_SPEC;
impl crate::RegisterSpec for DMA_INLINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_inlink::R`](R) reader structure"]
impl crate::Readable for DMA_INLINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_inlink::W`](W) writer structure"]
impl crate::Writable for DMA_INLINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INLINK to value 0"]
impl crate::Resettable for DMA_INLINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
