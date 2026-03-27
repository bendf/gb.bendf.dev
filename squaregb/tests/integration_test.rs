use squaregb::A;
use squaregb::Machine;

#[test]
fn it_adds_two_integers() {
    let mut machine = Machine::new();
    let rom: [u8; 65536] = [0x1; 65536];

    // Something like this.
    machine.set_memory(&rom);
    machine.set_pc(0x1000);

    machine.eval(1);

    assert_eq!(machine.get_reg(A), 10);
}
