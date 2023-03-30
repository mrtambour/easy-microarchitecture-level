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
pub struct V2 {
    pub supports_cmpxchg16b: bool,
    pub supports_lahf_sahf: bool,
    pub supports_popcnt: bool,
    pub supports_sse3: bool,
    pub supports_sse4_1: bool,
    pub supports_sse4_2: bool,
    pub supports_ssse3: bool,
}

impl V2 {
    pub fn new() -> V2 {
        V2 {
            supports_cmpxchg16b: false,
            supports_lahf_sahf: false,
            supports_popcnt: false,
            supports_sse3: false,
            supports_sse4_1: false,
            supports_sse4_2: false,
            supports_ssse3: false,
        }
    }
}
pub struct V3 {
    pub supports_avx: bool,
    pub supports_avx2: bool,
    pub supports_bmi1: bool,
    pub supports_bmi2: bool,
    pub supports_f16c: bool,
    pub supports_fma: bool,
    pub supports_lzcnt: bool,
    pub supports_movbe: bool,
    pub supports_osxsave: bool,
}

impl V3 {
    pub fn new() -> V3 {
        V3 {
            supports_avx: false,
            supports_avx2: false,
            supports_bmi1: false,
            supports_bmi2: false,
            supports_f16c: false,
            supports_fma: false,
            supports_lzcnt: false,
            supports_movbe: false,
            supports_osxsave: false,
        }
    }
}
pub struct V4 {
    pub supports_avx512f: bool,
    pub supports_avx512bw: bool,
    pub supports_avx512cd: bool,
    pub supports_avx512dq: bool,
    pub supports_avx512vl: bool,
}

impl V4 {
    pub fn new() -> V4 {
        V4 {
            supports_avx512f: false,
            supports_avx512bw: false,
            supports_avx512cd: false,
            supports_avx512dq: false,
            supports_avx512vl: false,
        }
    }
}
