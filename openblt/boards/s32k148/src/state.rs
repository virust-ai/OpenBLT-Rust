#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BootloaderState {
    Entry,              // Initial state after reset
    Idle,               // Waiting for programming request
    Programming,        // Active programming state
    UserProgramActive,  // Running user application
    Error,             // Error state
}

pub struct StateMachine {
    state: BootloaderState,
    backdoor_timeout: u32,
    checksum_valid: bool,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: BootloaderState::Entry,
            backdoor_timeout: 500, // 500ms default timeout
            checksum_valid: false,
        }
    }

    pub fn transition_to(&mut self, new_state: BootloaderState) {
        self.state = new_state;
    }

    pub fn current_state(&self) -> BootloaderState {
        self.state
    }

    pub fn is_backdoor_open(&self) -> bool {
        self.state == BootloaderState::Entry || self.state == BootloaderState::Idle
    }

    pub fn set_checksum_valid(&mut self, valid: bool) {
        self.checksum_valid = valid;
    }

    pub fn is_checksum_valid(&self) -> bool {
        self.checksum_valid
    }
} 
