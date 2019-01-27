use clap::{App, Arg};
use delay_coord::ForwardDelayCoordinates;

fn open_file_or_stdin<'a, T: AsRef<::std::path::Path>>(
    path: &Option<T>,
    stdin: &'a ::std::io::Stdin,
) -> either::Either<::std::io::BufReader<::std::fs::File>, ::std::io::StdinLock<'a>> {
    match path {
        Some(path) => {
            let file = ::std::fs::File::open(path).unwrap();
            let reader = ::std::io::BufReader::new(file);
            either::Either::Left(reader)
        }
        None => either::Either::Right(stdin.lock()),
    }
}

fn read_data_file<R: ::std::io::BufRead>(reader: &mut R) -> Vec<Vec<f64>> {
    let mut data = Vec::new();
    loop {
        let mut buf = String::new();
        let size = reader.read_line(&mut buf).unwrap();
        if size == 0 {
            break;
        }
        let v: Vec<f64> = buf.trim().split(',')
                             .map(|s| s.parse::<f64>().unwrap())
                             .collect();
        data.push(v);
    }
    data
}

fn main() {
    let matches = App::new("delay-coordinate mapping")
                          .version("0.1.0")
                          .author("Shotaro Tsuji <Shotaro.Tsuji@gmail.com>")
                          .about("Maps time series into a delay-coordinate space")
                          .arg(Arg::with_name("delay")
                               .short("d")
                               .long("delay")
                               .value_name("DELAY")
                               .help("Sets the delay in steps")
                               .takes_value(true))
                          .arg(Arg::with_name("dimension")
                               .short("m")
                               .long("dimension")
                               .value_name("DIM")
                               .help("Sets the embedding dimension")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file")
                               .index(1))
                          .get_matches();

    let dimension = matches.value_of("dimension")
                           .expect("Embedding dimension must be specified")
                           .parse::<usize>()
                           .expect("Embedding dimension must be usize");
    let delay = matches.value_of("delay")
                       .expect("Delay must be specified")
                       .parse::<usize>()
                       .expect("Delay must be usize");

    let input = matches.value_of("INPUT");
    let stdin = ::std::io::stdin();
    let mut input = open_file_or_stdin(&input, &stdin);
    let data = read_data_file(&mut input);

    let coord = ForwardDelayCoordinates {
            dimension: dimension,
            delay: delay,
    };
    for v in coord.mapping_iter(&data).map(|p| p.to_flatten_vec()) {
        for i in 0..v.len() {
            let delim = if i == v.len()-1 { '\n' } else { ',' };
            print!("{}{}", v[i], delim);
        }
    }
}
