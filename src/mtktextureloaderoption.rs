use objr::bindings::*;

objc_instance_newtype! {
    struct MTKTextureLoaderOption: NSString;
}
extern {
    pub static MTKTextureLoaderOptionAllocateMipmaps: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOptionGenerateMipmaps: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOptionSRGB: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOptionTextureUsage: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOptionTextureCPUCacheMode: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOptionTextureStorageMode: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOptionCubeLayout: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderCubeLayoutVertical: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOptionOrigin: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOriginTopLeft: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOriginBottomLeft: &'static MTKTextureLoaderOption;
    pub static MTKTextureLoaderOriginFlippedVertically: &'static MTKTextureLoaderOption;

}

impl MTKTextureLoaderOption{

}