use crate::GradleBuildpack;
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{Layer, LayerData, LayerResult, LayerResultBuilder};
use libcnb::Buildpack;
use std::path::Path;

pub struct GradleLayer;

impl Layer for GradleLayer {
    type Buildpack = GradleBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            launch: true,
            build: true,
            cache: true,
        }
    }

    fn create(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        _layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        LayerResultBuilder::new(GenericMetadata::default()).build()
    }

    fn update(
        &self,
        _context: &BuildContext<Self::Buildpack>,
        _layer_data: &LayerData<Self::Metadata>,
    ) -> Result<LayerResult<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        LayerResultBuilder::new(GenericMetadata::default()).build()
    }
}
