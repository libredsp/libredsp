/* Reads raw interleaved stereo 16 bit little-endian PCM from stdin and prints the DOA.
 * Any program can feed it, for example:
 *
 *     stty -F /dev/ttyUSB0 921600 raw -echo
 *     cat /dev/ttyUSB0 | cargo run --release --example doa_autocorrelation_based
 */
use libredsp::doa::autocorrelation_based::{DoaConfig, DoaEstimator};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let config = DoaConfig::default();
    let mut estimator = DoaEstimator::new(config);

    /* One hop = hop_size frames * 2 channels * 2 bytes. */
    let mut buf = vec![0u8; config.hop_size * 4];
    let mut stdin = std::io::stdin().lock();

    eprintln!("Waiting for audio on stdin...");

    while stdin.read_exact(&mut buf).is_ok() {
        let samples: Vec<i16> = buf
            .chunks_exact(2)
            .map(|b| i16::from_le_bytes([b[0], b[1]]))
            .collect();

        for e in estimator.push_interleaved_i16(&samples) {
            match e.angle_degrees {
                Some(a) => print!("DOA: {:+7.2} degrees ({:.1} dBFS)       \r", a, e.dbfs),
                None => print!("INACTIVE ({:.1} dBFS)       \r", e.dbfs),
            }
            std::io::stdout().flush()?;
        }
    }
    Ok(())
}
