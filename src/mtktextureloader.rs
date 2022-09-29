use objr::bindings::*;
use crate::MTKTextureLoaderOption;
use metalr::MTLTexture;
use foundationr::NSURL;
objr::objc_class! {
    pub struct MTKTextureLoader {
        @class(MTKTextureLoader)
    }
}

objr::objc_selector_group! {
    trait Selectors {
        @selector("initWithDevice:")
        @selector("newTextureWithContentsOfURL:options:completionHandler:")
        @selector("newTextureWithData:options:error:")
    }
    impl Selectors for Sel {}
}
blocksr::once_escaping!(MTKTextureLoaderCallback(texture: *mut MTLTexture,error: *mut NSError) -> ());
unsafe impl Arguable for &mut MTKTextureLoaderCallback {}
#[allow(non_snake_case)]
impl MTKTextureLoader {
    pub fn initWithDevice(device: &metalr::MTLDevice, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let uninit = Self::class().alloc(pool);
            let raw = Self::perform(uninit, Sel::initWithDevice_(), pool, (device.assume_nonmut_perform(),));
            Self::assume_nonnil(raw).assume_retained().assume_mut()
        }
    }
    pub fn newTextureWithContentsOfURLOptionsCompletionHandler<C: FnOnce(Result<&mut MTLTexture,&mut NSError>) + Send + 'static>(&mut self, url: &NSURL, options: &foundationr::NSDictionary<MTKTextureLoaderOption,NSObject>,callback: C, pool: &ActiveAutoreleasePool) {
        unsafe {
            let mut block = MTKTextureLoaderCallback::new(|raw_texture,raw_error| {
                if !raw_texture.is_null() {
                    callback(Ok(&mut *raw_texture))
                }
                else {
                    debug_assert!(!raw_error.is_null());
                    callback(Err(&mut *raw_error))
                }
            });
            Self::perform_primitive(self, Sel::newTextureWithContentsOfURL_options_completionHandler(),pool,(url.assume_nonmut_perform(),options.assume_nonmut_perform(),&mut block))
        }
    }
    pub fn newTextureWithDataOptionsError<'p>(&mut self, data: &foundationr::NSData, options: &foundationr::NSDictionary<MTKTextureLoaderOption,NSObject>, pool: &'p ActiveAutoreleasePool) -> Result<StrongCell<metalr::MTLTexture>,AutoreleasedCell<'p, foundationr::NSError>> {
        unsafe {
            let mut error: *mut NSError = std::ptr::null_mut();
            let texture = Self::perform_result(self, Sel::newTextureWithData_options_error(),pool,(data.assume_nonmut_perform(),options.assume_nonmut_perform(),&mut error));
            texture.map(|texture|
                //although this API is declared nullable, presumably this only occurs if there is no error
                MTLTexture::assume_nonnil(texture).assume_retained())
        }
    }
}

#[test] fn test_kit() {
    autoreleasepool(|pool| {
        match metalr::MTLDevice::default() {
            Some(device) => {
                let mut loader = MTKTextureLoader::initWithDevice(&device, pool);
                let url = NSURL::initFileURLWithPath(objc_nsstring!("test_data/frog.png"),pool);
                let dictionary = foundationr::NSDictionary::withObjectsForKeys(&[],&[],pool);
                let (sender,receiver) = std::sync::mpsc::sync_channel(0);
                loader.newTextureWithContentsOfURLOptionsCompletionHandler(&url,&dictionary,move |result| {
                    assert!(result.is_ok());
                    sender.send(()).unwrap();
                }, pool);
                receiver.recv_timeout(std::time::Duration::new(1,0)).unwrap();
            }
            None => {
                println!("Assuming CI");
            }
        }
    })

}