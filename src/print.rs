pub fn run(){
	println!("Hello, from print module!");

	// Print with single placeholder
	println!("Number: {}", 1);

	// Print with multiple placeholders (basic formating)
	println!("{} is a {}", 1, "number");

	// Positional arguments
	println!("{0} lives in {1}, {1} is a great country.", "imbdb", "India");

	// Named arguments
	println!("{name} lives in {country}, {country} is a great country.", name = "imbdb", country = "India");

	// Placeholder traits
	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

	// Placeholder for debug trait
	println!("{:?}", (12, true, "hello"));

	// Basic math
	println!("10 + 10 = {}", 10 + 10);
}