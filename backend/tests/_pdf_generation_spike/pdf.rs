use image::{ColorType, DynamicImage, GenericImageView, ImageFormat};
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{
    types::TextRenderingMode, Content, Filter, Finish, Name, PdfWriter, Rect, Ref, Str,
};
use usvg::TreeParsing;

fn pdf_writer() {
    let mut writer = PdfWriter::new();

    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let background_image_id = Ref::new(4);
    let font_id = Ref::new(5);
    let bg_s_mask_id = Ref::new(6);
    let content_id = Ref::new(7);
    // It's important to keep logo_image_id as last ref id, as svg2pdf will add
    // more ref ids with n + 1
    let logo_image_id = Ref::new(8);
    let background_image_name = Name(b"bg");
    let logo_image_name = Name(b"logo");
    let font_name = Name(b"font");

    writer.catalog(catalog_id).pages(page_tree_id);
    writer.pages(page_tree_id).kids([page_id]).count(1);

    let mut page = writer.page(page_id);
    let _dpi = 72.0; // if dpi == 72 then pt is px
    let _a5_long_mm = 210.0;
    let _a5_short_mm = 148.0;
    let a5_long = 595.2; // a5_long_mm * (dpi / 25.4); // if dpi == 72: ~ 595
    let a5_short = 419.5; // a5_short_mm * (dpi / 25.4); // if dpi == 72: ~ 419
    let a5 = Rect::new(0.0, 0.0, a5_long, a5_short);
    page.media_box(a5);
    page.parent(page_tree_id);
    page.contents(content_id);
    let mut resources = page.resources();
    resources
        .x_objects()
        .pair(background_image_name, background_image_id)
        .pair(logo_image_name, logo_image_id);
    resources.fonts().pair(font_name, font_id);
    resources.finish();
    page.finish();

    // BACKGROUND IMAGE

    let background_image_path = "tests/pdf_background.png";
    let (bg_dynamic_image, bg_filter, bg_encoded, bg_mask) =
        get_image(background_image_path, Some((a5_long, a5_short)));

    let mut image = writer.image_xobject(background_image_id, &bg_encoded);
    image.filter(bg_filter);
    image.width(bg_dynamic_image.width() as i32);
    image.height(bg_dynamic_image.height() as i32);
    image.color_space().device_rgb();
    image.bits_per_component(8);
    if bg_mask.is_some() {
        image.s_mask(bg_s_mask_id);
    }
    image.finish();

    // Add SMask if the image has transparency
    if let Some(encoded) = &bg_mask {
        let mut s_mask = writer.image_xobject(bg_s_mask_id, &encoded);
        s_mask.filter(bg_filter);
        s_mask.width(bg_dynamic_image.width() as i32);
        s_mask.height(bg_dynamic_image.height() as i32);
        s_mask.color_space().device_gray();
        s_mask.bits_per_component(8);
    }

    // Size the image at 1pt per pixel
    let w = bg_dynamic_image.width() as f32;
    let h = bg_dynamic_image.height() as f32;

    // Center the image on the page
    let x = (a5.x2 - w) / 2.0;
    let y = (a5.y2 - h) / 2.0;

    // Add image to content stream
    let mut content = Content::new();
    content.save_state();
    content.transform([w, 0.0, 0.0, h, x, y]);
    content.x_object(background_image_name);
    content.restore_state();

    // LOGO

    let svg = std::fs::read_to_string("tests/pdf_logo.svg").unwrap();
    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();

    svg2pdf::convert_tree_into(
        &tree,
        svg2pdf::Options::default(),
        &mut writer,
        logo_image_id,
    );

    content.save_state();
    let logo_x = 1584.0 * 0.05;
    let logo_y = 1504.0 * 0.05;
    content.transform([
        logo_x,
        0.0,
        0.0,
        logo_y,
        a5.x2 - logo_x - 15.0,
        a5.y2 - logo_y - 15.0,
    ]);
    content.x_object(logo_image_name);
    content.restore_state();

    writer
        .type1_font(font_id)
        .encoding_predefined(Name(b"WinAnsiEncoding"))
        .base_font(Name(b"Helvetica-Bold"));

    // PARTICIPANT NAME

    let participant_name = "Tasty McTest";
    let participant_name_font_size = 42.0;
    let participant_name_text: Vec<u8> =
        lopdf::Document::encode_text(Some("WinAnsiEncoding"), participant_name);
    content.begin_text();
    content.set_font(font_name, participant_name_font_size);
    content.set_text_rendering_mode(TextRenderingMode::Fill);
    content.set_fill_rgb(0.0, 0.0, 0.0);
    content.next_line(
        a5.x1 + 15.0,
        a5.y2 - participant_name_font_size - (150.0 / participant_name_font_size),
    );
    content.show(Str(participant_name_text.as_slice()));
    content.end_text();

    // STARTING NUMBER

    let starting_number = "6150";
    let starting_number_font_size = 124.0;
    let starting_number_text: Vec<u8> =
        lopdf::Document::encode_text(Some("WinAnsiEncoding"), starting_number);
    content.begin_text();
    content.set_font(font_name, starting_number_font_size);
    content.set_text_rendering_mode(TextRenderingMode::FillStroke);
    content.set_line_width(2.0);
    content.set_fill_rgb(1.0, 1.0, 1.0);
    content.set_stroke_rgb(0.0, 0.0, 0.0);
    content.next_line(
        (a5.x2 / 2.0) - (estimate_text_width(starting_number, starting_number_font_size) / 2.0),
        (a5.y2 / 2.0) - (starting_number_font_size / 3.0),
    );
    content.show(Str(starting_number_text.as_slice()));
    content.end_text();

    // COPYRIGHT NOTICE

    let copyright = "© Stefan Groenveld";
    let copyright_font_size = 10.0;
    let copyright_text: Vec<u8> = lopdf::Document::encode_text(Some("WinAnsiEncoding"), copyright);
    content.begin_text();
    content.set_font(font_name, copyright_font_size);
    content.set_text_rendering_mode(TextRenderingMode::Fill);
    content.set_fill_rgb(1.0, 1.0, 1.0);
    content.next_line(
        a5.x2 - estimate_text_width(copyright, copyright_font_size) - (copyright_font_size * 1.0),
        a5.y1 + (copyright_font_size * 0.8),
    );
    content.show(Str(copyright_text.as_slice()));
    content.end_text();

    // PAYMENT STATUS

    let unpaid = "NICHT BEZAHLT";
    let unpaid_font_size = 64.0;
    let unpaid_text: Vec<u8> = lopdf::Document::encode_text(Some("WinAnsiEncoding"), unpaid);
    content.begin_text();
    content.set_font(font_name, unpaid_font_size);
    content.set_text_rendering_mode(TextRenderingMode::FillStroke);
    content.set_stroke_rgb(0.0, 0.0, 0.0);
    content.set_fill_rgb(1.0, 0.10, 0.30);
    content.set_text_matrix([
        1.0,
        -0.667,
        0.667,
        1.0,
        a5.x1 + 20.0,
        a5.y2 - unpaid_font_size,
    ]);
    content.show(Str(unpaid_text.as_slice()));
    content.end_text();

    writer.stream(content_id, &content.finish());

    std::fs::write("target/pdf-writer_result.pdf", writer.finish()).unwrap();
}

