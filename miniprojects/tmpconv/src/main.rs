fn main() {
    let celcius:f32 = 28.3;
    let fahrenheit = c_to_f(celcius);
    println!("Temp {celcius} in Fahrenheit is: {fahrenheit}");

    let fahreneheit = 70.0;
    println!("Temp {} in Celcius is: {}", fahreneheit, f_to_c(fahreneheit));
}


fn c_to_f(temp_celcius: f32) -> f32 {
    temp_celcius * (9.0 / 5.0)
}

fn f_to_c(temp_fahrenheit: f32) -> f32 {
    temp_fahrenheit - 32.0 * (5.0/9.0)
}
