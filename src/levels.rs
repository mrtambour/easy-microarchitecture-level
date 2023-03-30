pub struct V1 {
    pub supports_cmov: bool,
    pub supports_cx8: bool,
    pub supports_fpu: bool,
    pub supports_fxsr: bool,
    pub supports_mmx: bool,
    pub supports_osfxsr: bool,
    pub supports_sce: bool,
    pub supports_sse: bool,
    pub supports_sse2: bool,
}

impl V1 {
    pub fn new() -> V1 {
        V1 {
            supports_cmov: false,
            supports_cx8: false,
            supports_fpu: false,
            supports_fxsr: false,
            supports_mmx: false,
            supports_osfxsr: false,
            supports_sce: false,
            supports_sse: false,
            supports_sse2: false,
        }
    }
}
struct V2 {}
struct V3 {}
struct V4 {}
