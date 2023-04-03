use iced::widget::{Column, Row};
use iced::Length::Fixed;
use iced::{window, Alignment, Element, Length, Sandbox, Settings};
use iced_native::widget::{button, container, horizontal_rule, text, Button, Container, Rule};
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
    cpu_name: String,
    cpu_vendor_info: String,
    cpu_model_id: String,
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
            cpu_name: String::from("Press Scan"),
            cpu_vendor_info: String::from("Press Scan"),
            cpu_model_id: String::from("Press Scan"),
        }
    }

    fn title(&self) -> String {
        String::from("Easy Microarchitecture Level")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ClickedScan => {
                if let Some(feature) = self.cpuid.get_feature_info() {
                    self.cpu_v1_support.supports_cmov = if feature.has_cmov() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v1_support.supports_cx8 = if feature.has_cmpxchg8b() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v1_support.supports_fpu = if feature.has_fpu() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v1_support.supports_fxsr = if feature.has_fxsave_fxstor() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v1_support.supports_mmx = if feature.has_mmx() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v1_support.supports_osfxsr = if feature.has_fxsave_fxstor() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v1_support.supports_sse = if feature.has_sse() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v1_support.supports_sse2 = if feature.has_sse2() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v2_support.supports_cmpxchg16b = if feature.has_cmpxchg16b() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v2_support.supports_popcnt = if feature.has_popcnt() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v2_support.supports_sse3 = if feature.has_sse3() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v2_support.supports_sse4_1 = if feature.has_sse41() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v2_support.supports_sse4_2 = if feature.has_sse42() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v2_support.supports_ssse3 = if feature.has_ssse3() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v3_support.supports_avx = if feature.has_avx() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v3_support.supports_f16c = if feature.has_f16c() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v3_support.supports_fma = if feature.has_fma() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v3_support.supports_movbe = if feature.has_movbe() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };
                } else {
                    self.cpu_v1_support.supports_cmov = String::from("Error");
                    self.cpu_v1_support.supports_cx8 = String::from("Error");
                    self.cpu_v1_support.supports_fpu = String::from("Error");
                    self.cpu_v1_support.supports_fxsr = String::from("Error");
                    self.cpu_v1_support.supports_mmx = String::from("Error");
                    self.cpu_v1_support.supports_osfxsr = String::from("Error");
                    self.cpu_v1_support.supports_sce = String::from("Error");
                    self.cpu_v1_support.supports_sse = String::from("Error");
                    self.cpu_v1_support.supports_sse2 = String::from("Error");
                    self.cpu_v2_support.supports_cmpxchg16b = String::from("Error");
                    self.cpu_v2_support.supports_lahf_sahf = String::from("Error");
                    self.cpu_v2_support.supports_popcnt = String::from("Error");
                    self.cpu_v2_support.supports_sse3 = String::from("Error");
                    self.cpu_v2_support.supports_sse4_1 = String::from("Error");
                    self.cpu_v2_support.supports_sse4_2 = String::from("Error");
                    self.cpu_v2_support.supports_ssse3 = String::from("Error");
                    self.cpu_v3_support.supports_avx = String::from("Error");
                    self.cpu_v3_support.supports_avx2 = String::from("Error");
                    self.cpu_v3_support.supports_bmi1 = String::from("Error");
                    self.cpu_v3_support.supports_bmi2 = String::from("Error");
                    self.cpu_v3_support.supports_f16c = String::from("Error");
                    self.cpu_v3_support.supports_fma = String::from("Error");
                    self.cpu_v3_support.supports_lzcnt = String::from("Error");
                    self.cpu_v3_support.supports_movbe = String::from("Error");
                    self.cpu_v3_support.supports_osxsave = String::from("Error");
                    self.cpu_v4_support.supports_avx512f = String::from("Error");
                    self.cpu_v4_support.supports_avx512bw = String::from("Error");
                    self.cpu_v4_support.supports_avx512cd = String::from("Error");
                    self.cpu_v4_support.supports_avx512dq = String::from("Error");
                    self.cpu_v4_support.supports_avx512vl = String::from("Error");
                }

                if let Some(feature) = self.cpuid.get_extended_feature_info() {
                    self.cpu_v3_support.supports_avx2 = if feature.has_avx2() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v3_support.supports_bmi1 = if feature.has_bmi1() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v3_support.supports_bmi2 = if feature.has_bmi2() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v4_support.supports_avx512f = if feature.has_avx512f() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v4_support.supports_avx512bw = if feature.has_avx512bw() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v4_support.supports_avx512cd = if feature.has_avx512cd() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v4_support.supports_avx512dq = if feature.has_avx512dq() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v4_support.supports_avx512vl = if feature.has_avx512vl() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };
                } else {
                    self.cpu_v3_support.supports_avx2 = String::from("Error");
                    self.cpu_v3_support.supports_bmi1 = String::from("Error");
                    self.cpu_v3_support.supports_bmi2 = String::from("Error");
                    self.cpu_v4_support.supports_avx512f = String::from("Error");
                    self.cpu_v4_support.supports_avx512bw = String::from("Error");
                    self.cpu_v4_support.supports_avx512cd = String::from("Error");
                    self.cpu_v4_support.supports_avx512dq = String::from("Error");
                    self.cpu_v4_support.supports_avx512vl = String::from("Error");
                }

                if let Some(feature) = self.cpuid.get_extended_processor_and_feature_identifiers() {
                    self.cpu_v1_support.supports_sce = if feature.has_syscall_sysret() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v2_support.supports_lahf_sahf = if feature.has_lahf_sahf() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };

                    self.cpu_v3_support.supports_lzcnt = if feature.has_lzcnt() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };
                } else {
                    self.cpu_v1_support.supports_sce = String::from("Error");
                    self.cpu_v2_support.supports_lahf_sahf = String::from("Error");
                    self.cpu_v3_support.supports_lzcnt = String::from("Error");
                }

                if let Some(feature) = self.cpuid.get_extended_state_info() {
                    self.cpu_v3_support.supports_osxsave = if feature.has_xgetbv() {
                        String::from("Yes")
                    } else {
                        String::from("No")
                    };
                } else {
                    self.cpu_v3_support.supports_osxsave = String::from("Error");
                }

                self.cpu_name = self
                    .cpuid
                    .get_processor_brand_string()
                    .unwrap()
                    .as_str()
                    .to_string();
                //
                self.cpu_vendor_info = self.cpuid.get_vendor_info().unwrap().as_str().to_string();
                self.cpu_model_id = self
                    .cpuid
                    .get_feature_info()
                    .unwrap()
                    .family_id()
                    .to_string();
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let scan_button = Button::new(text("Scan CPU").size(30.0)).on_press(Message::ClickedScan);
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
        //

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
                .push(container(text("V1 SUPPORT")).padding(5.0))
                .push(horizontal_rule(10.0))
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
                .push(container(text("V2 SUPPORT")).padding(5.0))
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
                .push(container(text("V3 SUPPORT")).padding(5.0))
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
                .push(container(text("V4 SUPPORT")).padding(5.0))
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

        let header_container = Container::new(
            Column::new()
                .push(text(&self.cpu_vendor_info))
                .push(container(horizontal_rule(12.0)).width(100.0))
                .push(text(&self.cpu_name))
                .align_items(Alignment::Center),
        )
        .padding(10.0)
        .height(100.0)
        .width(Length::Fixed(830.0))
        .center_x()
        .center_y()
        .style(style::CustomContainer);

        let main_column = Column::new()
            .push(header_container)
            .push(card_row)
            .push(
                container(scan_button)
                    .center_x()
                    .center_y()
                    .height(Length::Fill)
                    .width(Length::Fill),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center);

        Container::new(main_column)
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
