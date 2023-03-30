use iced::widget::{Column, Row, Scrollable};
use iced::{window, Element, Sandbox, Settings};
use iced_native::widget::{button, text};
use iced_native::{row, Theme};
use raw_cpuid::CpuId;

use crate::levels::V1;

mod levels;

struct MicroArchLevel {
    cpu_v1_support: V1,
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
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let scan_button = button("Scan").on_press(Message::ClickedScan);
        let mut local_column = Column::new();

        local_column = local_column
            .push(text(format!("CMOV: {}", self.cpu_v1_support.supports_cmov)))
            .push(text(format!("CX8: {}", self.cpu_v1_support.supports_cx8)))
            .push(text(format!("FPU: {}", self.cpu_v1_support.supports_fpu)))
            .push(text(format!("FXSR: {}", self.cpu_v1_support.supports_fxsr)))
            .push(text(format!("MMX: {}", self.cpu_v1_support.supports_mmx)))
            .push(text(format!(
                "OSFXSR: {}",
                self.cpu_v1_support.supports_osfxsr
            )))
            .push(text(format!("SCE: {}", self.cpu_v1_support.supports_sce)))
            .push(text(format!("SSE: {}", self.cpu_v1_support.supports_sse)))
            .push(text(format!("SSE2: {}", self.cpu_v1_support.supports_sse2)));

        let local_scrollable = Scrollable::new(local_column);

        Column::new()
            .push(local_scrollable)
            .push(scan_button)
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
