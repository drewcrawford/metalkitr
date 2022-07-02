use objr::bindings::*;
objr::objc_class! {
    pub struct MTKTextureLoader {
        @class(MTKTextureLoader)
    }
}

objr::objc_selector_group! {
    trait Selectors {
        @selector("initWithDevice:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl MTKTextureLoader {
    pub fn initWithDevice(device: &metalr::MTLDevice, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let raw = Class::<Self>::perform(Self::class().assume_nonmut_perform(), Sel::initWithDevice_(), pool, (device.assume_nonmut_perform(),));
            Self::assume_nonnil(raw).assume_retained().assume_mut()
        }
    }
}

#[test] fn test_kit() {
    autoreleasepool(|pool| {
        match metalr::MTLDevice::default() {
            Some(device) => {
                let loader = MTKTextureLoader::initWithDevice(&device, pool);
            }
            None => {
                println!("Assuming CI");
            }
        }
    })

}