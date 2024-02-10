// math utils
fn exp(num: f32) -> f32 { f32::powf(2.71828182, num) }
fn max(a: i32, b: i32) -> i32 { if a > b {a} else {b} }

fn convolve_rgba(src: &Vec<u32>, out: &mut Vec<u32>, line: &mut Vec<f32>, coeff: &[f32; 8], width: i32, height: i32) {
    // for guassian blur
    // takes src image and writes the blurred and transposed result into out
    let mut rgba: i32;

    // rgba values are technically u8 but are stored as f32 to avoid casting
    let mut prev_src_r: f32;
    let mut prev_src_g: f32;
    let mut prev_src_b: f32;
    let mut prev_src_a: f32;
    
    let mut curr_src_r: f32;
    let mut curr_src_g: f32;
    let mut curr_src_b: f32;
    let mut curr_src_a: f32;

    let mut curr_out_r: f32;
    let mut curr_out_g: f32;
    let mut curr_out_b: f32;
    let mut curr_out_a: f32;
    
    let mut prev_out_r: f32;
    let mut prev_out_g: f32;
    let mut prev_out_b: f32;
    let mut prev_out_a: f32;
    
    let mut prev_prev_out_r: f32;
    let mut prev_prev_out_g: f32;
    let mut prev_prev_out_b: f32;
    let mut prev_prev_out_a: f32;

    let mut src_index: i32;
    let mut out_index: i32;
    let mut line_index: i32;
    
    let mut coeff_a0: f32;
    let mut coeff_a1: f32;
    let mut coeff_b1: f32;
    let mut coeff_b2: f32;

    let mut i: i32;
    let mut j: i32;

    i = 0;
    while i < height {
        src_index = i * width;
        out_index = i;
        line_index = 0;

        // left to right
        rgba = src[src_index as usize] as i32;

        prev_src_r = (rgba & 0xff) as f32;
        prev_src_g = ((rgba >> 8) & 0xff) as f32;
        prev_src_b = ((rgba >> 16) & 0xff) as f32;
        prev_src_a = ((rgba >> 24) & 0xff) as f32;

        prev_prev_out_r = prev_src_r * coeff[6];
        prev_prev_out_g = prev_src_g * coeff[6];
        prev_prev_out_b = prev_src_b * coeff[6];
        prev_prev_out_a = prev_src_a * coeff[6];

        prev_out_r = prev_prev_out_r;
        prev_out_g = prev_prev_out_g;
        prev_out_b = prev_prev_out_b;
        prev_out_a = prev_prev_out_a;

        coeff_a0 = coeff[0];
        coeff_a1 = coeff[1];
        coeff_b1 = coeff[4];
        coeff_b2 = coeff[5];

        j = 0;
        while j < width {
            rgba = src[src_index as usize] as i32;
            curr_src_r = (rgba & 0xff) as f32;
            curr_src_g = ((rgba >> 8) & 0xff) as f32;
            curr_src_b = ((rgba >> 16) & 0xff) as f32;
            curr_src_a = ((rgba >> 24) & 0xff) as f32;

            curr_out_r = curr_src_r * coeff_a0 + prev_src_r * coeff_a1 + prev_out_r * coeff_b1 + prev_prev_out_r * coeff_b2;
            curr_out_g = curr_src_g * coeff_a0 + prev_src_g * coeff_a1 + prev_out_g * coeff_b1 + prev_prev_out_g * coeff_b2;
            curr_out_b = curr_src_b * coeff_a0 + prev_src_b * coeff_a1 + prev_out_b * coeff_b1 + prev_prev_out_b * coeff_b2;
            curr_out_a = curr_src_a * coeff_a0 + prev_src_a * coeff_a1 + prev_out_a * coeff_b1 + prev_prev_out_a * coeff_b2;

            prev_prev_out_r = prev_out_r;
            prev_prev_out_g = prev_out_g;
            prev_prev_out_b = prev_out_b;
            prev_prev_out_a = prev_out_a;

            prev_out_r = curr_out_r;
            prev_out_g = curr_out_g;
            prev_out_b = curr_out_b;
            prev_out_a = curr_out_a;

            prev_src_r = curr_src_r;
            prev_src_g = curr_src_g;
            prev_src_b = curr_src_b;
            prev_src_a = curr_src_a;

            line[line_index as usize] = prev_out_r;
            line[(line_index + 1) as usize] = prev_out_g;
            line[(line_index + 2) as usize] = prev_out_b;
            line[(line_index + 3) as usize] = prev_out_a;
            line_index += 4;
            src_index += 1;
            j += 1;
        }

        src_index -= 1;
        line_index -= 4;
        out_index += height * (width - 1);

        // right to left
        rgba = src[src_index as usize] as i32;

        prev_src_r = (rgba & 0xff) as f32;
        prev_src_g = ((rgba >> 8) & 0xff) as f32;
        prev_src_b = ((rgba >> 16) & 0xff) as f32;
        prev_src_a = ((rgba >> 24) & 0xff) as f32;

        prev_prev_out_r = prev_src_r * coeff[7];
        prev_prev_out_g = prev_src_g * coeff[7];
        prev_prev_out_b = prev_src_b * coeff[7];
        prev_prev_out_a = prev_src_a * coeff[7];

        prev_out_r = prev_prev_out_r;
        prev_out_g = prev_prev_out_g;
        prev_out_b = prev_prev_out_b;
        prev_out_a = prev_prev_out_a;

        curr_src_r = prev_src_r;
        curr_src_g = prev_src_g;
        curr_src_b = prev_src_b;
        curr_src_a = prev_src_a;

        coeff_a0 = coeff[2];
        coeff_a1 = coeff[3];

        j = width - 1;
        while j >= 0 {
            curr_out_r = curr_src_r * coeff_a0 + prev_src_r * coeff_a1 + prev_out_r * coeff_b1 + prev_prev_out_r * coeff_b2;
            curr_out_g = curr_src_g * coeff_a0 + prev_src_g * coeff_a1 + prev_out_g * coeff_b1 + prev_prev_out_g * coeff_b2;
            curr_out_b = curr_src_b * coeff_a0 + prev_src_b * coeff_a1 + prev_out_b * coeff_b1 + prev_prev_out_b * coeff_b2;
            curr_out_a = curr_src_a * coeff_a0 + prev_src_a * coeff_a1 + prev_out_a * coeff_b1 + prev_prev_out_a * coeff_b2;

            prev_prev_out_r = prev_out_r;
            prev_prev_out_g = prev_out_g;
            prev_prev_out_b = prev_out_b;
            prev_prev_out_a = prev_out_a;

            prev_out_r = curr_out_r;
            prev_out_g = curr_out_g;
            prev_out_b = curr_out_b;
            prev_out_a = curr_out_a;

            prev_src_r = curr_src_r;
            prev_src_g = curr_src_g;
            prev_src_b = curr_src_b;
            prev_src_a = curr_src_a;

            rgba = src[src_index as usize] as i32;
            curr_src_r = (rgba & 0xff) as f32;
            curr_src_g = ((rgba >> 8) & 0xff) as f32;
            curr_src_b = ((rgba >> 16) & 0xff) as f32;
            curr_src_a = ((rgba >> 24) & 0xff) as f32;

            rgba = ((line[line_index       as usize] as i32 + prev_out_r as i32) << 0 ) +
                   ((line[(line_index + 1) as usize] as i32 + prev_out_g as i32) << 8 ) +
                   ((line[(line_index + 2) as usize] as i32 + prev_out_b as i32) << 16) +
                   ((line[(line_index + 3) as usize] as i32 + prev_out_a as i32) << 24);

            out[out_index as usize] = rgba as u32;

            src_index -= 1;
            line_index -= 4;
            out_index -= height;
            j -= 1;
        }
        
        i += 1;
    }
}

