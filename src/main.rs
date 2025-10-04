use eframe::egui::{self};
use egui::FontFamily;
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

fn main() -> eframe::Result<()> {
    // ウィンドウの設定
    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_inner_size(egui::vec2(500., 500.)),
        ..Default::default()
    };

    // アプリケーションの起動
    eframe::run_native(
        "Kuwahara-filter",
        options,
        Box::new(|_cc| {
            egui_extras::install_image_loaders(&_cc.egui_ctx);
            // 日本語フォントの設定
            setup_custom_fonts(&_cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

// フォント設定用の関数
fn setup_custom_fonts(ctx: &egui::Context) {
    // フォント設定を取得
    let mut fonts = egui::FontDefinitions::default();

    // 日本語フォント（可変ウェイト）を追加
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/NotoSansJP-VariableFont_wght.ttf"))
            .into(),
    );

    // フォントファミリーに追加
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_jp".to_owned()); // 一番優先度高く追加

    // モノスペースフォントにも日本語フォントを追加
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("noto_sans_jp".to_owned());

    // フォント設定を適用
    ctx.set_fonts(fonts);
}

struct TargetImage {
    raw_file_name: String,
    processing_image: DynamicImage,
}

// アプリケーションの状態を保持する構造体
struct MyApp {
    image: Option<TargetImage>,
    is_selected: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: None,
            is_selected: false,
        }
    }
}

// アプリケーションの描画とロジックを実装
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Kuwahara-filter");

            // OpenImage
            if ui.button("Pick file").clicked() {
                // Open the file dialog to pick a file.
                // self.file_dialog.pick_file();
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let decoder = image::ImageReader::open(path.display().to_string().clone())
                        .unwrap()
                        .into_decoder()
                        .unwrap();

                    let _image = DynamicImage::from_decoder(decoder).unwrap();
                    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                    self.image = Some(TargetImage {
                        raw_file_name: file_name,
                        processing_image: _image,
                    });
                }
            }

            if self.is_selected {
                if ui.button("convert kuwahara").clicked() {
                    self.image = Some(TargetImage {
                        raw_file_name: self.image.as_ref().unwrap().raw_file_name.clone(),
                        processing_image: self.image.as_ref().unwrap().processing_image.clone(),
                    });
                }
            }

            if self.image.is_none() {
                ui.label("No file picked yet.");
                return;
            } else {
                ui.label(format!(
                    "Picked file: {:?}",
                    self.image.as_ref().unwrap().raw_file_name
                ));
            }

            ui.separator();

            if let Some(img) = &mut self.image {
                let size = [
                    img.processing_image.width() as usize,
                    img.processing_image.height() as usize,
                ];
                // RGBA8に変換する
                let rgba8 = img.processing_image.to_rgba8();
                let pixels = rgba8.as_flat_samples();

                // egui のColorImageに変換する
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                // テクスチャを作成
                let texture = ctx.load_texture(
                    "selected-image",
                    color_image,
                    egui::TextureOptions::default(),
                );

                ui.image((texture.id(), texture.size_vec2()));
                self.is_selected = true;
            };
        });
    }
}
