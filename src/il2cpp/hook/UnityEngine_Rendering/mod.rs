pub mod RenderingFeatures;
pub mod UniversalAdditionalCameraData;
pub mod UniversalRenderPipelineAsset;

pub fn init() {
    get_assembly_image_or_return!(image, "Unity.RenderPipelines.Universal.Runtime.dll");

    RenderingFeatures::init(image);
    UniversalAdditionalCameraData::init(image);
    UniversalRenderPipelineAsset::init(image);
}
