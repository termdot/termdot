use crate::IPC_DATA_SIZE;

#[repr(align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum IpcEvent {
    Exit,
    /// (Cols, Rows)
    SetTerminalSize(i32, i32),
    SendData([u8; IPC_DATA_SIZE], usize),
}

impl IpcEvent {
    /// Pack data to [`IpcEvent::SendData`]
    pub fn pack_data(data: String) -> Vec<IpcEvent> {
        let bytes = data.as_bytes();
        let mut events = Vec::new();
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
}

#[cfg(test)]
pub mod tests {
    use super::IpcEvent;

    #[test]
    fn test_pack_data() {
        let evt = IpcEvent::pack_data("Hello World".to_string());
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
