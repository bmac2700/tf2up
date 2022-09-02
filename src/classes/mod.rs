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
