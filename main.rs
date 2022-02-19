//use std::ptr::DynMetadata;

use core::panic;
use image::imageops::*;
use image::*;
use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::collections::HashMap;


fn kernel_mul(mat: Vec<Vec<i32>>, ker: Vec<Vec<i32>>) -> i32 {
    mat[2][2] * ker[0][0]
        + mat[2][1] * ker[0][1]
        + mat[2][0] * ker[0][2]
        + mat[1][2] * ker[1][0]
        + mat[1][1] * ker[1][1]
        + mat[1][0] * ker[1][2]
        + mat[0][2] * ker[2][0]
        + mat[0][1] * ker[2][1]
        + mat[0][0] * ker[2][2]
}

fn normalize(x: i32, y: i32) -> f64 {
    let sum = (x * x + y * y) as f64;
    sum.sqrt()
}

fn return_normalized_pixel(
    mat: Vec<Vec<i32>>,
    kernel: Vec<Vec<i32>>,
    kernel_tr: Vec<Vec<i32>>,
) -> i32 {
    let Gx = kernel_mul(mat.clone(), kernel.clone());
    let Gy = kernel_mul(mat.clone(), kernel_tr.clone());

    let mut normalized = normalize(Gx, Gy) as i32;

    if normalized > 255 {
        normalized = 255;
    } else if normalized < 0 {
        normalized = 0;
    }

    normalized
}

fn coordinates_not_on_edge(x: u32, y: u32, w: u32, h: u32) -> bool {
    x != 0 && y != 0 && x != w - 1 && y != h - 1
}

fn edge_detect(path: String) {
    let mut img = image::open(path).unwrap();

    //sobel operator works on grayscale images.
    img = img.grayscale();

    //maybe add gausina blur ?????
    let mut kernel: Vec<Vec<i32>> = Vec::new();
    let mut kernel_tr: Vec<Vec<i32>> = Vec::new();

    kernel.push(vec![1, 0, -1]);
    kernel.push(vec![2, 0, -2]);
    kernel.push(vec![1, 0, -1]);

    kernel_tr.push(vec![1, 2, 1]);
    kernel_tr.push(vec![0, 0, 0]);
    kernel_tr.push(vec![-1, -2, -1]);

    let mut gray_img = img.to_rgb8();
    let mut gray_img_cp = gray_img.clone();
    let mut edged_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(gray_img_cp.width(), gray_img_cp.height());

    let CONST_PIXEL = Rgb([0, 0, 0]);

    for i in gray_img.enumerate_pixels_mut() {
        let w = i.0;
        let h = i.1;
        let mut mat: Vec<Vec<i32>> = Vec::new();
        if coordinates_not_on_edge(
            w.clone(),
            h.clone(),
            img.width().clone(),
            img.height().clone(),
        ) {
            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h - 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h - 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h - 1).clone()[0] as i32,
            ]);
            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h).clone()[0] as i32,
            ]);
            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h + 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h + 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h + 1).clone()[0] as i32,
            ]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        } else if w == 0 && h == 0 {
            // we are on top left pixel
            mat.push(vec![0, 0, 0]);

            mat.push(vec![
                0,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h).clone()[0] as i32,
            ]);

            mat.push(vec![
                0,
                gray_img_cp.get_pixel(w + 1, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h + 1).clone()[0] as i32,
            ]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        } else if h == 0 && w == gray_img_cp.width() - 1 {
            mat.push(vec![0, 0, 0]);

            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                0,
            ]);

            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h + 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h + 1).clone()[0] as i32,
                0,
            ]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        } else if h == 0 {
            mat.push(vec![0, 0, 0]);

            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h).clone()[0] as i32,
            ]);

            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h + 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h + 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h + 1).clone()[0] as i32,
            ]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        } else if h == gray_img_cp.height() - 1 && w == 0 {
            //bottom left pixel

            mat.push(vec![
                0,
                gray_img_cp.get_pixel(w, h - 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h - 1).clone()[0] as i32,
            ]);

            mat.push(vec![
                0,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h).clone()[0] as i32,
            ]);

            mat.push(vec![0, 0, 0]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        } else if w == 0 {
            mat.push(vec![
                0,
                gray_img_cp.get_pixel(w, h - 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h - 1).clone()[0] as i32,
            ]);

            mat.push(vec![
                0,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h).clone()[0] as i32,
            ]);

            mat.push(vec![
                0,
                gray_img_cp.get_pixel(w, h + 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h + 1).clone()[0] as i32,
            ]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        } else if w == gray_img_cp.width() - 1 && h == gray_img_cp.height() - 1 {
            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h - 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h - 1).clone()[0] as i32,
                0,
            ]);
            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                0,
            ]);

            mat.push(vec![0, 0, 0]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        } else if h == gray_img_cp.height() - 1 {
            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h - 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h - 1).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h - 1).clone()[0] as i32,
            ]);

            mat.push(vec![
                gray_img_cp.get_pixel(w - 1, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w, h).clone()[0] as i32,
                gray_img_cp.get_pixel(w + 1, h).clone()[0] as i32,
            ]);

            mat.push(vec![0, 0, 0]);

            let p = return_normalized_pixel(mat.clone(), kernel.clone(), kernel_tr.clone()) as u8;

            edged_image.put_pixel(w, h, Rgb([p, p, p]));
        }
    }

    edged_image.save("what.jpg");
}

