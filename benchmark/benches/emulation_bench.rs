use criterion::{Criterion, criterion_group, criterion_main};
use rand::{Rng, distr::Alphanumeric, rng};
use termio::emulator::emulation::{Emulation, VT102Emulation};
use tmui::prelude::ActionHub;
use widestring::WideString;

/// Handle string with length `10000`.
///
/// emulation_receive_data  time:   [341.55 µs 345.06 µs 349.32 µs]
fn emulation_receive_data(emulation: &mut VT102Emulation, data: &str) {
    let wstr = WideString::from_str(data);
    for &c in wstr.as_slice() {
        #[allow(clippy::useless_transmute)]
        let c = unsafe { std::mem::transmute(c) };
        emulation.receive_char(c)
    }
}

pub fn criterion_values(c: &mut Criterion) {
    let random_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(10000)
        .map(char::from)
        .collect();

    ActionHub::initialize();

    let mut emulation = Box::new(VT102Emulation::new(None));

    c.bench_function("emulation_receive_data", |b| {
        b.iter(|| emulation_receive_data(emulation.as_mut(), &random_string))
    });
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
