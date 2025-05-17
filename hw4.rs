pub fn log( n: i32, base :i32 ) -> i32
{
    if base <= 1 || n < 1 {
        return 0;
    }
    let mut power = 1;
    let mut exponent = 0;
    while power <= n {
        power *= base;
        exponent += 1;
    }
    exponent - 1
}

pub fn double_list( list: &mut Vec<f32> )
{
    for i in 0..list.len() {
        let x = list[i];
        list[i] = x * x;
    }
}
pub fn sum_list( list : & Vec<f32> ) -> f32
{
    let mut sum = 0.0;
    for &x in list.iter() {
        sum += x;
    }
    sum
}
pub fn average_list( list : & Vec<f32> ) -> f32
{
    if list.is_empty() {
        return 0.0;
    }
    sum_list(list) / (list.len() as f32)
}

fn sqrtf(value: f32) -> f32 {
    if value < 0.0 {
        return f32::NAN;
    }
    if value == 0.0 {
        return 0.0;
    }
    let mut guess = value / 2.0;
    for _ in 0..15 {
        guess = 0.5 * (guess + value / guess);
    }
    guess
}

pub fn std_dev( list: Vec<f32> ) -> f32
{
    let n = list.len();
    if n == 0 {
        return 0.0;
    }

    let mut sum = 0.0;
    for &x in list.iter() {
        sum += x;
    }
    let mean = sum / (n as f32);

    let mut sum_sq_diff = 0.0;
    for &x in list.iter() {
        let diff = x - mean;
        sum_sq_diff += diff * diff;
    }

    let var = sum_sq_diff / (n as f32);
    sqrtf(var)
}

