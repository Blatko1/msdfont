use msdfont::Msdfont;

fn main() {
    let data = include_bytes!("fonts/monserat.ttf");
    let msdfont = Msdfont::try_from_bytes(data).unwrap();
}
