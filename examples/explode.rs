use std::io::Write as _;

fn main() {
    let file = std::env::args_os().nth(1).unwrap();
    let fh = std::fs::File::open(file).unwrap();
    let mut reader = ttyrec::blocking::Reader::new(fh);

    let mut idx = 1;
    loop {
        match reader.read_frame() {
            Ok(frame) => {
                let mut fh =
                    std::fs::File::create(&format!("{}.out", idx)).unwrap();
                fh.write_all(&frame.data).unwrap();
                idx += 1;
            }
            Err(ttyrec::Error::EOF) => {
                break;
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
