use crate::IPC_DATA_SIZE;

#[repr(align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum IpcEvent {
    HeartBeat,
    Ready,
    Exit,
    /// (Cols, Rows)
    SetTerminalSize(i32, i32),
    TerminalVersion([u8; IPC_DATA_SIZE], usize),
    HostNameChanged([u8; IPC_DATA_SIZE], usize),
    SendData([u8; IPC_DATA_SIZE], usize),
}

impl IpcEvent {
    /// Pack string to [`IpcEvent::SendData`]
    pub fn pack_data(data: &str) -> Vec<IpcEvent> {
        let bytes = data.as_bytes();
        let mut events = vec![];
        let mut start = 0;

        while start < bytes.len() {
            let mut end = (start + IPC_DATA_SIZE).min(bytes.len());

            while end > start && !data.is_char_boundary(end) {
                end -= 1;
            }

            let chunk = &bytes[start..end];
            let mut array = [0u8; IPC_DATA_SIZE];
            array[..chunk.len()].copy_from_slice(chunk);

            events.push(IpcEvent::SendData(array, chunk.len()));
            start = end;
        }

        events
    }

    /// Pack string to [`IpcEvent::HostNameChanged`]
    pub fn pack_host_name(host_name: &str) -> IpcEvent {
        if host_name.len() > IPC_DATA_SIZE {
            panic!(
                "[IpcEvent::pack_host_name] host name is too long, max length is {}",
                IPC_DATA_SIZE
            );
        }

        let bytes = host_name.as_bytes();
        let start = 0;

        let mut end = (start + IPC_DATA_SIZE).min(bytes.len());

        while end > start && !host_name.is_char_boundary(end) {
            end -= 1;
        }

        let chunk = &bytes[start..end];
        let mut array = [0u8; IPC_DATA_SIZE];
        array[..chunk.len()].copy_from_slice(chunk);

        IpcEvent::HostNameChanged(array, chunk.len())
    }

    pub fn pack_terminal_version(version: &str) -> IpcEvent {
        if version.len() > IPC_DATA_SIZE {
            panic!(
                "[IpcEvent::pack_terminal_version] version is too long, max length is {}",
                IPC_DATA_SIZE
            );
        }

        let bytes = version.as_bytes();
        let start = 0;

        let mut end = (start + IPC_DATA_SIZE).min(bytes.len());

        while end > start && !version.is_char_boundary(end) {
            end -= 1;
        }

        let chunk = &bytes[start..end];
        let mut array = [0u8; IPC_DATA_SIZE];
        array[..chunk.len()].copy_from_slice(chunk);

        IpcEvent::TerminalVersion(array, chunk.len())
    }
}

#[cfg(test)]
pub mod tests {
    use super::IpcEvent;

    #[test]
    fn test_pack_data() {
        let evt = IpcEvent::pack_data("Hello World");
        assert_eq!(evt.len(), 1);

        if let IpcEvent::SendData(data, len) = evt.first().unwrap() {
            let mut data = data.to_vec();
            data.truncate(*len);
            let r = String::from_utf8(data).unwrap();
            assert_eq!(r.as_str(), "Hello World");
        } else {
            unreachable!()
        }
    }
}
