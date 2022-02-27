extern crate png;
extern crate cairo;

use std::{fs::File, io::BufWriter};



fn main() {
    let file = File::open("./main.png").expect("couldn't open file");

    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();

    let mut data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut data).expect("invalid PNG");

    println!("{}", info.width);
    let mut surface = cairo::ImageSurface::create_for_data(
        data.into_boxed_slice(),
        cairo::Format::ARgb32,
        info.width as i32,
        info.height as i32,
        info.line_size as i32).unwrap();

    let context = cairo::Context::new(&surface).unwrap();
    context.select_font_face("serif",cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    context.set_source_rgb(1.0,0.0,0.0);
    context.move_to(50.0,50.0);
    context.show_text("Hello Nepal");
    
    let data = surface.data().unwrap().to_vec(); 

    let file = File::create("./main-with-quotes.pdf").expect("couldn't create file");
    let buf_writer = BufWriter::new(file);

    
    let encoder  = png::Encoder::new(buf_writer, info.width, info.height);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data[..]).expect("failed to write image");

}
