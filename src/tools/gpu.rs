/// The GPU module manages communication with the GL backend.
/// * Prevent redundant texture creation
/// * Create DrawFont instances
///

#[derive(Copy, Clone)]
pub struct GPUTool {
    /// Stores the mapping of the font file name and the texture index
    textures: HashMap<String, usize>,
}

impl GPUTool {
    pub fn new() -> Self {
        GPUTool {
            textures: HashMap::new(),
        }
    }
}