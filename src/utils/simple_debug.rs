use std::env;

pub struct SimpleDebug {
    level: u8,
}

impl SimpleDebug {
    pub fn new() -> Self {
        let level = env::var("VORTEX_DEBUG")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u8>()
            .unwrap_or(1);
        
        Self { level }
    }
    
    pub fn log(&self, level: u8, module: &str, message: &str) {
        if level <= self.level {
            let prefix = match level {
                0 => "❌", // Error
                1 => "⚠️", // Warning
                2 => "ℹ️", // Info
                3 => "🔍", // Debug
                4 => "🔬", // Trace
                _ => "📝", // Other
            };
            println!("{} [{}] {}", prefix, module, message);
        }
    }
    
    pub fn error(&self, module: &str, message: &str) {
        self.log(0, module, message);
    }
    
    pub fn warning(&self, module: &str, message: &str) {
        self.log(1, module, message);
    }
    
    pub fn info(&self, module: &str, message: &str) {
        self.log(2, module, message);
    }
    
    pub fn debug(&self, module: &str, message: &str) {
        self.log(3, module, message);
    }
    
    pub fn trace(&self, module: &str, message: &str) {
        self.log(4, module, message);
    }
}

// Global debug instance
static mut DEBUG_INSTANCE: Option<SimpleDebug> = None;

pub fn init_debug() {
    unsafe {
        DEBUG_INSTANCE = Some(SimpleDebug::new());
    }
}

pub fn get_debug() -> &'static SimpleDebug {
    unsafe {
        if DEBUG_INSTANCE.is_none() {
            init_debug();
        }
        DEBUG_INSTANCE.as_ref().unwrap()
    }
}

// Convenience functions
pub fn debug_error(module: &str, message: &str) {
    get_debug().error(module, message);
}

pub fn debug_warning(module: &str, message: &str) {
    get_debug().warning(module, message);
}

pub fn debug_info(module: &str, message: &str) {
    get_debug().info(module, message);
}

pub fn debug_debug(module: &str, message: &str) {
    get_debug().debug(module, message);
}

pub fn debug_trace(module: &str, message: &str) {
    get_debug().trace(module, message);
}
