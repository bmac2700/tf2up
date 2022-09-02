pub mod cbaseentity;
pub mod cbaseplayer;
pub mod cusercmd;

#[macro_export]
macro_rules! class_member {
    ($member_name:ident, $data_type:ty, $offset:expr) => {
        paste::paste! {
            pub fn [<get_ $member_name>](&self) -> $data_type {
                let value: *const $data_type = (self.object_address as usize + $offset) as _;
                return unsafe { *value };
            }

            pub fn [<set_ $member_name>](&self, [<new_ $member_name>]: $data_type) {
                let data: *mut $data_type = (self.object_address as usize + $offset) as _;
                unsafe { *data = [<new_ $member_name>] };
            }
        }
    };
}

#[macro_export]
macro_rules! netvar {
    ($netvar_name:ident, $data_type:ty, $netvar_path:expr) => {
        paste::paste! {
            pub fn [<get_ $netvar_name>](&self, netvars: &HashMap<usize, usize>) -> $data_type {
                let offset = netvars.get(&netvar_hash($netvar_path));
                let data = (self.object_address as usize + *offset.unwrap()) as *const _;

                return unsafe {*data};
            }
        }
    };
}
