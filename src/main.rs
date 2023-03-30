use iced::widget::{Column, Row, Scrollable};
use iced::{window, Element, Length, Sandbox, Settings};
use iced_aw::native::Card;
use iced_native::widget::{button, text, Container};
use iced_native::{row, Theme};
use iced_style::container;
use raw_cpuid::CpuId;

use crate::levels::{V1, V2, V3, V4};

mod levels;

struct MicroArchLevel {
    cpu_v1_support: V1,
    cpu_v2_support: V2,
    cpu_v3_support: V3,
    cpu_v4_support: V4,
    cpuid: CpuId,
}

#[derive(Debug, Clone)]
enum Message {
    ClickedScan,
}

impl Sandbox for MicroArchLevel {
    type Message = Message;

    fn new() -> Self {
        MicroArchLevel {
            cpu_v1_support: V1::new(),
            cpu_v2_support: V2::new(),
            cpu_v3_support: V3::new(),
            cpu_v4_support: V4::new(),
            cpuid: CpuId::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Easy Microarchitecture Level")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ClickedScan => {
                self.cpu_v1_support.supports_cmov =
                    self.cpuid.get_feature_info().unwrap().has_cmov();
                self.cpu_v1_support.supports_cx8 =
                    self.cpuid.get_feature_info().unwrap().has_cmpxchg8b();
                self.cpu_v1_support.supports_fpu = self.cpuid.get_feature_info().unwrap().has_fpu();
                self.cpu_v1_support.supports_fxsr =
                    self.cpuid.get_feature_info().unwrap().has_fxsave_fxstor();
                self.cpu_v1_support.supports_mmx = self.cpuid.get_feature_info().unwrap().has_mmx();
                self.cpu_v1_support.supports_osfxsr =
                    self.cpuid.get_feature_info().unwrap().has_fxsave_fxstor();
                self.cpu_v1_support.supports_sce = self
                    .cpuid
                    .get_extended_processor_and_feature_identifiers()
                    .unwrap()
                    .has_syscall_sysret();
                self.cpu_v1_support.supports_sse = self.cpuid.get_feature_info().unwrap().has_sse();
                self.cpu_v1_support.supports_sse2 =
                    self.cpuid.get_feature_info().unwrap().has_sse2();
                //
                self.cpu_v2_support.supports_cmpxchg16b =
                    self.cpuid.get_feature_info().unwrap().has_cmpxchg16b();
                self.cpu_v2_support.supports_lahf_sahf = self
                    .cpuid
                    .get_extended_processor_and_feature_identifiers()
                    .unwrap()
                    .has_lahf_sahf();
                self.cpu_v2_support.supports_popcnt =
                    self.cpuid.get_feature_info().unwrap().has_popcnt();
                self.cpu_v2_support.supports_sse3 =
                    self.cpuid.get_feature_info().unwrap().has_sse3();
                self.cpu_v2_support.supports_sse4_1 =
                    self.cpuid.get_feature_info().unwrap().has_sse41();
                self.cpu_v2_support.supports_sse4_2 =
                    self.cpuid.get_feature_info().unwrap().has_sse42();
                self.cpu_v2_support.supports_ssse3 =
                    self.cpuid.get_feature_info().unwrap().has_ssse3();
                //
                self.cpu_v3_support.supports_avx = self.cpuid.get_feature_info().unwrap().has_avx();
                self.cpu_v3_support.supports_avx2 =
                    self.cpuid.get_extended_feature_info().unwrap().has_avx2();
                self.cpu_v3_support.supports_bmi1 =
                    self.cpuid.get_extended_feature_info().unwrap().has_bmi1();
                self.cpu_v3_support.supports_bmi2 =
                    self.cpuid.get_extended_feature_info().unwrap().has_bmi2();
                self.cpu_v3_support.supports_f16c =
                    self.cpuid.get_feature_info().unwrap().has_f16c();
                self.cpu_v3_support.supports_fma = self.cpuid.get_feature_info().unwrap().has_fma();
                self.cpu_v3_support.supports_lzcnt = self
                    .cpuid
                    .get_extended_processor_and_feature_identifiers()
                    .unwrap()
                    .has_lzcnt();
                self.cpu_v3_support.supports_movbe =
                    self.cpuid.get_feature_info().unwrap().has_movbe();
                self.cpu_v3_support.supports_osxsave =
                    self.cpuid.get_extended_state_info().unwrap().has_xgetbv();
                //
                self.cpu_v4_support.supports_avx512f = self
                    .cpuid
                    .get_extended_feature_info()
                    .unwrap()
                    .has_avx512f();
                self.cpu_v4_support.supports_avx512bw = self
                    .cpuid
                    .get_extended_feature_info()
                    .unwrap()
                    .has_avx512bw();
                self.cpu_v4_support.supports_avx512cd = self
                    .cpuid
                    .get_extended_feature_info()
                    .unwrap()
                    .has_avx512cd();
                self.cpu_v4_support.supports_avx512dq = self
                    .cpuid
                    .get_extended_feature_info()
                    .unwrap()
                    .has_avx512dq();
                self.cpu_v4_support.supports_avx512vl = self
                    .cpuid
                    .get_extended_feature_info()
                    .unwrap()
                    .has_avx512vl();
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let scan_button = button("Scan").on_press(Message::ClickedScan);
        let mut local_column = Column::new();
        let v1_cmov = format!("CMOV: {}", self.cpu_v1_support.supports_cmov);
        let v1_cx8 = format!("CX8: {}", self.cpu_v1_support.supports_cx8);
        let v1_fpu = format!("FPU: {}", self.cpu_v1_support.supports_fpu);
        let v1_fxsr = format!("FXSR: {}", self.cpu_v1_support.supports_fxsr);
        let v1_mmx = format!("MMX: {}", self.cpu_v1_support.supports_mmx);
        let v1_osfxsr = format!("OSFXSR: {}", self.cpu_v1_support.supports_osfxsr);
        let v1_osce = format!("SCE: {}", self.cpu_v1_support.supports_sce);
        let v1_sse = format!("SSE: {}", self.cpu_v1_support.supports_sse);
        let v1_sse2 = format!("SSE2: {}", self.cpu_v1_support.supports_sse2);

        let v1_support_card = Card::new(
            "V1 SUPPORT",
            text(format!("{v1_cmov}\n{v1_cx8}\n{v1_fpu}\n{v1_fxsr}\n{v1_mmx}\n{v1_osfxsr}\n{v1_osce}\n{v1_sse}\n{v1_sse2}")),
        ).padding_body(10.0)
        .width(Length::Fill)
        .height(Length::Fill);

        local_column = local_column
            .push(v1_support_card)
            .push(scan_button)
            .spacing(10.0);

        let local_scrollable = Scrollable::new(local_column);

        Container::new(local_scrollable)
            .center_x()
            .center_y()
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() {
    println!("Easy Microarchitecture Level");
    let settings = Settings {
        window: window::Settings {
            size: (800, 500),
            resizable: true,
            decorations: true,

            ..Default::default()
        },
        ..Default::default()
    };
    MicroArchLevel::run(settings).unwrap();
}
