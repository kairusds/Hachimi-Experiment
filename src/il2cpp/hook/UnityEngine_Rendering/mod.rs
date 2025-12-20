pub mod UniversalAdditionalCameraData;

pub fn init() {
    get_assembly_image_or_return!(image, "Unity.RenderPipelines.Universal.Runtime.dll");

    UniversalAdditionalCameraData::init(image);
}