fn histogram_equalization(path: String) {
    let mut img = image::open(path).unwrap();
    img = img.grayscale();

    let mut img = img.to_rgb8();

    let mut number_of_intensity: Vec<u32> = Vec::new();
    let mut pdf: Vec<f64> = Vec::new(); //probability
    let mut cdf: Vec<f64> = Vec::new(); //cummulative distributive function

    for _ in 0..256 {
        number_of_intensity.push(0);
        pdf.push(0.0);
        cdf.push(0.0);
    }

    let mut max_pixel: u8 = 0;

    for i in img.enumerate_pixels() {
        if i.2[0].clone() > max_pixel {
            max_pixel = i.2[0].clone();
        }
        number_of_intensity[i.2[0].clone() as usize] += 1;
    }

    let mut n: u32 = 0;

    for i in 0..256 {
        n += number_of_intensity[i];
    }

    for i in 0..256 {
        pdf[i] = number_of_intensity[i] as f64 / n as f64;
    }

    cdf[0] = pdf[0];

    for i in 1..256 {
        cdf[i] = pdf[i] + cdf[i - 1];
    }

    //multiply by max pixel
    for i in 0..256 {
        cdf[i] *= max_pixel as f64;
    }

    for i in img.enumerate_pixels_mut() {
        i.2[0] = cdf[i.2[0].clone() as usize].floor() as u8;
        i.2[1] = cdf[i.2[1].clone() as usize].floor() as u8;
        i.2[2] = cdf[i.2[2].clone() as usize].floor() as u8;
    }

    img.save("histogram_test.jpg");
}

fn tolerance_check(color1: Rgb<u8>, color2: Rgb<u8>, tol: u8) -> bool {
    //taken from a github implementation

    let red_diff = max(color1[0], color2[0]) - min(color1[0], color2[0]);
    let green_diff = max(color1[1], color2[1]) - min(color1[1], color2[1]);
    let blue_diff = max(color1[2], color2[2]) - min(color1[2], color2[2]);

    let saturation_red = (red_diff / 255) as i32;
    let saturation_green = (green_diff / 255) as i32;
    let saturation_blue = (blue_diff / 255) as i32;

    let diff_percent = (saturation_blue + saturation_green + saturation_red) / 3 * 100;

    if diff_percent >= (tol as i32) {
        return false;
    }
    return true;
}

fn flood_fill(
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    color: Rgb<u8>,
    old_color: Rgb<u8>,
    tol: u8,
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let mut deq: VecDeque<(i32, i32)> = VecDeque::new();

    let mut visited:HashMap<(i32,i32) ,bool> = HashMap::new();
    
    for i in img.enumerate_pixels_mut(){
        visited.insert((i.0 as i32,i.1 as i32), false);
    }
    visited.insert((start_x , start_y) , false);

    
    deq.push_back((start_x, start_y));

    while deq.len() > 0 {
        let t = deq.front().unwrap().clone();
        deq.pop_front();
        let x = t.0;
        let y = t.1;
       // println!("{} , {} ", x, y);

        if x < 0 || y < 0 || x >= end_x || y >= end_y {
            continue;
        }

        if visited.get(&(x,y)).unwrap().clone() == true{
            continue;
        }


        visited.insert((x , y) , true);

        let p = img
            .get_pixel(start_x.try_into().unwrap(), start_y.try_into().unwrap())
            .clone();

        deq.push_back((x + 1, y));
        deq.push_back((x - 1, y));
        deq.push_back((x, y + 1));
        deq.push_back((x, y - 1));
        if //p == old_color
        tolerance_check(p, old_color, tol)
        {
            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color);
        }
    }
    img.save("12.png");
}

fn main() {
    let mut path=String::from("/home/ivan/fmi-courses/rust-course/rust_project/prj/src/Golden_Retriever_Carlos_(10581910556).jpg");
    let mut path = String::from("src/man_coat.jpg");
    let mut img = image::open(path.clone()).unwrap().to_rgb8();

    flood_fill(
        0,
        0,
        img.width().try_into().unwrap(),
        img.height().try_into().unwrap(),
        Rgb([0,0,0]),
        Rgb([255, 255, 255]),
        15,
        &mut img,
    );

    //  img=img.grayscale();
    // img.save("grmona.jpg");
    //  let mut img_cp = img.clone();

    //edge_detect(path.clone());
    //  let mut path =String::from("/home/ivan/fmi-courses/rust-course/rust_project/prj/src/1_0JwGb7OY6U3EvV_FFDiQkw.jpg");
    //histogram_equalization(path);
}
