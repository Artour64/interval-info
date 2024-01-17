
//maybe make u128 so that the last 2 intervals also fit
#[derive(PartialEq)]
pub struct Interval {
	pub num: u64,
	pub den: u64
}

impl Interval {
	pub fn new(num: u64, den: u64) -> Self {
		Self{num,den}
	}
	
	pub fn get_name(&self) -> &str {
		for i in NAMES.iter() {
			if i.interval == *self {
				return i.name
			}
		}
		"Unknown"
	}
	
	pub fn to_f64(&self) -> f64 {
		(self.num as f64) / (self.den as f64)
	}
	
	pub fn octaves(&self) -> f64 {
		self.to_f64().log2()
	}
	
	pub fn tritaves(&self) -> f64 {
		//last digit changed from 4 to 5 so that 3/1 is one tritave
		self.octaves() * 0.6309297535714575
	}
	
	pub fn cents(&self) -> f64 {
		1200.0 * self.octaves()
	}
	
	//returns common factor reduced by. 1 means already in reduced form
	pub fn reduce(&mut self) -> u64 {
		let mut reduced = 1;
		let mut factor = 2;
		while factor <= self.num {
			if self.num % factor == 0 && self.den % factor == 0 {
				self.num /= factor;
				self.den /= factor;
				reduced *= factor;
			} else {
				factor += 1;
			}
		}
		reduced
	}
	
	pub fn from_str(s: &str) -> Self {
		let pivot = s.chars().position(|c| c == '/').unwrap_or(0);
		let num = s[..pivot].parse::<u64>().unwrap_or(1);
		let den = s[pivot+1..].parse::<u64>().unwrap_or(1);
		Self::new(num,den)
	}
}

pub fn largest_prime_factor(mut n: u64) -> u64 {
	let mut limit = 1;
	let mut factor = 2;
	
	while 1 < n {
		if n % factor == 0 {
			n /= factor;
			limit = factor
		} else {
			factor += 1;
		}
	}
	limit
}

pub struct PrimeFactors {
	len: u8,
	ar: [IntPower; 16]
}

impl PrimeFactors {
	pub fn init(num: u64) -> Self {
		let mut x = Self::new();
		x.set(num);
		x
	}
	
	fn new() -> Self {
		Self{len: 0, ar: [IntPower::new(0,0); 16]}
	}
	
	fn set(&mut self, mut num: u64) {
		self.len = 1;
		self.ar[0].num = 2;
		self.ar[0].pow = 0;
		
		while self.ar[(self.len - 1) as usize].num <= num {
			if num % self.ar[(self.len - 1) as usize].num == 0 {
				num /= self.ar[(self.len - 1) as usize].num;
				self.ar[(self.len - 1) as usize].pow += 1;
				
			} else if self.ar[(self.len - 1) as usize].pow > 0 {
				self.ar[self.len as usize].num = self.ar[(self.len - 1) as usize].num + 1;
				self.ar[self.len as usize].pow = 0;
				self.len += 1;
				
			} else {
				self.ar[(self.len - 1) as usize].num += 1;
			}
		}
		if self.ar[(self.len - 1) as usize].pow == 0 {
			self.len -= 1;
		}
	}
	
	pub fn to_string(&self) -> String {
		if self.len == 0 {
			return "1".to_string();
		}
		
		let mut s = self.ar[0].to_string();
		
		for c in 1..self.len {
			s += "*";
			s += &self.ar[c as usize].to_string();
		}
		
		s
	}
}

#[derive(Copy,Clone)]
struct IntPower {
	pub num: u64,
	pub pow: u8//u64 numbers can only have max 64 pow
}

impl IntPower {
	fn new(num: u64, pow: u8) -> Self {
		Self{num, pow}
	}
	
	pub fn to_string(&self) -> String {
		let mut s = self.num.to_string();
		if self.pow > 1 {
			s += "^";
			s += &self.pow.to_string();
		}
		s
	}
}

pub struct ScaleApproximation {
	pub note: u64,
	pub offset: f64
}

impl ScaleApproximation {
	fn new(note: u64, offset: f64) -> Self {
		ScaleApproximation{note, offset}
	}
	
	pub fn to_string(&self) -> String {
		let plus = if self.offset > 0.0 { "+" } else { "" };
		format!("{:<6}|  {}{}", self.note, plus, self.offset)
	}
}

