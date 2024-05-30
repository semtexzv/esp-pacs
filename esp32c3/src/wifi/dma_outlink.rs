#[doc = "Register `DMA_OUTLINK` reader"]
pub type R = crate::R<DMA_OUTLINK_SPEC>;
#[doc = "Register `DMA_OUTLINK` writer"]
pub type W = crate::W<DMA_OUTLINK_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_outlink::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_outlink::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUTLINK_SPEC;
impl crate::RegisterSpec for DMA_OUTLINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_outlink::R`](R) reader structure"]
impl crate::Readable for DMA_OUTLINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_outlink::W`](W) writer structure"]
impl crate::Writable for DMA_OUTLINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_OUTLINK to value 0"]
impl crate::Resettable for DMA_OUTLINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