fn get_image(
    image_path: &str,
    width_height: Option<(f32, f32)>,
) -> (DynamicImage, Filter, Vec<u8>, Option<Vec<u8>>) {
    let data = std::fs::read(image_path).unwrap();
    let format = image::guess_format(&data).unwrap();
    let dynamic = if let Some((width, height)) = width_height {
        let dynamic = image::load_from_memory(&data).unwrap();

        // The image might not have the same width/height ratio as the given values.
        // We have to find the smaller ratio which will then be used as divisor for
        // reducing the size of the image.
        // We must not use a larger value, as it would reduce the size of the image
        // to a smaller size than the page.
        let adjusted_width_ratio = dynamic.width() as f32 / width;
        let adjusted_height_ratio = dynamic.height() as f32 / height;
        let image_size_divisor = f32::min(adjusted_width_ratio, adjusted_height_ratio);

        dynamic.thumbnail_exact(
            (dynamic.width() as f32 / image_size_divisor).ceil() as u32,
            (dynamic.height() as f32 / image_size_divisor).ceil() as u32,
        )
    } else {
        image::load_from_memory(&data).unwrap()
    };

    let (filter, encoded, mask) = match format {
        ImageFormat::Jpeg => {
            assert_eq!(dynamic.color(), ColorType::Rgb8);
            (Filter::DctDecode, data, None)
        }

        ImageFormat::Png => {
            let level = CompressionLevel::UberCompression as u8;
            let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

            let mask = dynamic.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, encoded, mask)
        }

        _ => panic!("unsupported image format"),
    };
    (dynamic, filter, encoded, mask)
}

/// We simply assume fixed font family, character spacing etc.
fn estimate_text_width(text: &str, font_size: f32) -> f32 {
    text.chars()
        .map(|ch| match ch {
            '0' => 1.0_f32,
            '1' => 1.0_f32,
            '2' => 1.0_f32,
            '3' => 1.0_f32,
            '4' => 1.0_f32,
            '5' => 1.0_f32,
            '6' => 1.0_f32,
            '7' => 1.0_f32,
            '8' => 1.0_f32,
            '9' => 1.0_f32,
            _ => 1.0_f32,
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
        * (font_size / 2.0)
}
