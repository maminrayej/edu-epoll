use bitflags::bitflags;

bitflags! {
    pub struct Interest: u32 {
        const READ = libc::EPOLLIN as u32;
        const WRITE = libc::EPOLLOUT as u32;
    }
}
