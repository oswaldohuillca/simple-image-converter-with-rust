use image::*;
use std::path::Path;
use webp::*;

// si el ancho de la imagen es mayor a 1920, se redimensiona a 1920.
fn resize_image(img: DynamicImage) -> DynamicImage {
    // Obtiene las dimensiones de la imagen.
    // Gets the dimensions of the image.
    let (w, h) = img.dimensions();

    if w > 1920 {
        // Redimensiona la imagen.
        // Resize the image.
        let size_factor = 1920.0 / w as f32;
        return img.resize(
            (w as f32 * size_factor) as u32,
            (h as f32 * size_factor) as u32,
            imageops::FilterType::Lanczos3,
        );
    }

    let size_factor = 1.0;

    return img.resize(
        (w as f32 * size_factor) as u32,
        (h as f32 * size_factor) as u32,
        imageops::FilterType::Lanczos3,
    );
}

fn main() {
    let quality = 80.0;
    let input_image = String::from("input.jpg");
    let output_image = String::from("output");

    // Representa una imagen que ha sido abierta desde un archivo.
    // Represents an image that has been opened from a file.
    let img = image::open(input_image).unwrap();

    // Crea una imagen dinámica.
    // Creates a dynamic image.
    let img: DynamicImage = resize_image(img);

    // Crea un codificador a partir de la imagen.
    // Creates an encoder from the image.
    let enconder: Encoder = Encoder::from_image(&img).unwrap();

    // Codifica la imagen en formato webp.
    let webp: WebPMemory = enconder.encode(quality);

    // Crea un archivo de salida.
    // Creates an output file.
    let output_path = Path::new(&output_image).with_extension("webp");

    // Escribe el archivo de salida.
    // Writes the output file.
    match std::fs::write(&output_path, &*webp) {
        Ok(_) => println!("Archivo escrito con éxito."),
        Err(e) => eprintln!("Ocurrió un error al escribir el archivo: {}", e),
    }
}
