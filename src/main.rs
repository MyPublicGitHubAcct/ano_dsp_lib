use ano_dsp_lib::dsp::GainControl;

fn main() {
    let mut gc = GainControl::new(1.0);
    gc.set_gain(2.0, 4);

    let input = [1.0, 1.0, 1.0, 1.0];
    let mut output = [0.0; 4];
    gc.process_block(&input, &mut output);

    println!("Output with gain ramp:");
    for (i, &sample) in output.iter().enumerate() {
        println!("Sample {}: {}", i, sample);
    }
}
