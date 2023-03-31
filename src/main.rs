use iced::widget::{Column, Row, Scrollable};
use iced::Length::Fixed;
use iced::{window, Alignment, Element, Length, Sandbox, Settings};
use iced_aw::native::Card;
use iced_native::widget::{button, container, horizontal_rule, text, Container, Rule};
use iced_native::Theme;
use raw_cpuid::CpuId;

use crate::levels::{V1, V2, V3, V4};

mod levels;
mod style;

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
        let v2_cmpxchg16b = format!("CMPXCHG16B: {}", self.cpu_v2_support.supports_cmpxchg16b);
        let v2_lahf_sahf = format!("LAHF-SAHF: {}", self.cpu_v2_support.supports_lahf_sahf);
        let v2_popcnt = format!("POPCNT: {}", self.cpu_v2_support.supports_popcnt);
        let v2_sse3 = format!("SSE3: {}", self.cpu_v2_support.supports_sse3);
        let v2_sse4_1 = format!("SSE4_1: {}", self.cpu_v2_support.supports_sse4_1);
        let v2_sse4_2 = format!("SSE4_2: {}", self.cpu_v2_support.supports_sse4_2);
        let v2_ssse3 = format!("SSE4_2: {}", self.cpu_v2_support.supports_ssse3);
        let v3_avx = format!("AVX: {}", self.cpu_v3_support.supports_avx);
        let v3_avx2 = format!("AVX2: {}", self.cpu_v3_support.supports_avx2);
        let v3_bmi1 = format!("BMI1: {}", self.cpu_v3_support.supports_bmi1);
        let v3_bmi2 = format!("BMI2: {}", self.cpu_v3_support.supports_bmi2);
        let v3_f16c = format!("F16C: {}", self.cpu_v3_support.supports_f16c);
        let v3_fma = format!("FMA: {}", self.cpu_v3_support.supports_fma);
        let v3_lzcnt = format!("LZCNT: {}", self.cpu_v3_support.supports_lzcnt);
        let v3_movbe = format!("MOVBE: {}", self.cpu_v3_support.supports_movbe);
        let v3_osxsave = format!("OSXSAVE: {}", self.cpu_v3_support.supports_osxsave);
        let v4_avx512f = format!("AVX512F: {}", self.cpu_v4_support.supports_avx512f);
        let v4_avx512bw = format!("AVX512BW: {}", self.cpu_v4_support.supports_avx512bw);
        let v4_avx512cd = format!("AVX512CD: {}", self.cpu_v4_support.supports_avx512cd);
        let v4_avx512dq = format!("AVX512DQ: {}", self.cpu_v4_support.supports_avx512dq);
        let v4_avx512vl = format!("AVX512VL: {}", self.cpu_v4_support.supports_avx512vl);

        let v1_text_column = Column::new()
            .push(text(v2_cmpxchg16b))
            .push(text(v2_lahf_sahf))
            .push(text(v2_popcnt))
            .push(text(v2_sse3))
            .push(text(v2_sse4_1))
            .push(text(v2_sse4_2))
            .push(text(v2_ssse3))
            .spacing(5.0)
            .align_items(Alignment::Fill)
            .height(Length::Shrink);

        let v1_container = Container::new(
            Column::new()
                .push(text("V1 SUPPORT"))
                .push(horizontal_rule(15.0))
                .push(v1_text_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Center),
        )
        .style(style::CustomContainer)
        .height(Length::Fixed(275.0))
        .width(Fixed(200.0));

        let v1_text_column = Column::new()
            .push(text(v1_cmov))
            .push(text(v1_cx8))
            .push(text(v1_fpu))
            .push(text(v1_fxsr))
            .push(text(v1_mmx))
            .push(text(v1_osfxsr))
            .push(text(v1_osce))
            .push(text(v1_sse))
            .push(text(v1_sse2))
            .spacing(5.0)
            .align_items(Alignment::Fill)
            .height(Length::Shrink);

        let v2_container = Container::new(
            Column::new()
                .push(text("V2 SUPPORT"))
                .push(horizontal_rule(15.0))
                .push(v1_text_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Center),
        )
        .style(style::CustomContainer)
        .height(Length::Fixed(275.0))
        .width(Fixed(200.0));

        let v3_text_column = Column::new()
            .push(text(v3_avx))
            .push(text(v3_avx2))
            .push(text(v3_bmi1))
            .push(text(v3_bmi2))
            .push(text(v3_f16c))
            .push(text(v3_fma))
            .push(text(v3_lzcnt))
            .push(text(v3_movbe))
            .push(text(v3_osxsave))
            .spacing(5.0)
            .align_items(Alignment::Fill)
            .height(Length::Shrink);

        let v3_container = Container::new(
            Column::new()
                .push(text("V3 SUPPORT"))
                .push(horizontal_rule(15.0))
                .push(v3_text_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Center),
        )
        .style(style::CustomContainer)
        .height(Length::Fixed(275.0))
        .width(Fixed(200.0));

        let v4_text_column = Column::new()
            .push(text(v4_avx512f))
            .push(text(v4_avx512bw))
            .push(text(v4_avx512cd))
            .push(text(v4_avx512dq))
            .push(text(v4_avx512vl))
            .spacing(5.0)
            .align_items(Alignment::Fill)
            .height(Length::Fill);

        let v4_container = Container::new(
            Column::new()
                .push(text("V4 SUPPORT"))
                .push(horizontal_rule(15.0))
                .push(v4_text_column)
                .width(Length::Fill)
                .height(Length::Fixed(275.0))
                .align_items(Alignment::Center),
        )
        .style(style::CustomContainer)
        .height(Length::Fixed(275.0))
        .width(Fixed(200.0));

        let card_row = Row::new()
            .push(v1_container)
            .push(v2_container)
            .push(v3_container)
            .push(v4_container)
            .spacing(10.0);

        let header_container = Container::new(Row::new().push(text("HEADER")))
            .height(100.0)
            .width(Length::Fixed(830.0))
            .style(style::CustomContainer);

        local_column = local_column
            .push(header_container)
            .push(card_row)
            .push(scan_button)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10.0);

        Container::new(local_column)
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
            size: (850, 500),
            resizable: false,
            decorations: true,

            ..Default::default()
        },
        ..Default::default()
    };
    MicroArchLevel::run(settings).unwrap();
}