pub fn blur(w: i32, h: i32, pixels: &mut Vec<u8>, radius: f32) -> Vec<u8> {
    // Unify input data type, to keep convolver calls isomorphic
    let mut src32: Vec<u32> = pixels
        .chunks(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    
    let mut out: Vec<u32> = vec![0; src32.len()];
    let mut tmp_line: Vec<f32> = vec![0.0; (max(w, h) * 4) as usize];

    // gaussCoef
    let sigma: f32 = if radius < 0.5 {0.5} else {radius};
    
    let a = exp(0.726 * 0.726) / sigma;
    let g1 = exp(-a);
    let g2 = exp(-2.0 * a);
    let k = (1.0 - g1) * (1.0 - g1) / (1.0 + 2.0 * a * g1 - g2);
    
    let a0 = k;
    let a1 = k * (a - 1.0) * g1;
    let a2 = k * (a + 1.0) * g1;
    let a3 = -k * g2;
    let b1 = 2.0 * g1;
    let b2 = -g2;
    let left_corner = (a0 + a1) / (1.0 - b1 - b2);
    let right_corner = (a2 + a3) / (1.0 - b1 - b2);
  
    let coeff: [f32; 8] = [a0, a1, a2, a3, b1, b2, left_corner, right_corner];

    convolve_rgba(&src32, &mut out, &mut tmp_line, &coeff, w, h);
    convolve_rgba(&out, &mut src32, &mut tmp_line, &coeff, h, w);

    return src32.iter().flat_map(|&x| u32::to_le_bytes(x)).collect();
}