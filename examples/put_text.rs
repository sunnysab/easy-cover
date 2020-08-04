use image::{DynamicImage, Rgba};
use imageproc::definitions::HasBlack;
use imageproc::drawing::{draw_text_mut, Canvas};
use rusttype::{Font, Point, Scale};
use std::fs;

fn open_font(font: &str) -> std::vec::Vec<u8> {
    let font_vec1 = fs::read(font).expect("Unable to read file");
    return font_vec1;
}

#[derive(Debug)]
pub struct Text<'a> {
    pub text: &'a str,
    pub font_size: f32,
    pub color: Rgba<u8>,
    pub font: &'a Font<'static>,
}

fn get_actual_box_size(text_box: &Text) -> (u32, u32) {
    let scale = Scale::uniform(text_box.font_size as f32);
    let start_point = Point { x: 0f32, y: 0f32 };
    // 根据字库字型，计算每个文字的坐标、大小等
    let mut glyphs_iter = text_box.font.layout(text_box.text, scale, start_point);
    // 取第一和最后一个字符
    // TODO: 考虑过少字符的情况
    let first_char = glyphs_iter.nth(0).unwrap().pixel_bounding_box().unwrap();
    let last_char = glyphs_iter.last().unwrap().pixel_bounding_box().unwrap();
    // 取整段文字的左上角和右下角点返回
    (
        (last_char.max.x - first_char.min.x) as u32,
        (last_char.max.y - first_char.min.y) as u32,
    )
}

fn draw_text(canvas: &mut DynamicImage, x: u32, y: u32, text: &Text<'_>) {
    let scale = Scale::uniform(text.font_size as f32);
    draw_text_mut(canvas, text.color, x, y, scale, text.font, text.text);
}

fn add_multi_text_ex(bg: &mut DynamicImage, texts: Vec<Text<'_>>) {
    let (image_w, image_h) = Canvas::dimensions(bg);
    // 画布大小 （宽，高）为图像大小的 3/4
    let (canvas_w, canvas_h) = (
        (image_w as f32 * 0.8f32) as u32, // 文字所占最大宽度为图像宽度的 80%
        (image_h as f32 * 0.75f32) as u32, // 文字所占最大高度为图像高度的 75%
    );
    // 计算所有文本大小
    let mut all_text_height = 0;
    let mut texts = texts
        .into_iter()
        .map(|each_line| {
            let (width, height) = get_actual_box_size(&each_line);
            all_text_height += height;

            (width, height, each_line)
        })
        .collect::<Vec<(u32, u32, Text<'_>)>>();

    // 重新计算所有文本高度和字号
    let mut all_text_height_2 = 0;
    for (width, height, text) in &mut texts {
        // 按照高度比例重新计算文本框大小
        let mut new_height = ((*height as f32 / all_text_height as f32) * canvas_h as f32) as u32;
        let mut new_width = ((new_height as f32 / *height as f32) * *width as f32) as u32;
        // 如果满足高度的同时，宽度超过最大宽度，则以宽度为准：缩小到原来的 canvas_w / new_width
        if new_width > canvas_w {
            new_height = (new_height as f32 * (canvas_w as f32 / new_width as f32)) as u32;
            new_width = canvas_w;
        }
        // 计算最终缩放比例，并缩放字体大小
        text.font_size *= new_height as f32 / *height as f32;
        all_text_height_2 += new_height;
        *width = new_width;
        *height = new_height;
    }
    println!("{:?}", texts);
    // 让所有文字垂直居中，计算左上角 y 轴坐标
    let mut start_y = (image_h - all_text_height_2 - 30 * texts.len() as u32) / 2;
    // 开始画画
    for (width, height, text) in texts {
        let x = (image_w - width) / 2;
        println!("x = {}, y = {}", x, start_y);
        draw_text(bg, x, start_y, &text);
        start_y += height + 30;
    }
}

fn add_multi_text(bg: &mut DynamicImage, texts: &[&str], font: Font<'static>) {
    add_multi_text_ex(
        bg,
        texts
            .into_iter()
            .map(|x| Text {
                text: x,
                font_size: 72f32,
                color: Rgba::black(),
                font: &font,
            })
            .collect(),
    );
}

fn main() {
    let file = "backgrounds/common-1.png";
    let mut img = image::open(file).unwrap();

    // Read font
    let font_data = open_font("fonts/华文中宋.ttf");
    let font: Font<'static> = Font::try_from_vec(font_data).unwrap();

    let text = "机器人爱好者协会";
    let text2 = "这是一张测试用的图片！！";

    add_multi_text(&mut img, &[text, text2], font);

    let img2 = img.resize(800, 600, image::imageops::FilterType::Triangle);
    img2.save("Hello.png");
}
