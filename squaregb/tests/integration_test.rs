use squaregb::Machine;
use squaregb::R8::*;

#[test]
fn it_adds_two_integers() {
    let mut machine = Machine::new();

    let binary = [0b00_000_110, 0x06, 0b00_111_110, 0x04, 0b1000_0000];

    // Something like this.
    machine.set_memory(0x0, &binary);

    machine.set_pc(0x0000);

    machine.eval(1);
    assert_eq!(machine.get_r8(B), 6);
    machine.eval(1);
    assert_eq!(machine.get_r8(A), 4);
    machine.eval(1);
    assert_eq!(machine.get_r8(A), 10);
}
