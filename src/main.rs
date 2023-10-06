use std::{cmp, env};
use interval_info::*;

//TODO add flags to suppress some output, or only show specific output
//TODO option to pass interval as decimal
//TODO passing interval as cents might not make sense, but think about it.

fn main() {

	let auto_flip = true;
	
	let mut scale_start_0 = true;
	
	let args: Vec<String> = env::args().skip(1).collect();
	
	if args.is_empty() {
		println!("No arguments passed in.\nPass in a fraction.\ne.g. 3/2");
		return;
	}
	
	if args.is_empty() || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
		println!(
			"\
help:
 last argument is the interval to input
 u64/u64
 e.g. 3/2

options:
 -h --help          | flag | display this help message
 -1 --scale-start-1 | flag | start scale tonic note count at 1 instead of 0"
		);
		return;
	}
	
	if args.contains(&"-1".to_string()) || args.contains(&"--scale-start-1".to_string()) {
		scale_start_0 = false;
	}
	
	let mut i = Interval::from_str(&args[args.len() -1]);
	
	let common_factor = i.reduce();
	if common_factor > 1 {
		println!("Interval reduced by common factor: {}\n", common_factor);
	}
	
	if auto_flip && i.den > i.num {
		println!("Interval flipped! Original: {}/{}\n", i.num, i.den);
		(i.num, i.den) = (i.den, i.num);
	}
	
	let num_limit = largest_prime_factor(i.num);
	let den_limit = largest_prime_factor(i.den);
	let limit = cmp::max(num_limit, den_limit);
	
	let num_primes = PrimeFactors::init(i.num);
	let den_primes = PrimeFactors::init(i.den);
	
	let cents = i.cents();
	
	println!(
		"\
Interval                   : {}/{}
Decimal                    : {}
Octaves                    : {}
Tritaves                   : {}
Cents                      : {}

Numerator                  : {}
Denominator                : {}

Tuning Limit               : {}
Numerator Tuning Limit     : {}
Denominator Tuning Limit   : {}

Numerator Prime Factors    : {}
Denominator Primes Factors : {}

Name(s) : {}

Equal Temperament Approximations:
Scale / EDO / TET          : note  |  cents off
12 standard semi tone scale: {}
24 quarter tone scale      : {}
19 ~ 1/3 comma meantone ext: {}
31 ~ 1/4 comma meantone ext: {}
41                         : {}
53                         : {}

1 octave scale             : {}
2 12tet tritone scale      : {}
3 12tet major third scale  : {}
4 12tet minor third scale  : {}

5                          : {}
6 whole tone scale         : {}
7                          : {}
8 neutral second, 3/2 tone : {}
9 2/3 tone scale           : {}
10 5*2                     : {}

15 5*3                     : {}
17                         : {}
22                         : {}
23                         : {}
26                         : {}
27 9*3                     : {}
29                         : {}
34 17*2                    : {}
72 sixth tone scale        : {}
96 eighth tone scale       : {}

Non Octave TET Scales:
Bohlen Pierce (TET version): {}
3/1 div 19                 : {}

Alpha (3/2)^(1/9)  ~78.0c  : {}
Beta  (3/2)^(1/11) ~63.8c  : {}
Gamma (3/2)^(1/20) ~35.1c  : {}

16/15 just semitone scale  : {}
10/9 just minor whole tone : {}
9/8 just major whole tone  : {}
8/7 septimal whole tone    : {}
7/6 septimal minor third   : {}
6/5 just minor third scale : {}
5/4 just major third scale : {}
4/3 just perfect fourth    : {}
3/2 just perfect fifth     : {}

Non TET Scales:
Just major scale           : {}
Just minor scale           : {}
Just chromatic scale       : {}
Pythagorean scale          : {}
Harmonic 12 tone scale     : {}
Harmonic 16 tone scale     : {}
Just Bohlen Pierce scale   : {}",
		
		i.num,
		i.den,
		i.to_f64(),
		i.octaves(),
		i.tritaves(),
		cents,
		
		i.num,
		i.den,
		
		limit,
		num_limit,
		den_limit,
		
		num_primes.to_string(),
		den_primes.to_string(),
		
		i.get_name(),
		
		closest_edo_note(cents, 12, scale_start_0).to_string(),
		closest_edo_note(cents, 24, scale_start_0).to_string(),
		closest_edo_note(cents, 19, scale_start_0).to_string(),
		closest_edo_note(cents, 31, scale_start_0).to_string(),
		closest_edo_note(cents, 41, scale_start_0).to_string(),
		closest_edo_note(cents, 53, scale_start_0).to_string(),
		
		closest_edo_note(cents, 1, scale_start_0).to_string(),
		closest_edo_note(cents, 2, scale_start_0).to_string(),
		closest_edo_note(cents, 3, scale_start_0).to_string(),
		closest_edo_note(cents, 4, scale_start_0).to_string(),
		
		closest_edo_note(cents, 5, scale_start_0).to_string(),
		closest_edo_note(cents, 6, scale_start_0).to_string(),
		closest_edo_note(cents, 7, scale_start_0).to_string(),
		closest_edo_note(cents, 8, scale_start_0).to_string(),
		closest_edo_note(cents, 9, scale_start_0).to_string(),
		closest_edo_note(cents, 10, scale_start_0).to_string(),
		
		closest_edo_note(cents, 15, scale_start_0).to_string(),
		closest_edo_note(cents, 17, scale_start_0).to_string(),
		closest_edo_note(cents, 22, scale_start_0).to_string(),
		closest_edo_note(cents, 23, scale_start_0).to_string(),
		closest_edo_note(cents, 26, scale_start_0).to_string(),
		closest_edo_note(cents, 27, scale_start_0).to_string(),
		closest_edo_note(cents, 29, scale_start_0).to_string(),
		closest_edo_note(cents, 34, scale_start_0).to_string(),
		closest_edo_note(cents, 72, scale_start_0).to_string(),
		closest_edo_note(cents, 96, scale_start_0).to_string(),
		
		closest_bp_note(cents, scale_start_0).to_string(),
		closest_tritave_19_note(cents, scale_start_0).to_string(),
		
		closest_root_fifth_note(cents, 9, scale_start_0).to_string(),
		closest_root_fifth_note(cents, 11, scale_start_0).to_string(),
		closest_root_fifth_note(cents, 20, scale_start_0).to_string(),
		
		closest_just_stack_note(cents, &Interval::new(16,15), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(10,9), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(9,8), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(8,7), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(7,6), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(6,5), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(5,4), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(4,3), scale_start_0).to_string(),
		closest_just_stack_note(cents, &Interval::new(3,2), scale_start_0).to_string(),
		
		closest_just_major_scale_note(cents, scale_start_0).to_string(),
		closest_just_minor_scale_note(cents, scale_start_0).to_string(),
		closest_just_chromatic_scale_note(cents, scale_start_0).to_string(),
		closest_pythagorean_19_tone_tritave_scale_note(cents, scale_start_0).to_string(),
		closest_just_harmonic_12_scale_note(cents, scale_start_0).to_string(),
		closest_just_harmonic_16_scale_note(cents, scale_start_0).to_string(),
		closest_just_bp_scale_note(cents, scale_start_0).to_string(),
	);
}
