#[repr(C)]
pub struct SimpleData {
    pub value: i32,
    pub flag: bool,
}

#[no_mangle]
pub extern "C" fn modify_simple_data(data: *mut SimpleData) {
    // Function to modify SimpleData structure
    if !data.is_null() {
        unsafe {
            (*data).value += 1; // Example modification
            (*data).flag = true; // Example modification
        }
    }
}

fn main() {
    // Example usage of SimpleData and modify_simple_data
    let mut data = SimpleData {
        value: 0,
        flag: false,
    };

    // Call the function to modify the data
    modify_simple_data(&mut data as *mut _);

    // Print the modified data
    println!("Value: {}, Flag: {}", data.value, data.flag);
}
