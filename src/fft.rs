use hound::WavReader;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use plotters::prelude::*;


pub fn fft_once() {
    // Step 1: 读取音频文件
    let reader = WavReader::open("audio.wav").unwrap();
    let samples: Vec<i16> = reader
        .samples::<i16>()
        .map(|s| s.unwrap())
        .collect();
    
    // Step 2: 使用 FFT 进行频谱分析
    let mut fft_input: Vec<Complex<f32>> = samples.iter()
        .map(|&x| Complex::new(x as f32, 0.0))
        .collect();
    
    let mut planner = FftPlanner::new(false); // false for FFT
    let fft = planner.plan_fft(fft_input.len());
    fft.process(&mut fft_input);
    
    // Step 3: 提取频谱并绘制
    let frequencies: Vec<f32> = fft_input.iter()
        .map(|c| c.norm()) // 获取幅度
        .collect();
    
    let root = BitMapBackend::new("spectrum.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..frequencies.len(), 0.0..*frequencies.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
        .unwrap();
    
    chart
        .draw_series(LineSeries::new(
            frequencies.iter().enumerate().map(|(x, y)| (x, *y)),
            &RED,
        ))
        .unwrap();
}
