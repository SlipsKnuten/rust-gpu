use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This builds your learning-shader and makes it available as an environment variable
    SpirvBuilder::new(
        "../shader",  // Path to your shader crate (relative to runner directory)
        "spirv-unknown-vulkan1.2",  // Target Vulkan 1.2
    )
    .print_metadata(MetadataPrintout::Full)
    .build()?;

    Ok(())
}
