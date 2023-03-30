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
struct V2 {
    supports_cmpxchg16b: bool,
    supports_lahf_sahf: bool,
    supports_popcnt: bool,
    supports_sse3_addsubpd: bool,
    supports_sse4_1: bool,
    supports_sse4_2: bool,
    supports_sse3_phaddd: bool,
}

impl V2 {
    pub fn new() -> V2 {
        V2 {
            supports_cmpxchg16b: false,
            supports_lahf_sahf: false,
            supports_popcnt: false,
            supports_sse3_addsubpd: false,
            supports_sse4_1: false,
            supports_sse4_2: false,
            supports_sse3_phaddd: false,
        }
    }
}
struct V3 {
    supports_avx: bool,
    supports_avx2: bool,
    supports_bmi1: bool,
    supports_bmi2: bool,
    supports_f16c: bool,
    supports_fma: bool,
    supports_lzcnt: bool,
    supports_movbe: bool,
    supports_osxsave: bool,
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
struct V4 {
    supports_avx512f: bool,
    supports_avx512bw: bool,
    supports_avx512cd: bool,
    supports_avx512dq: bool,
    supports_avx512vl: bool,
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