pub fn closest_edo_note(cents: f64, div: u64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = edo_note_cents(div, note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn edo_note_cents(div: u64, note: u64) -> f64 {
	((1200 * note) as f64) / (div as f64)
}

pub fn closest_bp_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = bp_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn bp_note_cents(note: u64) -> f64 {
	(1901.9550008653873 * (note as f64)) / 13.0
}

pub fn closest_root_fifth_note(cents: f64, root: u64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = root_fifth_note_cents(root, note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn root_fifth_note_cents(root: u64, note: u64) -> f64 {
	1.5f64.powf((note as f64)/(root as f64)).log2() * 1200.0//maybe refactor
}

pub fn closest_just_stack_note(cents: f64, interval: &Interval, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		//let cents2 = just_stack_note_cents(interval, note);//change back to this one when it no longer integer overflows
		let cents2 = just_stack_note_cents_simple(interval, note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

/*
//TODO check if integer will overflow, and use the other one if so
fn just_stack_note_cents(i: &Interval, note: u64) -> f64 {
	Interval::new(
		(i.num as u128).pow(note as u32) as u64,
		(i.den as u128).pow(note as u32) as u64
	).cents()
}
*/

//unlike the other one, will not integer overflow
fn just_stack_note_cents_simple(i: &Interval, note: u64) -> f64 {
	i.cents() * note as f64
}

pub fn closest_tritave_19_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = tritave_19_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn tritave_19_note_cents(note: u64) -> f64 {
	(1901.9550008653873 * (note as f64)) / 19.0
}

pub fn closest_just_major_scale_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = just_major_scale_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn just_major_scale_note_cents(note: u64) -> f64 {
	let scale: [f64;7] = [
		0.0,//					1/1
		203.91000173077484,//	9/8
		386.3137138648348,//	5/4
		498.0449991346125,//	4/3
		701.9550008653874,//	3/2
		884.3587129994474,//	5/3
		1088.2687147302222,//	15/8
	];
	scale[(note % 7) as usize] + ((note / 7) * 1200) as f64
}

pub fn closest_just_minor_scale_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = just_minor_scale_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn just_minor_scale_note_cents(note: u64) -> f64 {
	let scale: [f64;7] = [
		0.0,//					1/1
		111.73128526977774,//	16/15
		315.64128700055255,//	6/5
		498.0449991346125,//	4/3
		701.9550008653874,//	3/2
		813.6862861351652,//	8/5
		1017.5962878659401,//	9/5
	];
	scale[(note % 7) as usize] + ((note / 7) * 1200) as f64
}

pub fn closest_just_chromatic_scale_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = just_chromatic_scale_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn just_chromatic_scale_note_cents(note: u64) -> f64 {
	let scale: [f64;12] = [
		0.0,//					1/1
		111.73128526977774,//	16/15
		203.91000173077484,//	9/8
		315.64128700055255,//	6/5
		386.3137138648348,//	5/4
		498.0449991346125,//	4/3
		590.2237155956096,//	45/32, maybe use a different tritone? 7/5?
		701.9550008653874,//	3/2
		813.6862861351652,//	8/5
		884.3587129994474,//	5/3
		1017.5962878659401,//	9/5
		1088.2687147302222,//	15/8
	];
	scale[(note % 12) as usize] + ((note / 12) * 1200) as f64
}

pub fn closest_just_harmonic_12_scale_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = just_harmonic_12_scale_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn just_harmonic_12_scale_note_cents(note: u64) -> f64 {
	let scale: [f64;12] = [
		0.0,//					1/1
		104.95540950040728,//	17/16
		203.91000173077484,//	9/8
		297.5130161323026,//	19/16
		386.3137138648348,//	5/4
		470.7809073345124,//	21/16
		551.3179423647567,//	11/8
		701.9550008653874,//	3/2
		840.5276617693106,//	13/8
		905.8650025961623,//	27/16
		968.8259064691249,//	7/4
		1088.2687147302222,//	15/8
	];
	scale[(note % 12) as usize] + ((note / 12) * 1200) as f64
}

pub fn closest_just_harmonic_16_scale_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = just_harmonic_16_scale_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn just_harmonic_16_scale_note_cents(note: u64) -> f64 {
	let scale: [f64;16] = [
		0.0,//					1/1
		104.95540950040728,//	17/16
		203.91000173077484,//	9/8
		297.5130161323026,//	19/16
		386.3137138648348,//	5/4
		470.7809073345124,//	21/16
		551.3179423647567,//	11/8
		628.2743472684155,//	23/16
		701.9550008653874,//	3/2
		772.6274277296696,//	25/16
		840.5276617693106,//	13/8
		905.8650025961623,//	27/16
		968.8259064691249,//	7/4
		1029.5771941530866,//	29/16
		1088.2687147302222,//	15/8
		1145.0355724642502,//	31/16
	];
	scale[(note % 16) as usize] + ((note / 16) * 1200) as f64
}

pub fn closest_just_bp_scale_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = just_bp_scale_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn just_bp_scale_note_cents(note: u64) -> f64 {
	let scale: [f64;13] = [
		0.0,//					1/1
		133.23757486649274,//	27/25
		301.84652039515726,//	25/21
		435.08409526165,//		9/7
		582.51219260429,//		7/5
		736.930615656807,//		75/49
		884.3587129994474,//	5/3
		1017.5962878659401,//	9/5
		1165.0243852085803,//	49/25
		1319.442808261097,//	15/7
		1466.8709056037378,//	7/3
		1600.10848047023,//		63/25
		1768.7174259988947,//	25/9
	];
	scale[(note % 13) as usize] + ((note / 13) as f64 * 1901.9550008653873)
}

pub fn closest_pythagorean_19_tone_tritave_scale_note(cents: f64, scale_start_0: bool) -> ScaleApproximation {
	let mut note = 0;
	let mut offset = f64::MAX;
	
	loop {
		let cents2 = pythagorean_19_tone_tritave_scale_note_cents(note);
		let offset2 = cents2 - cents;
		
		if offset.abs() < offset2.abs() {
			return ScaleApproximation::new(note - scale_start_0 as u64, offset);
		}
		
		note += 1;
		offset = offset2;
	}
}

fn pythagorean_19_tone_tritave_scale_note_cents(note: u64) -> f64 {
	let scale: [f64;19] = [
		0.0,//					1/1			1   / 1
		90.22499567306306,//	256/243		2^8 / 3^5
		203.91000173077484,//	9/8			3^2 / 2^3
		294.13499740383764,//	32/27		2^5 / 3^3
		407.8200034615497,//	81/64		3^4 / 2^6
		498.0449991346125,//	4/3			2^2 / 3
		611.7300051923246,//	729/512		3^6 / 2^9
		701.9550008653874,//	3/2			3   / 2
		792.1799965384502,//	128/81		2^7 / 3^4
		905.8650025961623,//	27/16		3^3 / 2^4
		996.089998269225,//		16/9		2^4 / 3^2
		1109.775004326937,//	243/128		3^5 / 2^7
		1200.0,//				2/1     	2   / 1
		1290.224995673063,//	512/243		2^9 / 3^5
		1403.9100017307749,//	9/4			3^2 / 2^2
		1494.1349974038376,//	64/27		2^6 / 2^3
		1607.8200034615497,//	81/32		3^4 / 2^5
		1698.0449991346125,//	8/3			2^3 / 3
		1811.7300051923246,//	729/256		3^6 / 2^8
	];
	scale[(note % 19) as usize] + ((note / 19) as f64 * 1901.9550008653873)
}

//TODO Refactor find_closest_note and the just_scales_note to use common code. Maybe make a struct that describes a scale

//-----------------------

struct IntervalNamePair {
	pub interval: Interval,
	pub name: &'static str 
}

//shorter
const fn i(num: u64, den: u64, name: &'static str) -> IntervalNamePair {
	IntervalNamePair{interval: Interval{num,den}, name}
}

//maybe add some intervals and names from other sources
const NAMES: [IntervalNamePair;560] = [
	
	i(3, 1, "tritave, perfect twelfth, BP thirteenth"),
	i(4, 1, "double octave"),
	
	//https://www.huygens-fokker.org/docs/intervals.html
	//commented out last 2 intervals, because their numerators and/or denominators don't fit in u64
	i(1, 1, "unison, perfect prime"),
	i(2, 1, "octave"),
	i(3, 2, "perfect fifth"),
	i(4, 3, "perfect fourth"),
	i(5, 3, "major sixth, BP sixth"),
	i(5, 4, "major third"),
	i(6, 5, "minor third"),
	i(7, 3, "minimal tenth, BP tenth"),
	i(7, 4, "harmonic seventh"),
	i(7, 5, "septimal or Huygens' tritone, BP fourth"),
	i(7, 6, "septimal minor third"),
	i(8, 5, "minor sixth"),
	i(8, 7, "septimal whole tone"),
	i(9, 4, "major ninth"),
	i(9, 5, "just minor seventh, BP seventh"),
	i(9, 7, "septimal major third, BP third"),
	i(9, 8, "major whole tone"),
	i(10, 7, "Euler's tritone"),
	i(10, 9, "minor whole tone"),
	i(11, 5, "neutral ninth"),
	i(11, 6, "undecimal neutral seventh, 21/4-tone"),
	i(11, 7, "undecimal augmented fifth"),
	i(11, 8, "undecimal semi-augmented fourth"),
	i(11, 9, "undecimal neutral third"),
	i(11, 10, "Ptolemy's second, 4/5-tone"),
	i(12, 7, "septimal major sixth"),
	i(12, 11, "undecimal neutral second, 3/4-tone"),
	i(13, 7, "16/3-tone"),
	i(13, 8, "tridecimal neutral sixth"),
	i(13, 9, "tridecimal diminished fifth"),
	i(13, 10, "tridecimal semi-diminished fourth"),
	i(13, 11, "tridecimal minor third"),
	i(13, 12, "tridecimal 2/3-tone"),
	i(14, 9, "septimal minor sixth"),
	i(14, 11, "undecimal diminished fourth or major third"),
	i(14, 13, "2/3-tone"),
	i(15, 7, "septimal minor ninth, BP ninth"),
	i(15, 8, "classic major seventh"),
	i(15, 11, "undecimal augmented fourth"),
	i(15, 13, "tridecimal 5/4-tone"),
	i(15, 14, "major diatonic semitone"),
	i(16, 7, "septimal major ninth"),
	i(16, 9, "Pythagorean minor seventh"),
	i(16, 11, "undecimal semi-diminished fifth"),
	i(16, 13, "tridecimal neutral third"),
	i(16, 15, "minor diatonic semitone"),
	i(17, 8, "septendecimal minor ninth"),
	i(17, 9, "septendecimal major seventh"),
	i(17, 10, "septendecimal diminished seventh"),
	i(17, 12, "2nd septendecimal tritone"),
	i(17, 14, "supraminor third"),
	i(17, 15, "septendecimal whole tone"),
	i(17, 16, "17th harmonic"),
	i(18, 11, "undecimal neutral sixth"),
	i(18, 13, "tridecimal augmented fourth"),
	i(18, 17, "Arabic lute index finger"),
	i(19, 10, "undevicesimal major seventh"),
	i(19, 12, "undevicesimal minor sixth"),
	i(19, 15, "undevicesimal ditone"),
	i(19, 16, "19th harmonic"),
	i(19, 17, "quasi-meantone"),
	i(19, 18, "undevicesimal semitone"),
	i(20, 9, "small ninth"),
	i(20, 11, "large minor seventh"),
	i(20, 13, "tridecimal semi-augmented fifth"),
	i(20, 17, "septendecimal augmented second"),
	i(20, 19, "small undevicesimal semitone"),
	i(21, 11, "undecimal major seventh"),
	i(21, 16, "narrow fourth"),
	i(21, 17, "submajor third"),
	i(21, 20, "minor semitone"),
	i(22, 13, "tridecimal major sixth"),
	i(22, 15, "undecimal diminished fifth"),
	i(22, 19, "Godzilla third"),
	i(22, 21, "undecimal minor semitone"),
	i(23, 12, "vicesimotertial major seventh"),
	i(23, 16, "23rd harmonic"),
	i(23, 18, "vicesimotertial major third"),
	i(24, 13, "tridecimal neutral seventh"),
	i(24, 17, "1st septendecimal tritone"),
	i(24, 19, "smaller undevicesimal major third"),
	i(24, 23, "vicesimotertial minor semitone"),
	i(25, 9, "classic augmented eleventh, BP twelfth"),
	i(25, 12, "classic augmented octave"),
	i(25, 14, "middle minor seventh"),
	i(25, 16, "classic augmented fifth"),
	i(25, 18, "classic augmented fourth"),
	i(25, 21, "BP second, quasi-equal minor third"),
	i(25, 22, "undecimal acute whole tone"),
	i(25, 24, "classic chromatic semitone, minor chroma"),
	i(26, 15, "tridecimal semi-augmented sixth"),
	i(26, 25, "tridecimal 1/3-tone"),
	i(27, 14, "septimal major seventh"),
	i(27, 16, "Pythagorean major sixth"),
	i(27, 17, "septendecimal minor sixth"),
	i(27, 20, "acute fourth"),
	i(27, 22, "neutral third, Zalzal wosta of al-Farabi"),
	i(27, 23, "vicesimotertial minor third"),
	i(27, 25, "large limma, BP small semitone, Zarlino semitone"),
	i(27, 26, "tridecimal comma"),
	i(28, 15, "grave major seventh"),
	i(28, 17, "submajor sixth"),
	i(28, 19, "Hendrix fifth"),
	i(28, 25, "middle second"),
	i(28, 27, "Archytas' 1/3-tone"),
	i(29, 16, "29th harmonic"),
	i(30, 17, "septendecimal minor seventh"),
	i(30, 19, "smaller undevicesimal minor sixth"),
	i(31, 16, "31st harmonic"),
	i(31, 30, "31st-partial chroma"),
	i(32, 15, "minor ninth"),
	i(32, 17, "17th subharmonic"),
	i(32, 19, "19th subharmonic"),
	i(32, 21, "wide fifth"),
	i(32, 23, "23rd subharmonic"),
	i(32, 25, "classic diminished fourth"),
	i(32, 27, "Pythagorean minor third"),
	i(32, 29, "29th subharmonic"),
	i(32, 31, "Greek enharmonic 1/4-tone"),
	i(33, 20, "undecimal submajor sixth"),
	i(33, 25, "2 pentatones"),
	i(33, 26, "tridecimal major third"),
	i(33, 28, "undecimal minor third"),
	i(33, 32, "undecimal comma, al-Farabi's 1/4-tone"),
	i(34, 19, "quasi-mean seventh"),
	i(34, 21, "supraminor sixth"),
	i(34, 27, "septendecimal major third"),
	i(35, 18, "septimal semi-diminished octave"),
	i(35, 24, "septimal semi-diminished fifth"),
	i(35, 27, "septimal semi-diminished fourth, 9/4-tone"),
	i(35, 32, "septimal neutral second"),
	i(35, 34, "septendecimal 1/4-tone"),
	i(36, 19, "smaller undevicesimal major seventh"),
	i(36, 25, "classic diminished fifth"),
	i(36, 35, "septimal diesis, 1/4-tone"),
	i(37, 32, "37th harmonic"),
	i(39, 32, "39th harmonic, Zalzal wosta of Ibn Sina"),
	i(40, 21, "acute major seventh"),
	i(40, 27, "grave fifth"),
	i(40, 33, "undecimal supraminor third"),
	i(40, 39, "tridecimal minor diesis"),
	i(42, 25, "quasi-equal major sixth"),
	i(44, 25, "undecimal grave minor seventh"),
	i(44, 27, "neutral sixth"),
	i(45, 28, "septimale wide minor sixth"),
	i(45, 32, "diatonic tritone"),
	i(45, 44, "1/5-tone"),
	i(46, 45, "23rd-partial chroma"),
	i(48, 25, "classic diminished octave"),
	i(48, 35, "septimal semi-augmented fourth"),
	i(49, 25, "BP eighth"),
	i(49, 30, "larger approximation to neutral sixth"),
	i(49, 36, "Arabic lute acute fourth"),
	i(49, 40, "larger approximation to neutral third"),
	i(49, 45, "BP minor semitone"),
	i(49, 44, "undecimal minor whole tone"),
	i(49, 48, "slendro diesis, septimal 1/6-tone"),
	i(50, 27, "grave major seventh"),
	i(50, 33, "3 pentatones"),
	i(50, 49, "Erlich's decatonic comma, tritonic diesis"),
	i(51, 40, "septendecimal diminished fourth"),
	i(51, 50, "17th-partial chroma"),
	i(52, 33, "tridecimal minor sixth"),
	i(54, 35, "septimal semi-augmented fifth"),
	i(54, 49, "Zalzal's mujannab"),
	i(55, 36, "undecimal semi-augmented fifth"),
	i(55, 48, "undecimal semi-augmented whole tone"),
	i(55, 49, "quasi-equal major second"),
	i(55, 54, "telepathma"),
	i(56, 45, "septimal narrow major third"),
	i(56, 55, "undecimal diesis, konbini comma"),
	i(57, 32, "undevicesimal minor seventh"),
	i(57, 56, "Hendrix comma"),
	i(60, 49, "smaller approximation to neutral third"),
	i(63, 25, "quasi-equal major tenth, BP eleventh"),
	i(63, 32, "octave - septimal comma"),
	i(63, 34, "submajor seventh"),
	i(63, 40, "narrow minor sixth"),
	i(63, 50, "quasi-equal major third"),
	i(64, 33, "33rd subharmonic"),
	i(64, 35, "septimal neutral seventh"),
	i(64, 37, "37th subharmonic"),
	i(64, 39, "39th subharmonic"),
	i(64, 45, "2nd tritone"),
	i(64, 49, "2 septatones or septatonic major third"),
	i(64, 63, "septimal comma, Archytas' comma"),
	i(65, 64, "13th-partial chroma"),
	i(66, 65, "Winmeanma"),
	i(68, 35, "23/4-tone"),
	i(68, 63, "supraminor second"),
	i(68, 65, "Valentine semitone"),
	i(72, 49, "Arabic lute grave fifth"),
	i(72, 55, "undecimal semi-diminished fourth"),
	i(72, 59, "Ibn Sina's neutral third"),
	i(74, 73, "approximation to Pythagorean comma"),
	i(75, 49, "BP fifth"),
	i(75, 56, "marvelous fourth"),
	i(75, 64, "classic augmented second"),
	i(77, 64, "Keemun minor third"),
	i(77, 72, "undecimal secor"),
	i(77, 76, "approximation to 53-tone comma"),
	i(78, 71, "porcupine neutral second"),
	i(78, 77, "tridecimal minor third comma"),
	i(80, 49, "smaller approximation to neutral sixth"),
	i(80, 63, "wide major third"),
	i(81, 44, "2nd undecimal neutral seventh"),
	i(81, 50, "acute minor sixth"),
	i(81, 64, "Pythagorean major third"),
	i(81, 68, "Persian wosta"),
	i(81, 70, "Al-Hwarizmi's lute middle finger"),
	i(81, 80, "syntonic comma, Didymus comma"),
	i(85, 72, "septendecimal minor third"),
	i(88, 49, "undecimal minor seventh"),
	i(88, 81, "2nd undecimal neutral second"),
	i(89, 84, "quasi-equal semitone"),
	i(91, 59, "15/4-tone"),
	i(91, 90, "medium tridecimal comma, superleap"),
	i(96, 95, "19th-partial chroma"),
	i(98, 55, "quasi-equal minor seventh"),
	i(99, 70, "2nd quasi-equal tritone"),
	i(99, 98, "small undecimal comma"),
	i(100, 63, "quasi-equal minor sixth"),
	i(100, 81, "grave major third"),
	i(100, 99, "Ptolemy's comma"),
	i(105, 64, "septimal neutral sixth"),
	i(105, 104, "small tridecimal comma"),
	i(112, 75, "marvelous fifth"),
	i(112, 99, "undecimal major second"),
	i(117, 88, "tridecimal gentle fourth"),
	i(121, 64, "Alpharabian major seventh"),
	i(121, 120, "undecimal seconds comma, biyatisma"),
	i(125, 64, "classic augmented seventh, octave - minor diesis"),
	i(125, 72, "classic augmented sixth"),
	i(125, 96, "classic augmented third"),
	i(125, 108, "semi-augmented whole tone"),
	i(125, 112, "classic augmented semitone"),
	i(126, 125, "septimal semicomma, Starling comma"),
	i(128, 75, "diminished seventh"),
	i(128, 81, "Pythagorean minor sixth"),
	i(128, 105, "septimal neutral third"),
	i(128, 121, "undecimal semitone"),
	i(128, 125, "minor diesis, diesis"),
	i(135, 112, "septimal wide minor third"),
	i(135, 128, "major chroma, major limma"),
	i(140, 99, "quasi-equal tritone"),
	i(144, 125, "classic diminished third"),
	i(144, 143, "Grossma"),
	i(145, 144, "29th-partial chroma"),
	i(153, 125, "7/4-tone"),
	i(153, 152, "Ganassi's comma"),
	i(160, 81, "octave - syntonic comma"),
	i(160, 107, "Milliet de Chales 1/4-comma fifth"),
	i(161, 93, "19/4-tone"),
	i(162, 149, "Persian neutral second"),
	i(168, 89, "quasi-equal major seventh"),
	i(169, 162, "tridecimal tritonic third tone"),
	i(169, 168, "Schulter's comma"),
	i(176, 175, "valinorsma"),
	i(192, 125, "classic diminished sixth"),
	i(192, 175, "septimal 4/5-tone"),
	i(196, 195, "mynucuma"),
	i(210, 209, "spleen comma"),
	i(216, 125, "semi-augmented sixth"),
	i(224, 135, "narrow septimal major sixth"),
	i(225, 128, "augmented sixth"),
	i(225, 224, "septimal kleisma"),
	i(231, 200, "5/4-tone"),
	i(241, 221, "Meshaqah's 3/4-tone"),
	i(243, 125, "octave - maximal diesis"),
	i(243, 128, "Pythagorean major seventh"),
	i(243, 160, "acute fifth"),
	i(243, 200, "acute minor third"),
	i(243, 224, "Archytas' 2/3-tone"),
	i(243, 242, "neutral third comma, rastma"),
	i(245, 242, "Nautilus comma"),
	i(245, 243, "minor BP diesis, Sensamagic comma"),
	i(246, 239, "Meshaqah's 1/4-tone"),
	i(248, 243, "tricesoprimal comma"),
	i(250, 153, "17/4-tone"),
	i(250, 243, "maximal diesis, Porcupine comma"),
	i(256, 135, "octave - major chroma"),
	i(256, 225, "Neapolitan diminished third"),
	i(256, 243, "limma, Pythagorean minor second"),
	i(256, 245, "septimal minor semitone"),
	i(256, 255, "septendecimal kleisma"),
	i(261, 256, "vicesimononal comma"),
	i(270, 161, "Kirnberger's sixth"),
	i(272, 243, "Persian whole tone"),
	i(273, 256, "Ibn Sina's minor second"),
	i(273, 272, "Tannisma"),
	i(275, 273, "Garibert comma"),
	i(289, 288, "septendecimal minor second comma"),
	i(320, 243, "grave fourth"),
	i(321, 320, "Milliet de Chales 1/4-comma"),
	i(325, 324, "marveltwin"),
	i(351, 350, "ratwolf comma"),
	i(352, 343, "supracomma"),
	i(352, 351, "minthma"),
	i(361, 360, "Dudon comma"),
	i(364, 363, "gentle comma"),
	i(375, 256, "double augmented fourth"),
	i(375, 343, "BP major semitone, minor BP chroma"),
	i(385, 384, "undecimal kleisma, Keemun comma"),
	i(400, 243, "grave major sixth"),
	i(405, 256, "wide augmented fifth"),
	i(405, 392, "greenwoodma"),
	i(441, 440, "Werckmeister's undecimal septenarian schisma"),
	i(512, 343, "3 septatones or septatonic fifth"),
	i(512, 375, "double diminished fifth"),
	i(512, 405, "narrow diminished fourth"),
	i(512, 507, "tridecimal neutral third comma"),
	i(513, 512, "undevicesimal comma, Boethius' comma"),
	i(525, 512, "Avicenna enharmonic diesis"),
	i(540, 539, "Swets' comma"),
	i(561, 560, "Tsaharuk comma"),
	i(621, 620, "owowhatsthisma"),
	i(625, 324, "octave - major diesis"),
	i(625, 512, "classic neutral third"),
	i(625, 567, "BP great semitone, major BP chroma"),
	i(640, 637, "huntma"),
	i(648, 625, "major diesis"),
	i(675, 512, "wide augmented third"),
	i(676, 675, "island comma"),
	i(686, 675, "senga"),
	i(687, 500, "11/4-tone"),
	i(715, 714, "septendecimal bridge comma"),
	i(729, 400, "acute minor seventh"),
	i(729, 512, "Pythagorean tritone"),
	i(729, 640, "acute major second"),
	i(729, 704, "undecimal major diesis"),
	i(729, 728, "squbema"),
	i(736, 729, "vicesimotertial comma"),
	i(749, 500, "ancient Chinese quasi-equal fifth"),
	i(750, 749, "ancient Chinese tempering"),
	i(800, 729, "grave whole tone"),
	i(847, 845, "Cuthbert comma"),
	i(875, 864, "keema"),
	i(896, 891, "undecimal semicomma, pentacircle"),
	i(936, 935, "ainos comma"),
	i(1001, 1000, "fairytale comma"),
	i(1024, 675, "narrow diminished sixth"),
	i(1024, 729, "Pythagorean diminished fifth"),
	i(1029, 1000, "keega"),
	i(1029, 1024, "gamelan residue"),
	i(1053, 1024, "tridecimal major diesis"),
	i(1125, 1024, "double augmented prime"),
	i(1188, 1183, "kestrel comma"),
	i(1215, 1024, "wide augmented second"),
	i(1216, 1215, "Eratosthenes' comma"),
	i(1225, 1224, "noema"),
	i(1232, 1215, "sensmus"),
	i(1280, 729, "grave minor seventh"),
	i(1288, 1287, "triaphonisma"),
	i(1331, 1323, "aphrowe"),
	i(1344, 1331, "hemimin"),
	i(1375, 1372, "moctdel"),
	i(1445, 1444, "quasi-meantone comma"),
	i(1575, 1573, "Nicola"),
	i(1716, 1715, "lummic comma"),
	i(1728, 1715, "Orwell comma"),
	i(1729, 1728, "Ramajunanisma"),
	i(1732, 1731, "approximation to 1 cent"),
	i(1875, 1024, "double augmented sixth"),
	i(2025, 1024, "2 tritones"),
	i(2048, 1125, "double diminished octave"),
	i(2048, 1215, "narrow diminished seventh"),
	i(2048, 1875, "double diminished third"),
	i(2048, 2025, "diaschisma"),
	i(2057, 2048, "Blume comma"),
	i(2058, 2057, "xenisma"),
	i(2080, 2079, "ibnsinma"),
	i(2187, 1280, "acute major sixth"),
	i(2187, 2000, "Gorgo limma"),
	i(2187, 2048, "apotome"),
	i(2187, 2176, "septendecimal comma"),
	i(2192, 2187, "Fine Structure comma"),
	i(2200, 2197, "Parizek comma, petrma"),
	i(2401, 2400, "Breedsma"),
	i(2430, 2401, "nuwell comma"),
	i(2432, 2431, "Blumeyer comma"),
	i(2560, 2187, "grave minor third"),
	i(3025, 3024, "Lehmerisma"),
	i(3125, 2916, "quasi-equal 10-tone semitone"),
	i(3125, 3072, "small diesis, magic comma"),
	i(3125, 3087, "major BP diesis, gariboh comma"),
	i(3136, 3125, "middle second comma"),
	i(3375, 2048, "double augmented fifth"),
	i(3388, 3375, "myhemiwell"),
	i(3645, 2048, "wide augmented sixth"),
	i(4000, 3969, "small septimal comma"),
	i(4000, 3993, "undecimal schisma"),
	i(4096, 2187, "Pythagorean diminished octave"),
	i(4096, 2401, "4 septatones or septatonic major sixth"),
	i(4096, 3375, "double diminished fourth"),
	i(4096, 3645, "narrow diminished third"),
	i(4096, 3993, "Alpharabian paralimma"),
	i(4096, 4095, "tridecimal schisma, Sagittal schismina"),
	i(4131, 4096, "Hunt flat 2 comma"),
	i(4225, 4224, "leprechaun comma"),
	i(4375, 4374, "ragisma"),
	i(4608, 4235, "Arabic neutral second"),
	i(5120, 5103, "Beta 5, Garibaldi comma"),
	i(5625, 4096, "double augmented third"),
	i(5632, 5625, "vishdel"),
	i(6144, 3125, "octave - small diesis"),
	i(6144, 6125, "porwell comma"),
	i(6561, 4096, "Pythagorean augmented fifth"),
	i(6561, 5120, "acute major third"),
	i(6561, 6125, "BP major link"),
	i(6561, 6250, "ripple"),
	i(6561, 6400, "Mathieu superdiesis"),
	i(6655, 6561, "Triple BP comma"),
	i(6656, 6655, "jacobin comma"),
	i(8019, 8000, "Catakleismic comma"),
	i(8192, 5625, "double diminished sixth"),
	i(8192, 6561, "Pythagorean diminished fourth"),
	i(8192, 8019, "undecimal minor diesis"),
	i(9801, 9800, "kalisma, Gauss' comma"),
	i(10125, 8192, "double augmented second"),
	i(10240, 6561, "grave minor sixth"),
	i(10648, 10647, "harmonisma"),
	i(10935, 8192, "fourth + schisma, 5-limit approximation to ET fourth"),
	i(10976, 10935, "hemimage"),
	i(10985, 10976, "cantonisma"),
	i(12376, 12375, "flashma"),
	i(14400, 14399, "sparkisma"),
	i(14641, 14580, "semicanousma"),
	i(15625, 15309, "great BP diesis"),
	i(15625, 15552, "kleisma, semicomma majeur"),
	i(16384, 10125, "double diminished seventh"),
	i(16384, 10935, "fifth - schisma, 5-limit approximation to ET fifth"),
	i(16807, 16384, "cloudy"),
	i(16875, 16384, "double augmentation diesis, Negri comma"),
	i(16875, 16807, "small BP diesis, mirkwai comma"),
	i(17496, 16807, "septimal major diesis"),
	i(18225, 16807, "minimal BP chroma"),
	i(19657, 19656, "greater harmonisma"),
	i(19683, 10000, "octave - minimal diesis"),
	i(19683, 10240, "acute major seventh"),
	i(19683, 16384, "Pythagorean augmented second"),
	i(19683, 19208, "Laquadru comma"),
	i(19683, 19600, "cataharry comma"),
	i(20000, 19683, "minimal diesis"),
	i(20480, 19683, "grave minor second"),
	i(21875, 19683, "maximal BP chroma"),
	i(23232, 23231, "lesser harmonisma"),
	i(24576, 24565, "mavka comma"),
	i(26411, 26244, "mechanism comma"),
	i(28672, 28431, "Secorian"),
	i(32768, 16875, "octave - double augmentation diesis"),
	i(32768, 19683, "Pythagorean diminished seventh"),
	i(32768, 16807, "5 septatones or septatonic diminished octave"),
	i(32805, 32768, "schisma"),
	i(33075, 32768, "mirwomo comma"),
	i(43923, 43904, "hemigail"),
	i(48013, 48000, "undevicesimal schisma"),
	i(50421, 50000, "trimyna"),
	i(52973, 50000, "Mersenne's quasi-equal semitone"),
	i(59049, 32768, "Pythagorean augmented sixth"),
	i(59049, 57344, "Harrison's comma"),
	i(64827, 64000, "Squalentine"),
	i(65536, 32805, "octave - schisma"),
	i(65536, 59049, "Pythagorean diminished third"),
	i(65536, 64827, "Saquadru comma"),
	i(65536, 65219, "orgonisma"),
	i(65625, 65536, "horwell comma"),
	i(78125, 73728, "Woolhouse semitone"),
	i(78732, 78125, "medium semicomma, Sensi comma"),
	i(83349, 78125, "BP minor link"),
	i(100000, 99873, "Fine Structure schisma"),
	i(118098, 117649, "stearnsma"),
	i(123201, 123200, "chalmersia"),
	i(131072, 130321, "Hunt 19-cycle comma"),
	i(131769, 131072, "Alpharabian comma"),
	i(147456, 78125, "Woolhouse major seventh"),
	i(151263, 151250, "odiheim"),
	i(160083, 160000, "bilozogugu"),
	i(177147, 131072, "Pythagorean augmented third"),
	i(177147, 175000, "secanticornisma"),
	i(177147, 175616, "Latriru comma"),
	i(179200, 177147, "tolerma"),
	i(194481, 194480, "supraminor scintillisma"),
	i(234375, 234256, "sesdecal"),
	i(235298, 234375, "triwellisma"),
	i(250047, 250000, "Landscape comma"),
	i(262144, 177147, "Pythagorean diminished sixth"),
	i(262144, 253125, "Passion comma"),
	i(264627, 262144, "Betarabian comma"),
	i(321489, 320000, "varunisma"),
	i(390625, 196608, "octave - Würschmidt's comma"),
	i(390625, 373248, "doublewide"),
	i(390625, 388962, "dimcomp comma"),
	i(393216, 390625, "Würschmidt's comma"),
	i(413343, 390625, "BP small link"),
	i(420175, 419904, "wizma"),
	i(458752, 455625, "mistisma"),
	i(531441, 262144, "Pythagorean augmented seventh"),
	i(531441, 500000, "wronecki"),
	i(531441, 524288, "Pythagorean comma, ditonic comma"),
	i(537824, 531441, "Saquinzo comma"),
	i(589824, 588245, "hewuermera"),
	i(823543, 819200, "quince"),
	i(823543, 820125, "complementary BP diesis"),
	i(839808, 823543, "Sepru comma"),
	i(1048576, 531441, "Pythagorean diminished ninth"),
	i(1594323, 1048576, "Pythagorean double augmented fourth"),
	i(1594323, 1562500, "Unicorn comma"),
	i(1600000, 1594323, "Amity comma, kleisma - schisma"),
	i(1638400, 1594323, "Immunity comma"),
	i(1771561, 1771470, "parimo"),
	i(1990656, 1953125, "Valentine comma"),
	i(2097152, 1594323, "Pythagorean double diminished fifth"),
	i(2097152, 2083725, "bronzisma"),
	i(2097152, 2093663, "bean"),
	i(2100875, 2097152, "rainy comma"),
	i(2109375, 2097152, "semicomma, Fokker's comma"),
	i(3294225, 3294172, "ultimo"),
	i(4782969, 4194304, "Pythagorean double augmented prime"),
	i(4802000, 4782969, "Canou comma"),
	i(5000000, 4782969, "sevond"),
	i(8388608, 4782969, "Pythagorean double diminished octave"),
	i(9765625, 9565938, "Fifives comma"),
	i(10077696, 9765625, "mynic"),
	i(14348907, 8388608, "Pythagorean double augmented fifth"),
	i(16777216, 14348907, "Pythagorean double diminished fourth"),
	i(29360128, 29296875, "Freivald comma"),
	i(33554432, 33480783, "Beta 2, septimal schisma"),
	i(34171875, 33554432, "Ampersand's comma"),
	i(43046721, 33554432, "Pythagorean double augmented second"),
	i(43046721, 41943040, "Lalagu comma"),
	i(48828125, 47775744, "Sycamore comma"),
	i(51018336, 48828125, "nusecond"),
	i(67108864, 43046721, "Pythagorean double diminished seventh"),
	i(67108864, 66430125, "Misty comma, diaschisma - schisma"),
	i(78125000, 78121827, "euzenius"),
	i(129140163, 67108864, "Pythagorean double augmented sixth"),
	i(129140163, 128000000, "graviton, gravity comma"),
	i(131072000, 129140163, "roda"),
	i(134217728, 119574225, "whole tone - 2 schismas, 5-limit approximation to ET whole tone"),
	i(134217728, 129140163, "Pythagorean double diminished third"),
	i(134217728, 133984375, "vorwell comma"),
	i(201768035, 201326592, "wadisma"),
	i(387420489, 268435456, "Pythagorean double augmented third"),
	i(390625000, 387420489, "quartonic comma"),
	i(536870912, 387420489, "Pythagorean double diminished sixth"),
	i(645700815, 645657712, "starscape"),
	i(854296875, 843308032, "Blackjack comma"),
	i(1162261467, 536870912, "Pythagorean double augmented seventh"),
	i(1162261467, 1073741824, "Pythagorean-19 comma"),
	i(1220703125, 1162261467, "Trithagorean comma"),
	i(1220703125, 1207959552, "ditonma"),
	i(1224440064, 1220703125, "parakleisma"),
	i(6115295232, 6103515625, "Vishnu comma"),
	i(34359738368, 31381059609, "Pythagorean-22 comma"),
	i(274877906944, 274658203125, "semithirds comma"),
	i(7629394531250, 7625597484987, "ennealimmal comma"),
	i(19073486328125, 19042491875328, "'19-tone' comma"),
	i(450359962737049600, 450283905890997363, "monzisma"),
	//i(36893488147419103232, 36472996377170786403, "'41-tone' comma"),
	//i(19383245667680019896796723, 19342813113834066795298816, "Mercator's comma"),
];

