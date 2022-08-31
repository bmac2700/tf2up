pub fn setup_netvars(client_classes: *const u8) {
    let mut client_class = client_classes;

    while !client_class.is_null() {
        let recv_table = (client_class as usize + 0xC) as *const u8;
        if !recv_table.is_null() {
            println!("looping");
        }

        unsafe {
            client_class = (*((client_class as usize + 0x10) as *const usize)) as *const u8;
        }
    }
}
