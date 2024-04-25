const MAXAPP_NUM: usize = 4;

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAXAPP_NUM + 1]   
}

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE]
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE]
}

static KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE] };
static USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE] };

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}