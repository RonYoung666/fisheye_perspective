use chrono::Local;
use image::error::ImageError;
use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel, Rgb, RgbImage};
use std::env;
use std::f32::consts::PI;

fn main() -> Result<(), ImageError> {
    /* 入参 */
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <File> <width> <distance>", &args[0]);
        return Ok(());
    }
    let file = &args[1];
    let width = (&args[2]).parse::<f32>().unwrap();
    let distance = (&args[3]).parse::<f32>().unwrap();
    println!("图片 = \"{file}\"");
    println!("宽度 = {width}");
    println!("距离 = {distance}");

    /* 输入图片 */
    let input = ImageReader::open(file)?.decode()?;
    let (w, h) = input.dimensions();
    println!("input w = {w}, h = {h}");

    /* 计算视觉画面 */
    let mut output = RgbImage::new(630, 630);
    for i in 0 as u32..630 {
        for j in 0 as u32..630 {
            let x: i32 = i as i32 - 315;
            let y: i32 = j as i32 - 315;

            /* 输出像素点的角度 */
            let radian = ((x * x + y * y) as f32).sqrt() / 200.0;

            /* 超过 90° 范围 */
            if radian >= PI / 2.0 {
                output.put_pixel(i, j, Rgb([0, 0, 0]));
                continue;
            }

            /* 正切值 */
            let tan = radian.tan();

            /* 图片距离视点的像素数 */
            let pix_distance = distance * tan * w as f32 / width;

            /* 输入图片对应位置的坐标 */
            let in_x = x
                .checked_mul(pix_distance as i32)
                .unwrap_or(i32::MAX)
                .checked_div(((x * x + y * y) as f32).sqrt() as i32)
                .unwrap_or(0)
                .checked_add(w as i32 / 2)
                .unwrap_or(i32::MAX);
            let in_y = y
                .checked_mul(pix_distance as i32)
                .unwrap_or(i32::MAX)
                .checked_div(((x * x + y * y) as f32).sqrt() as i32)
                .unwrap_or(0)
                .checked_add(h as i32 / 2)
                .unwrap_or(i32::MAX);
            if in_x < 0 || in_x >= w as i32 || in_y < 0 || in_y >= h as i32 {
                output.put_pixel(i, j, Rgb([100, 100, 100]));
                continue;
            }

            /* 复制输入图片对应位置的像素 */
            output.put_pixel(i, j, input.get_pixel(in_x as u32, in_y as u32).to_rgb());

            if i == 204 && j == 204 {
                println!("radian = {radian}");
                println!("tan = {tan}");
                println!("pix_distance = {pix_distance}");
                println!("in_x = {in_x}");
                println!("in_y = {in_y}");
            }
        }
    }

    /* 写入文件 */
    let out_file = format!(
        "out_{width}_{distance}_{}.png",
        Local::now().format("%Y-%m-%d_%H%M%S").to_string()
    );
    println!("out_file = \"{out_file}\"");
    output.save(out_file)?;

    Ok(())
}
