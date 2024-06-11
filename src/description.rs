
pub trait Method {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

pub struct Asm {}
pub struct Create_fiber {}
pub struct Create_process {}
pub struct Create_remote_thread {}
pub struct Create_remote_thread_native {}
pub struct Create_thread {}
pub struct Create_thread_native {}
pub struct Early_bird {}

pub struct Etwp_create_etw_thread {}

pub struct Memmap2_transmute {}

pub struct Module_stomping {}

pub struct Nt_queue_apc_thread_ex_local {}

pub struct Rtl_create_user_thread {}


impl Asm {
    pub fn new() -> Self {
        Asm {}
    }
}

impl Create_fiber {
    pub fn new() -> Self {
        Create_fiber {}
    }
}

impl Create_process {
    pub fn new() -> Self {
        Create_process {}
    }
}

impl Create_remote_thread {
    pub fn new() -> Self {
        Create_remote_thread {}
    }
}

impl Create_remote_thread_native {
    pub fn new() -> Self {
        Create_remote_thread_native {}
    }
}


impl Create_thread {
    pub fn new() -> Self {
        Create_thread {}
    }
}

impl Create_thread_native {
    pub fn new() -> Self {
        Create_thread_native {}
    }
}

impl Early_bird {
    pub fn new() -> Self {
        Early_bird {}
    }
}

impl Etwp_create_etw_thread {
    pub fn new() -> Self {
        Etwp_create_etw_thread {}
    }
}

impl Memmap2_transmute {
    pub fn new() -> Self {
        Memmap2_transmute {}
    }
}

impl Module_stomping {
    pub fn new() -> Self {
        Module_stomping {}
    }
}

impl Nt_queue_apc_thread_ex_local {
    pub fn new() -> Self {
        Nt_queue_apc_thread_ex_local {}
    }
}

impl Rtl_create_user_thread {
    pub fn new() -> Self {
        Rtl_create_user_thread {}
    }
}


impl Method for Asm {
    fn name(&self) -> String {
        String::from("asm")
    }

    fn description(&self) -> String {
        String::from("使用内联汇编直接在代码段中执行SHELLCODE")
    }
}

impl Method for Create_fiber {
    fn name(&self) -> String {
        String::from("create_fiber")
    }

    fn description(&self) -> String {
        String::from("将当前线程转换为纤程，分配内存，并跳转至SHELLCODE执行。")
    }
}

impl Method for Create_process {
    fn name(&self) -> String {
        String::from("create_process")
    }

    fn description(&self) -> String {
        String::from("创建一个处于暂停状态的进程，注入SHELLCODE，并修改入口点来执行SHELLCODE")
    }
}

impl Method for Create_remote_thread {
    fn name(&self) -> String {
        String::from("create_remote_thread")
    }

    fn description(&self) -> String {
        String::from("默认注入 explorer.exe，远程分配内存并创建线程来执行SHELLCODE")
    }
}

impl Method for Create_remote_thread_native {
    fn name(&self) -> String {
        String::from("create_remote_thread_native")
    }

    fn description(&self) -> String {
        String::from("与 create_remote_thread 相似，但使用原生系统调用而不是通过高级库。")
    }
}

impl Method for Create_thread {
    fn name(&self) -> String {
        String::from("create_thread")
    }

    fn description(&self) -> String {
        String::from("在本地进程内部创建一个新线程来执行SHELLCODE。")
    }
}

impl Method for Create_thread_native {
    fn name(&self) -> String {
        String::from("create_thread_native")
    }

    fn description(&self) -> String {
        String::from("类似于 create_thread，但不依赖 windows-sys crate，而是直接使用系统调用。")
    }
}

impl Method for Early_bird {
    fn name(&self) -> String {
        String::from("early_bird")
    }

    fn description(&self) -> String {
        String::from("创建一个新进程（通常是 svchost.exe），注入并执行SHELLCODE。")
    }
}

impl Method for Etwp_create_etw_thread {
    fn name(&self) -> String {
        String::from("etwp_create_etw_thread")
    }

    fn description(&self) -> String {
        String::from("使用ETW线程创建功能来执行SHELLCODE")
    }
}

impl Method for Memmap2_transmute {
    fn name(&self) -> String {
        String::from("memmap2_transmute")
    }

    fn description(&self) -> String {
        String::from("使用内存映射文件技术分配执行内存，并将内存地址转换为函数指针来执行SHELLCODE")
    }
}

impl Method for Module_stomping {
    fn name(&self) -> String {
        String::from("module_stomping")
    }

    fn description(&self) -> String {
        String::from("替换已加载模块的入口点，执行远程进程中的SHELLCODE。")
    }
}

impl Method for Nt_queue_apc_thread_ex_local {
    fn name(&self) -> String {
        String::from("nt_queue_apc_thread_ex_local")
    }

    fn description(&self) -> String {
        String::from("在本地线程上排队一个APC来执行SHELLCODE")
    }
}

impl Method for Rtl_create_user_thread {
    fn name(&self) -> String {
        String::from("rtl_create_user_thread")
    }

    fn description(&self) -> String {
        String::from("使用 RtlCreateUserThread 函数远程创建线程来执行SHELLCODE。")
    }
}

pub fn all_loader_method() -> Vec<Box<dyn Method>> {
    return vec![
        Box::new(Asm::new()),
        Box::new(Create_fiber::new()),
        Box::new(Create_process::new()),
        Box::new(Create_remote_thread::new()),
        Box::new(Create_remote_thread_native::new()),
        Box::new(Create_thread::new()),
        Box::new(Create_thread_native::new()),
        Box::new(Early_bird::new()),
        Box::new(Etwp_create_etw_thread::new()),
        Box::new(Memmap2_transmute::new()),
        Box::new(Module_stomping::new()),
        Box::new(Nt_queue_apc_thread_ex_local::new()),
        Box::new(Rtl_create_user_thread::new()),
    ];
}

pub fn method() {
    let methods = all_loader_method();
    print!("all loader method\n");
    for method in methods {
        print!("{:<40} {}\n", method.name(), method.description());
    }
}