mod cpu;

fn main() {
    let my_cpu = cpu::Cpu::new(Path::new("test.willfail",));
	println!("Wont print but its okay");
}