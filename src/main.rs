extern crate avirus;
#[macro_use]
extern crate clap;

use clap::App;
use clap::Arg;

use avirus::frame::Frame;
use avirus::AVI;

fn main() {
    let matches = App::new("byedeo")
        .version(crate_version!())
        .about("Data moshes videos by removing I frames")
        .arg(Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("The input file to operate on. The file must be avi container. For best results encode with ffmpeg -i input.mp4 -c:v libxvid -g 9999 output.avi")
        .takes_value(true))
        .arg(Arg::with_name("output").help("The name of the output file").index(1))
        .get_matches();

    let mut avi = AVI::new(matches.value_of("input").unwrap()).unwrap();
    let mut new_meta: Vec<Frame> = Vec::new();
    let mut last_frame = avi.frames.meta.first().unwrap();
    for (i, frame) in &mut avi.frames.meta.iter().enumerate() {
        if i > 10 {
            if frame.is_pframe() || frame.is_audioframe() {
                new_meta.push(frame.clone());
                if frame.is_pframe() {
                    last_frame = frame;
                }
            } else {
                new_meta.push(last_frame.clone());
            }
        } else {
            new_meta.push(frame.clone());
        }
    }
    avi.frames.meta = new_meta;
    avi.output(matches.value_of("output").unwrap()).unwrap();
}
