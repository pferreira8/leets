use criterion::{Criterion};
use leets::two_sum;
use rand::{thread_rng, Rng};
use gnuplot::{Figure, Caption, Color};



fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen_range(-50..100)).collect()
}

fn generate_random_target(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

fn two_sum_benchmark(c: &mut Criterion, rand_target: i32) {
    let input_data = generate_random_array(200); // Adjust the size as per your requirements

    c.bench_function("Two Sum Benchmark", |b| b.iter(|| two_sum(&input_data, rand_target)));
}

fn main() {
    // let x = [0u32, 1, 2];
    // let y = [3u32, 4, 5];
    // let mut fg = Figure::new();
    // fg.set_terminal("canvas", "stuff.html");
    // fg.axes2d()
    // .lines(&x, &y, &[Caption("A line"), Color("black")]);
    // fg.show().unwrap();
    let mut criterion = Criterion::default();
    let target = generate_random_target(25, 50);
    two_sum_benchmark(&mut criterion, target);
    criterion.final_summary();
}
