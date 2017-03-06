extern crate time;

use std::env;
use time::precise_time_ns;

/// Audio sample rate for the test set, used for realtime speed
/// calculation
const SAMPLE_RATE: f64 = 48000.0;
/// Total length of samples the filter benchmarks are ran on
const SAMPLE_COUNT: u64 = 524288;
/// Select how many IIR filters should be applied consecutively
/// on each buffer during the benchmark
const FILTER_COUNT: usize = 100;

const BUFFER_LEN: usize = 128;


/// 2nd order biquad filter
#[derive(Copy)]
struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,

    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl Clone for Biquad {
    fn clone(&self) -> Biquad {
        *self
    }
}

impl Biquad {
    fn new() -> Biquad {
        Biquad {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

/// Displays the benchmark timing results and a real-time performance estimate
fn print_elapsed(msg: &str, start: u64, filter_count: usize) {
    let elapsed = precise_time_ns() - start;
    let duration = elapsed as f64 / filter_count as f64 / SAMPLE_COUNT as f64;
    let realtime = 1.0 / duration / SAMPLE_RATE * 1e+9;
    println!("\t{:<40}{:.3} ns\t{:.0}x realtime", msg, duration, realtime);
}

macro_rules! create_iir_intoiterator_fn {
    ($func:ident) => (
        fn $func(buf: &mut [f64], bq: &mut Biquad) {
            for y in buf {
                let x = *y;
                *y = (bq.b0 * x) + (bq.b1 * bq.x1) +  (bq.b2 * bq.x2)
                     - (bq.a1 * bq.y1) - (bq.a2 * bq.y2);

                bq.x2 = bq.x1;
                bq.x1 = x;

                bq.y2 = bq.y1;
                bq.y1 = *y;
            }
        }
    )
}

fn iir_c_style_for_loop(buf: &mut [f64], bq: &mut Biquad) {
    for i in 0..buf.len() {
        let x = buf[i];
        buf[i] = (bq.b0 * x) + (bq.b1 * bq.x1) + (bq.b2 * bq.x2) - (bq.a1 * bq.y1) -
                 (bq.a2 * bq.y2);

        bq.x2 = bq.x1;
        bq.x1 = x;

        bq.y2 = bq.y1;
        bq.y1 = buf[i];
    }
}

fn iir_unchecked_c_style_for_loop(buf: &mut [f64], bq: &mut Biquad) {
    unsafe {
        for i in 0..buf.len() {
            let x = *buf.get_unchecked(i);
            *buf.get_unchecked_mut(i) =
                (bq.b0 * x) + (bq.b1 * bq.x1) + (bq.b2 * bq.x2) - (bq.a1 * bq.y1) - (bq.a2 * bq.y2);

            bq.x2 = bq.x1;
            bq.x1 = x;

            bq.y2 = bq.y1;
            bq.y1 = *buf.get_unchecked(i);
        }
    }
}

fn iir_intoiterator_enforced_len(buf: &mut [f64], bq: &mut Biquad) {

    if buf.len() != BUFFER_LEN {
        unreachable!()
    }

    for y in buf {
        let x = *y;
        *y = (bq.b0 * x) + (bq.b1 * bq.x1) + (bq.b2 * bq.x2) - (bq.a1 * bq.y1) - (bq.a2 * bq.y2);

        bq.x2 = bq.x1;
        bq.x1 = x;

        bq.y2 = bq.y1;
        bq.y1 = *y;
    }
}

fn iir_array_c_style_for_loop(buf: &mut [f64; BUFFER_LEN], bq: &mut Biquad) {
    for i in 0..buf.len() {
        let x = buf[i];
        buf[i] = (bq.b0 * x) + (bq.b1 * bq.x1) + (bq.b2 * bq.x2) - (bq.a1 * bq.y1) -
                 (bq.a2 * bq.y2);

        bq.x2 = bq.x1;
        bq.x1 = x;

        bq.y2 = bq.y1;
        bq.y1 = buf[i];
    }
}


create_iir_intoiterator_fn!(iir_intoiterator_shared);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_1);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_2);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_3);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_4);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_5);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_6);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_7);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_8);
create_iir_intoiterator_fn!(iir_intoiterator_uniq_9);


