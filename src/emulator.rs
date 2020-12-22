use project::Project;
use unicorn::RegisterARM;
use unicorn::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};

enum Hook {
    InstructionHook,
    FunctionHook,
    SyscallHook,
}

pub struct Emulator {
    pub unicorn: unicorn::Unicorn,
}

impl Emulator {
    pub fn new(project: &Project) -> Self {
        return Emulator {
            unicorn: unicorn::Unicorn::new(Arch::ARM, Mode::LITTLE_ENDIAN, 0).expect("failed to initialize Unicorn"),
        };
    }

    pub fn run(&self) {
        info!("Running unicorn emulator");
    }

    pub fn hook(&self, hook: Hook) {

    }
}
