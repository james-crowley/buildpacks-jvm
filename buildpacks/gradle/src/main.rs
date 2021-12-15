mod gradle_layer;

use crate::gradle_layer::GradleLayer;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::layer_name;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::{buildpack_main, Buildpack};
use libherokubuildpack::{log_header, log_warning};
use std::path::Path;
use std::process::Command;

pub struct GradleBuildpack;

#[derive(Debug)]
pub enum GradleBuildpackError {}

impl Buildpack for GradleBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = GradleBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        let supported_file_names = vec!["gradlew", "settings.gradle", "build.gradle"];

        let app_has_supported_file = supported_file_names
            .iter()
            .find(|file| context.app_dir.join(&file).exists())
            .is_some();

        if app_has_supported_file {
            DetectResultBuilder::pass()
                .build_plan(BuildPlanBuilder::new().requires("jdk").build())
                .build()
        } else {
            DetectResultBuilder::fail().build()
        }
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        if !context.app_dir.join("gradlew").exists() {
            log_header("Installing Gradle Wrapper");
            log_warning(
                "Your application does not have it's own gradlew file",
                "We'll install one for you, but this is a deprecated feature and in the future may not be supported."
            );

            install_gradle_wrapper(&context.app_dir)?;
        }

        let gradle_layer = context.handle_layer(layer_name!("gradle_home"), GradleLayer)?;

        let gradle_task = "stage";

        let mut buildpack_gradle_opts = vec![];
        buildpack_gradle_opts.push("-Dorg.gradle.daemon=false");
        buildpack_gradle_opts.push("-Dorg.gradle.internal.launcher.welcomeMessageEnabled=false");

        Command::new(context.app_dir.join("gradlew"))
            .current_dir(&context.app_dir)
            .args(&[
                "--gradle-user-home",
                gradle_layer.path.to_str().unwrap(),
                gradle_task,
            ])
            .spawn()
            .unwrap();

        BuildResultBuilder::new().build()
    }
}

buildpack_main!(GradleBuildpack);

impl From<GradleBuildpackError> for libcnb::Error<GradleBuildpackError> {
    fn from(buildpack_error: GradleBuildpackError) -> Self {
        libcnb::Error::BuildpackError(buildpack_error)
    }
}

fn install_gradle_wrapper(app_dir: impl AsRef<Path>) -> Result<(), GradleBuildpackError> {
    Ok(())
}
