#[derive(Debug, Clone, Copy, Default)]
pub struct IClientMode {
    pub interface_address: usize,
}

#[allow(dead_code)]
impl IClientMode {
    pub fn new(base_client_address: *const u8) -> Self {
        const INDEX: usize = 10;

        let pinterface_address = unsafe {
            let virtual_function_table = *(base_client_address as *const usize); //Reads the address of the virtual function table
            let function_address = virtual_function_table + std::mem::size_of::<usize>() * INDEX; //Adds <pointer size> multiplied by the INDEX value of the virtual function
            *(function_address as *const usize) + 5 //Skips 5 bytes to get the actual address of ClientMode interface
        } as *const u8;

        Self {
            interface_address: pinterface_address as usize,
        }
    }
}
