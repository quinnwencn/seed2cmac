#![windows_subsystem = "windows"]

use iced::widget::{button, column, container, pick_list, row, text, text_input, horizontal_space, vertical_space, stack, opaque, mouse_area, center};
use iced::{Color, Element, Font, Length, Settings, window, Padding, alignment};
use once_cell::sync::Lazy;
use image::GenericImageView;
use std::error::Error;
use arboard;

mod util;
mod crypto;

// Import necessary utilities for mask calculation
use util::{hex_string_to_bytes, xor_bytes};

include!(concat!(env!("OUT_DIR"), "/generated_ecu_mask.rs"));

static HIGHLIGHT_COLOR: Lazy<Color> = Lazy::new(|| Color::from_rgb8(0, 191, 255)); // Deep Sky Blue

// EcuType and SecurityLevel enums are dynamically generated from ecu_mask.txt in build.rs

#[derive(Debug)]
struct Seed2Cmac {
    ecu_type: Option<EcuType>,
    security_level: Option<SecurityLevel>,
    seed_input: String,
    key_input: String,
    key_output: String,
    error_message: Option<String>,
    show_error: bool,
}

impl Default for Seed2Cmac {
    fn default() -> Self {
        // Get the first ECU type from the generated list
        // all_ecu_types().first() returns an Option<&EcuType>, so we map it to an Option<EcuType>
        let first_ecu = all_ecu_types().first().copied();
        
        // Get the first security level from the generated list
        let first_security_level = all_security_levels().first().copied();
        
        Self {
            ecu_type: Some(first_ecu.unwrap_or_else(|| panic!("No ECU types available"))),
            security_level: Some(first_security_level.unwrap_or_else(|| panic!("No security levels available"))),
            seed_input: String::new(),
            key_input: String::new(),
            key_output: String::new(),
            error_message: None,
            show_error: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EcuTypeSelected(EcuType),
    SecurityLevelSelected(SecurityLevel),
    SeedInputChanged(String),
    KeyInputChanged(String),
    KeyOutputChanged(String),
    Calculate,
    Clear,
    DismissError,
    CopyToClipboard,
    ClipboardError(String),
}

impl Seed2Cmac {
    fn title(&self) -> String {
        String::from("Seed2Cmac-v2.1")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::EcuTypeSelected(ecu_type) => {
                self.ecu_type = Some(ecu_type);
                self.error_message = None;
                self.show_error = false;
            }
            Message::SecurityLevelSelected(security_level) => {
                self.security_level = Some(security_level);
                self.error_message = None;
                self.show_error = false;
            }
            Message::SeedInputChanged(input) => {
                self.seed_input = input;
                self.error_message = None;
                self.show_error = false;
            }
            Message::KeyInputChanged(input) => {
                self.key_input = input;
                self.error_message = None;
                self.show_error = false;
            }
            Message::KeyOutputChanged(_) => {}
            Message::Calculate => {
                self.error_message = None;
                self.show_error = false;
                
                if let (Some(ecu), Some(level)) = (self.ecu_type, self.security_level) {
                    match self.calculate_cmac_key(ecu, level) {
                        Ok(cmac_hex) => {
                            self.key_output = cmac_hex;
                        },
                        Err(err) => {
                            self.error_message = Some(err.to_string());
                            self.show_error = true;
                        }
                    }
                }
            }
            Message::Clear => {
                self.seed_input = String::new();
                self.key_input = String::new();
                self.key_output = String::new();
                self.error_message = None;
                self.show_error = false;
            }
            Message::DismissError => {
                self.show_error = false;
            }
            Message::CopyToClipboard => {
                if !self.key_output.is_empty() {
                    match arboard::Clipboard::new() {
                        Ok(mut clipboard) => {
                            match clipboard.set_text(self.key_output.clone()) {
                                Ok(_) => {},
                                Err(err) => {
                                    self.error_message = Some(format!("复制到剪贴板失败: {}", err));
                                    self.show_error = true;
                                }
                            }
                        },
                        Err(err) => {
                            self.error_message = Some(format!("无法访问剪贴板: {}", err));
                            self.show_error = true;
                        }
                    }
                }
            }
            Message::ClipboardError(error) => {
                self.error_message = Some(error);
                self.show_error = true;
            }
        }
    }
    
    /// Calculate CMAC key using the seed, key, ECU type, and security level
    fn calculate_cmac_key(&self, ecu: EcuType, level: SecurityLevel) -> Result<String, Box<dyn Error>> {
        // Validate and convert seed to bytes
        if self.seed_input.is_empty() {
            return Err("输入的Seed不能为空".into());
        }
        
        let seed = match hex_string_to_bytes(&self.seed_input) {
            Ok(bytes) => bytes,
            Err(_) => return Err("无效的Seed输入：必须是32个字符的十六进制字符串".into()),
        };
        
        // Get the security level as a u8
        let level_str = level.to_string();
        let level_num = level_str.parse::<u8>().map_err(|_| "Invalid security level")?;
        
        // Get the mask for the selected ECU and security level
        if let Some(mask_str) = get_matched_mask(&ecu.to_string(), level_num) {
            let mask = match hex_string_to_bytes(mask_str) {
                Ok(bytes) => bytes,
                Err(_) => return Err("配置中的掩码无效".into()),
            };
            
            // XOR the seed with the mask
            let mask_value = match xor_bytes(&seed, &mask) {
                Ok(bytes) => bytes,
                Err(e) => return Err(format!("异或操作失败: {}", e).into()),
            };
            
            crypto::calculate_cmac_key(&self.key_input, &mask_value)
        } else {
            Err(format!("找不到ECU: {} 与安全等级: {} 对应的掩码", ecu, level_num).into())
        }
    }

    fn view(&self) -> Element<Message> {
        // ECU Type picker
        let ecu_type_text = text("ECU选型:").size(16).width(Length::Fixed(80.0));
        // Use the dynamically generated list of ECU types
        let ecu_types = all_ecu_types();
        let ecu_picker = pick_list(
            ecu_types,
            self.ecu_type,
            Message::EcuTypeSelected,
        )
        .padding(8)
        .width(Length::Fixed(150.0));

        // Security Level picker
        let security_level_text = text("安全等级:").size(16).width(Length::Fixed(80.0));
        // Use the dynamically generated list of security levels
        let security_levels = all_security_levels();
        let security_picker = pick_list(
            security_levels,
            self.security_level,
            Message::SecurityLevelSelected,
        )
        .padding(8)
        .width(Length::Fixed(150.0));

        // Note text
        let note_text = column![
            text("注意事项:").size(14).color(*HIGHLIGHT_COLOR),
            text("1.Seed和Key数据长度为16个字节，格式为十六进制，数据前面不需要加0X'或'0x';").size(14).color(*HIGHLIGHT_COLOR),
            text("2.计算出的CMAC Key长度为16个字节，显示格式为十六进制。").size(14).color(*HIGHLIGHT_COLOR),
        ].spacing(5);

        // Seed input
        let seed_label = text("输入Seed:").size(16).width(Length::Fixed(80.0));
        let seed_input = text_input(
            "Ox",
            &self.seed_input
        )
        .on_input(Message::SeedInputChanged)
        .padding(8)
        .width(Length::Fixed(400.0));

        // Key input
        let key_label = text("输入Key:").size(16).width(Length::Fixed(80.0));
        let key_input = text_input(
            "Ox",
            &self.key_input
        )
        .on_input(Message::KeyInputChanged)
        .padding(8)
        .width(Length::Fixed(400.0));

        // CMAC Key output
        let cmac_key_label = text("CMAC Key:").size(16).width(Length::Fixed(80.0));
        let cmac_key_output = text_input(
            "Ox",
            &self.key_output
        )
        .on_input(Message::KeyOutputChanged)
        .padding(8)
        .width(Length::Fixed(340.0));
        
        // Copy button for CMAC key
        let copy_button = button(text("copy"))
            .style(button::secondary)
            .on_press(Message::CopyToClipboard)
            .height(iced::Fill)
            .width(Length::Fixed(60.0));

        // Buttons
        let calculate_button = button(text("计算").center())
            .style(button::primary)
            .on_press(Message::Calculate)
            .width(Length::Fixed(180.0))
            .height(Length::Fixed(50.0))
            .padding(10);

        let clear_button = button(text("清空").center())
            .style(button::secondary)
            .on_press(Message::Clear)
            .width(Length::Fixed(180.0))
            .height(Length::Fixed(50.0))
            .padding(10);

        // Footer
        let footer_text = text("Any feedback or issues, please contact us.").size(12);

        // Main layout
        let content = column![
            row![
                column![
                    row![
                        ecu_type_text,
                        ecu_picker,
                    ].spacing(10).align_y(alignment::Vertical::Center),
                    
                    row![
                        security_level_text,
                        security_picker,
                    ].spacing(10).align_y(alignment::Vertical::Center),
                ].spacing(10),
                note_text,
            ].spacing(10).align_y(alignment::Vertical::Center),

            column![
                row![   
                    seed_label,
                    seed_input,
                ].spacing(10).align_y(alignment::Vertical::Center),
                
                row![
                    key_label,
                    key_input,
                ].spacing(10).align_y(alignment::Vertical::Center),

                row![
                    cmac_key_label,
                    row![
                        cmac_key_output,
                        copy_button
                    ].spacing(0).height(iced::Shrink),
                ].spacing(10).align_y(alignment::Vertical::Center),
            ].spacing(10),
            
            row![
                calculate_button,
                clear_button,
            ]
            .spacing(20)
            .padding(Padding::new(0.0).top(10.0)),

            vertical_space(),
            
            row![
                horizontal_space(),
                footer_text,
            ].padding(0),
            ]
        .spacing(10)
        .padding(Padding { top: 20.0, right: 20.0, bottom: 0.0, left: 20.0 });

        // Use main container directly
    let main_content = container(content)
        .width(Length::Fill)
        .height(Length::Fill);
    
    // Show error modal if needed
    if self.show_error {
        let error_message = self.error_message.as_deref().unwrap_or("发生错误");
        
        let error_modal = container(
            column![
                text("错误").size(24),
                text(error_message).size(16).color(Color::from_rgb(1.0, 0.0, 0.0)),
                button(text("关闭").center())
                    .on_press(Message::DismissError)
                    .padding(10)
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::Fixed(400.0))
        .padding(10)
        .style(container::rounded_box);
        
        modal(main_content, error_modal, Message::DismissError)
    } else {
        main_content.into()
    }
    }
}

/// Creates a modal dialog that overlays the base content
/// 
/// - `base`: The application UI that will be shown beneath the modal
/// - `content`: The modal dialog content
/// - `on_blur`: Message to send when clicking outside the modal content
fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}


fn main() -> iced::Result {
    let settings = Settings {
        fonts: vec![include_bytes!("../assets/fonts/NotoSansCJKtc-Regular.otf").into()],
        default_font: Font::with_name("Noto Sans CJK TC"),
        antialiasing: true,
        ..Settings::default()
    };

    let window = window::Settings {
        min_size: Some((600.0, 370.0).into()),
        size: (600.0, 370.0).into(),
        icon: icon(),
        ..window::Settings::default()
    };

    iced::application(Seed2Cmac::title, Seed2Cmac::update, Seed2Cmac::view)
    .settings(settings)
    .window(window)
    .centered()
    .run()
}

fn icon() -> Option<window::Icon> {
    let img: Result<image::DynamicImage, image::ImageError> = image::load_from_memory_with_format(
        include_bytes!("../assets/icon.png"),
        image::ImageFormat::Png,
    );
    match img {
        Ok(img) => {
            let (width, height) = img.dimensions();
            window::icon::from_rgba(img.into_rgba8().into_raw(), width, height).ok()
        }
        Err(_) => panic!("Failed to load icon"),
    }
}
