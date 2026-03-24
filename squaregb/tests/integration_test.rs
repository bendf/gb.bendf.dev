use squaregb::Machine;

#[test]
fn it_adds_two_integers() {
    let machine = Machine::new();
    let rom: [u8; 1] = [0x1];

    // Something like this.
    machine.set_rom(&rom);
    machine.set_pc(0x1000);

    machine.eval(1);

    assert_eq!(machine.get_reg_a(), 10);
}
