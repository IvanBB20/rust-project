//use std::ptr::DynMetadata;

use core::panic;

use image::imageops::*;
use image::*;

fn grayscale(path: String) -> DynamicImage {
    /*
     let mut img=image::open(path).unwrap().into_rgb8();
     let mut new_img =img.clone();
     for mut i in img.enumerate_pixels_mut(){
         let r:u8 = (i.2[0] as f64 *0.3 )as u8;
         let g:u8 = (i.2[1] as f64 *0.59) as u8;
         let b:u8=(i.2[2] as f64 *0.11) as u8;
         let gray = r+g+b;
     //    print!("{} , {}  , {} \n" ,r ,g,b);
         new_img.put_pixel(i.0, i.1, Rgb( [ gray,gray,gray ] ) );

     }
    new_img
    */
    let mut img = image::open(path).unwrap();
    img = img.grayscale();

    // let cp=img.to_rgb8()
    img
}

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



fn histogram_equalization(path:String){
    let mut img = image::open(path).unwrap();
    img=img.grayscale();

    let mut img = img.to_rgb8();


    let mut frequency:Vec<i32> = Vec::new();
    //let mut commulative_frequency:Vec<i32>=Vec::new;
    let mut commulative_frequency:Vec<i32>= Vec::new();
    let mut normalized_frequency:Vec<i32>=Vec::new();

    for _ in 0..256{
        frequency.push(0);
        commulative_frequency.push(0);
        normalized_frequency.push(0);
    }

    let mut max_pixel:u8=0;

    for i in img.enumerate_pixels(){
        frequency[i.2.clone()[0] as usize]+=1;
        if i.2.clone()[0]>max_pixel{
            max_pixel=i.2.clone()[0];
        }
    }

    //panic!("wron");
    
    commulative_frequency[0] = frequency[0];

    for i in 1..256{
        commulative_frequency[i]=frequency[i]+commulative_frequency[i-1];
    }

    commulative_frequency[0] = frequency[0];

    let max=commulative_frequency[255];

    for i in 0..256{
        
        normalized_frequency[i] =  ( (commulative_frequency[i] / max as i32  ) as f64).ceil() as i32;
    }
   
    for i in img.enumerate_pixels_mut(){
        i.2[0] = max_pixel*normalized_frequency[i.2[0] as usize] as u8;
        i.2[1] = max_pixel*normalized_frequency[i.2[0] as usize] as u8;
        i.2[2] = max_pixel*normalized_frequency[i.2[0] as usize] as u8;
    }

    //let i :ImageBuffer<>
     img.save("histogram.jpg");
}
fn main() {
    let mut path=String::from("/home/ivan/fmi-courses/rust-course/rust_project/prj/src/Golden_Retriever_Carlos_(10581910556).jpg");
    let mut path = String::from("src/Valve_original_(1).png");
   // let mut img = image::open(path.clone()).unwrap().to_rgb8();

  //  let mut img_cp = img.clone();

    //edge_detect(path.clone());
  //  let mut path =String::from("/home/ivan/fmi-courses/rust-course/rust_project/prj/src/1_0JwGb7OY6U3EvV_FFDiQkw.jpg");
    histogram_equalization(path);
}
