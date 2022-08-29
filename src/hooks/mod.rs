pub unsafe fn get_virtual(class: *const u8, index: usize) -> *const u8 {
    let virtual_functions_table = *(*((*(class as *const usize)) as *const usize) as *const usize); //Reads into the address 3 times to get the virtual function table address

    let virtual_function_address =
        *((virtual_functions_table + std::mem::size_of::<usize>() * index) as *const usize); //Adds gets the pointer to the wanted virtual function

    return virtual_function_address as *const u8;
}

pub mod detour;