fn main() {
    println!("Rust Vector and Array performance comparison");


    let mut buffer_len_mut = BUFFER_LEN;
    if let Some(arg1) = env::args().nth(1) {
        buffer_len_mut = arg1.parse::<usize>().unwrap();
        println!("Overriding buffer_len_immut to {}", {
            buffer_len_mut
        });
    }

    let buffer_len_immut = {
        buffer_len_mut
    };
    let buffer_len_borrowed_immut = {
        buffer_len_immut
    };
    let mut buffer_len_borrowed_mut = {
        buffer_len_mut
    };

    // actually mutate variables
    let r = precise_time_ns as usize;
    buffer_len_borrowed_mut = buffer_len_borrowed_mut + r;
    buffer_len_mut = buffer_len_mut + r;
    println!("Temp: {}, {}", buffer_len_borrowed_mut, {
        buffer_len_mut
    });
    buffer_len_borrowed_mut = buffer_len_borrowed_mut - r;
    buffer_len_mut = buffer_len_mut - r;

    let buffer_count = SAMPLE_COUNT / buffer_len_immut as u64;

    println!("buffer_len_immut: {}, buffer_len_mut: {}",
             {
                 buffer_len_immut
             },
             {
                 buffer_len_mut
             });
    println!("buffer_len_borrowed_immut: {}, buffer_len_borrowed_mut: {}",
             buffer_len_borrowed_immut,
             buffer_len_borrowed_mut);


    println!("\nVector of borrowed mutable binding size");
    {
        let mut buf = vec![0.0; buffer_len_borrowed_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];

        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_shared(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_borrowed_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_uniq_1(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_uniq_1", start, FILTER_COUNT);
    }
    if buffer_len_immut == BUFFER_LEN {
        let mut buf = vec![0.0; buffer_len_borrowed_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_enforced_len(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_borrowed_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_borrowed_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_unchecked_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
    }


    println!("\nVector of borrowed immutable binding size");
    {
        let mut buf = vec![0.0; buffer_len_borrowed_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];

        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_shared(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_borrowed_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_uniq_2(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_uniq_2", start, FILTER_COUNT);
    }
    if buffer_len_immut == BUFFER_LEN {
        let mut buf = vec![0.0; buffer_len_borrowed_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_enforced_len(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_borrowed_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_borrowed_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_unchecked_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
    }


    println!("\nVector of mutable binding size");
    {
        let mut buf = vec![0.0; buffer_len_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];

        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_shared(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_uniq_3(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_uniq_3", start, FILTER_COUNT);
    }
    if buffer_len_immut == BUFFER_LEN {
        let mut buf = vec![0.0; buffer_len_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_enforced_len(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_mut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_unchecked_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
    }


    println!("\nVector of immutable binding size");
    {
        let mut buf = vec![0.0; buffer_len_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];

        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_shared(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_uniq_4(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_uniq_4", start, FILTER_COUNT);
    }
    if buffer_len_immut == BUFFER_LEN {
        let mut buf = vec![0.0; buffer_len_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_enforced_len(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];

        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
    }
    {
        let mut buf = vec![0.0; buffer_len_immut];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_unchecked_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
    }


    if buffer_len_immut == BUFFER_LEN {
        println!("\nVector of const size");
        {
            let mut buf = vec![0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_shared(buf.as_mut_slice(), &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
        }
        {
            let mut buf = vec![0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_uniq_5(buf.as_mut_slice(), &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_uniq_5", start, FILTER_COUNT);
        }
        {
            let mut buf = vec![0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_enforced_len(buf.as_mut_slice(), &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
        }
        {
            let mut buf = vec![0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
                }
            }
            print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
        }
        {
            let mut buf = vec![0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_unchecked_c_style_for_loop(buf.as_mut_slice(), &mut biquads[f]);
                }
            }
            print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
        }
    }


    println!("\nSlice of mutable size from Array (4096)");
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_shared(&mut buf[..buffer_len_mut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
    }
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_uniq_6(&mut buf[..buffer_len_mut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_uniq_6", start, FILTER_COUNT);
    }
    if buffer_len_immut == BUFFER_LEN {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_enforced_len(&mut buf[..buffer_len_mut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
    }
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_c_style_for_loop(&mut buf[..buffer_len_mut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
    }
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_unchecked_c_style_for_loop(&mut buf[..buffer_len_mut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
    }



    println!("\nSlice of immutable size from Array (4096)");
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_shared(&mut buf[..buffer_len_immut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
    }
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_uniq_7(&mut buf[..buffer_len_immut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_uniq_7", start, FILTER_COUNT);
    }
    if buffer_len_immut == BUFFER_LEN {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_intoiterator_enforced_len(&mut buf[..buffer_len_immut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
    }
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_c_style_for_loop(&mut buf[..buffer_len_immut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
    }
    {
        let mut buf = [0.0; 4096];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir_unchecked_c_style_for_loop(&mut buf[..buffer_len_immut], &mut biquads[f]);
            }
        }
        print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
    }


    if buffer_len_immut == BUFFER_LEN {
        println!("\nSlice of const size from Array (4096)");
        {
            let mut buf = [0.0; 4096];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_shared(&mut buf[..BUFFER_LEN], &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; 4096];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_uniq_8(&mut buf[..BUFFER_LEN], &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_uniq_8", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; 4096];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_enforced_len(&mut buf[..BUFFER_LEN], &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; 4096];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_c_style_for_loop(&mut buf[..BUFFER_LEN], &mut biquads[f]);
                }
            }
            print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; 4096];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_unchecked_c_style_for_loop(&mut buf[..BUFFER_LEN], &mut biquads[f]);
                }
            }
            print_elapsed("iir_unchecked_c_style_for_loop", start, FILTER_COUNT);
        }
    }


    if buffer_len_immut == BUFFER_LEN {
        println!("\nArray ({})", BUFFER_LEN);
        {
            let mut buf = [0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_shared(&mut buf, &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_shared", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_uniq_9(&mut buf, &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_uniq_9", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_intoiterator_enforced_len(&mut buf, &mut biquads[f]);
                }
            }
            print_elapsed("iir_intoiterator_enforced_len", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_c_style_for_loop(&mut buf, &mut biquads[f]);
                }
            }
            print_elapsed("iir_c_style_for_loop", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_unchecked_c_style_for_loop(&mut buf, &mut biquads[f]);
                }
            }
            print_elapsed("iir_unchecked_c_style_for_loop_shared", start, FILTER_COUNT);
        }
        {
            let mut buf = [0.0; BUFFER_LEN];
            let mut biquads = [Biquad::new(); FILTER_COUNT];
            let start = precise_time_ns();
            for _ in 0..buffer_count {
                for f in 0..FILTER_COUNT {
                    iir_array_c_style_for_loop(&mut buf, &mut biquads[f]);
                }
            }
            print_elapsed("iir_array_c_style_for_loop", start, FILTER_COUNT);
        }
    }
}

