pub struct V1 {
    pub supports_cmov: String,
    pub supports_cx8: String,
    pub supports_fpu: String,
    pub supports_fxsr: String,
    pub supports_mmx: String,
    pub supports_osfxsr: String,
    pub supports_sce: String,
    pub supports_sse: String,
    pub supports_sse2: String,
}

impl V1 {
    pub fn new() -> V1 {
        V1 {
            supports_cmov: String::from(""),
            supports_cx8: String::from(""),
            supports_fpu: String::from(""),
            supports_fxsr: String::from(""),
            supports_mmx: String::from(""),
            supports_osfxsr: String::from(""),
            supports_sce: String::from(""),
            supports_sse: String::from(""),
            supports_sse2: String::from(""),
        }
    }
}

pub struct V2 {
    pub supports_cmpxchg16b: String,
    pub supports_lahf_sahf: String,
    pub supports_popcnt: String,
    pub supports_sse3: String,
    pub supports_sse4_1: String,
    pub supports_sse4_2: String,
    pub supports_ssse3: String,
}

impl V2 {
    pub fn new() -> V2 {
        V2 {
            supports_cmpxchg16b: String::from(""),
            supports_lahf_sahf: String::from(""),
            supports_popcnt: String::from(""),
            supports_sse3: String::from(""),
            supports_sse4_1: String::from(""),
            supports_sse4_2: String::from(""),
            supports_ssse3: String::from(""),
        }
    }
}

pub struct V3 {
    pub supports_avx: String,
    pub supports_avx2: String,
    pub supports_bmi1: String,
    pub supports_bmi2: String,
    pub supports_f16c: String,
    pub supports_fma: String,
    pub supports_lzcnt: String,
    pub supports_movbe: String,
    pub supports_osxsave: String,
}

impl V3 {
    pub fn new() -> V3 {
        V3 {
            supports_avx: String::from(""),
            supports_avx2: String::from(""),
            supports_bmi1: String::from(""),
            supports_bmi2: String::from(""),
            supports_f16c: String::from(""),
            supports_fma: String::from(""),
            supports_lzcnt: String::from(""),
            supports_movbe: String::from(""),
            supports_osxsave: String::from(""),
        }
    }
}

pub struct V4 {
    pub supports_avx512f: String,
    pub supports_avx512bw: String,
    pub supports_avx512cd: String,
    pub supports_avx512dq: String,
    pub supports_avx512vl: String,
}

impl V4 {
    pub fn new() -> V4 {
        V4 {
            supports_avx512f: String::from(""),
            supports_avx512bw: String::from(""),
            supports_avx512cd: String::from(""),
            supports_avx512dq: String::from(""),
            supports_avx512vl: String::from(""),
        }
    }
}
