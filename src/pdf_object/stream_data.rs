// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use ascii85;
use flate2::{write::ZlibEncoder, Compression};
use image::{ImageFormat, ImageReader};
use maplit::hashmap;
use std::{collections::HashMap, io::Write};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use super::PdfObject;
use crate::helpers::write_all_count;

/// Standard filters for stream data.  See PDF 1.7 - 7.4
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    ASCIIHexDecode,
    ASCII85Decode,
    LZWDecode,          // params
    FlateDecode,        // params
    RunLengthDecode,
    CCITTFaxDecode,     // params
    JBIG2Decode,        // params
    DCTDecode,          // params
    JPXDecode,
    Crypt,              // params
}

impl ToString for Filter {

    fn to_string(&self) -> String {
        match self {
            Filter::ASCIIHexDecode => "ASCIIHexDecode".to_string(),
            Filter::ASCII85Decode => "ASCII85Decode".to_string(),
            Filter::LZWDecode => "LZWDecode".to_string(),
            Filter::FlateDecode => "FlateDecode".to_string(),
            Filter::RunLengthDecode => "RunLengthDecode".to_string(),
            Filter::CCITTFaxDecode => "CCITTFaxDecode".to_string(),
            Filter::JBIG2Decode => "JBIG2Decode".to_string(),
            Filter::DCTDecode => "DCTDecode".to_string(),
            Filter::JPXDecode => "JPXDecode".to_string(),
            Filter::Crypt => "Crypt".to_string(),
        }
    }

}

/// Object types.
#[derive(Debug, Clone, PartialEq)]
pub struct FilterData {
    pub filter: Filter,
    pub decode_params: HashMap<String, PdfObject>,
}

impl FilterData {

    pub fn new_ascii85() -> Self {
        Self { 
            filter: Filter::ASCII85Decode, 
            decode_params: hashmap! {},
        }
    }

    pub fn new_image(filter: Filter, _image: &image::DynamicImage) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(
            Self { 
                filter: filter, 
                decode_params: hashmap! {},
            }
        )
    }
    
}

/// Object types.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamData {
    pub length: usize,
    pub dict: HashMap<String, PdfObject>,
    pub filter_data: Vec<FilterData>,
    pub data: Vec<u8>,
}

impl ToString for StreamData {

    fn to_string(&self) -> String {
        let mut vec: Vec<u8> = Vec::new();
        self.render_stream(&mut vec).unwrap();
        String::from_utf8(vec).unwrap()
    }

}

impl Into<PdfObject> for StreamData {

    fn into(self) -> PdfObject {
        PdfObject::Stream(self)
    }

}

impl StreamData {

    pub fn render_stream(&self, writer: &mut dyn std::io::Write) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;

        // create stream dict
        let mut dict = self.dict.clone();

        // add length
        dict.insert("Length".to_string(), PdfObject::from(self.length));
        
        // add filters
        let mut filter_arr: Vec<PdfObject> = Vec::new();
        let mut decode_parms: Vec<PdfObject> = Vec::new();
        for filter in &self.filter_data {
            filter_arr.push(PdfObject::from(PdfObject::Name(filter.filter.to_string())));
            decode_parms.push(PdfObject::Dictionary(filter.decode_params.clone()));
        }
        dict.insert("Filter".to_string(), PdfObject::Array(filter_arr));
        dict.insert("DecodeParms".to_string(), PdfObject::Array(decode_parms));

        // write stream dict
        count += PdfObject::Dictionary(dict).render(writer)?;
       
        // write stream data
        count += write_all_count(writer, b"\nstream\n")?;
        count += write_all_count(writer, &self.data)?;
        count += write_all_count(writer, b"\nendstream")?;

        Ok(count)
    }


    /// Encodes stream data from an image file.
    pub fn encode_image_file(image_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // read the image
        let file = File::open(image_path)?;
        let buffer: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();
        Self::encode_image(&buffer)
    }

    /// Encodes stream data for an image.
    pub fn encode_image(image_bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // read the image
        let reader = ImageReader::new(std::io::Cursor::new(image_bytes))
            .with_guessed_format()?;
        let format = reader.format();
        let image = reader.decode()?;

        // encode the image for the specified format
        let mut stream_data = match format {
            Some(ImageFormat::Png) => Self::encode_png(&image),
            Some(ImageFormat::Jpeg) => Self::encode_jpeg(&image, image_bytes),
            // Some(ImageFormat::Gif) => Self::encode_gif(&image),
            // Some(ImageFormat::WebP) => Self::encode_webp(&image),
            // Some(ImageFormat::Pnm) => Self::encode_prm(&image),
            // Some(ImageFormat::Tiff) => Self::encode_tiff(&image),
            // Some(ImageFormat::Tga) => Self::encode_tga(&image),
            // Some(ImageFormat::Dds) => Self::encode_dds(&image),
            // Some(ImageFormat::Bmp) => Self::encode_bmp(&image),
            // Some(ImageFormat::Ico) => Self::encode_ico(&image),
            // Some(ImageFormat::Hdr) => Self::encode_hdr(&image),
            // Some(ImageFormat::OpenExr) => Self::encode_openexr(&image),
            // Some(ImageFormat::Farbfeld) => Self::encode_farbfeld(&image),
            // Some(ImageFormat::Avif) => Self::encode_avif(&image),
            // Some(ImageFormat::Qoi) => Self::encode_qoi(&image),
            // Some(ImageFormat::Pcx) => Self::encode_pcx(&image),
            _ => Err("Unsupported image format".into()),
        }?;

        // add image information
        stream_data.dict.insert("Type".to_string(), PdfObject::Name("XObject".to_string()));
        stream_data.dict.insert("Subtype".to_string(), PdfObject::Name("Image".to_string()));
        stream_data.dict.insert("Width".to_string(), PdfObject::from(image.width() as usize));
        stream_data.dict.insert("Height".to_string(), PdfObject::from(image.height() as usize));

        // add color type information
        let (color_space, bits_per_component) = match image.color() {
            image::ColorType::L8 => ("DeviceGray", 8),
            image::ColorType::La8 => ("DeviceGray", 8),
            image::ColorType::Rgb8 => ("DeviceRGB", 8),
            image::ColorType::Rgba8 => ("DeviceRGB", 8),
            image::ColorType::L16 => ("DeviceGray", 16),
            image::ColorType::La16 => ("DeviceGray", 16),
            image::ColorType::Rgb16 => ("DeviceRGB", 16),
            image::ColorType::Rgba16 => ("DeviceRGB", 16),
            image::ColorType::Rgb32F => ("DeviceRGB", 32),
            image::ColorType::Rgba32F => ("DeviceRGB", 32),
            _ => ("DeviceRGB", 0) // Unsupported color type
        };
        if bits_per_component == 0 { return Err("Unsupported color type".into()) };
        stream_data.dict.insert("ColorSpace".to_string(), PdfObject::Name(color_space.to_string()));
        stream_data.dict.insert("BitsPerComponent".to_string(), PdfObject::from(bits_per_component as usize));

        Ok(stream_data)
    }

    fn encode_png(image: &image::DynamicImage) -> Result<Self, Box<dyn std::error::Error>> {
        // setup filters
        let mut filters: Vec<FilterData> = Vec::new();

        // prepare png for pdf stream filter
        let height: usize = image.height() as usize;
        let width: usize = image.width() as usize;
        let rgb = image.to_rgb8();
        let mut raw_data = Vec::with_capacity(width * height * 3 + height);
        for row in rgb.rows() {
            raw_data.push(0); // predictor byte: 0 (None)
            for pixel in row {
                raw_data.extend_from_slice(&pixel.0);
            }
        }

        // compress png data using flate
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&raw_data)?;
        let compressed_data = encoder.finish()?;

        // add flate filter
        let mut flate_filter = FilterData::new_image(Filter::FlateDecode, image)?;
        flate_filter.decode_params.extend(hashmap!{
            "Predictor".to_string() => PdfObject::from(15),
            "Colors".to_string() => PdfObject::from(3),
            "BitsPerComponent".to_string() => PdfObject::from(8),
            "Columns".to_string() => PdfObject::from(width),
        });
        filters.push(flate_filter);

        // encode png data using ascii85
        let encoded_data = ascii85::encode(&compressed_data).as_bytes().to_vec();

        // add ascii85 filter
        filters.insert(0, FilterData::new_ascii85());
        
        Ok(StreamData {
            length: encoded_data.len(),
            dict: hashmap! {},
            filter_data: filters,
            data: encoded_data,
        })
    }

    fn encode_jpeg(image: &image::DynamicImage, bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // setup filters
        let mut filters: Vec<FilterData> = Vec::new();

        // add DCT filter
        let dct_filter = FilterData::new_image(Filter::DCTDecode, image)?;
        filters.push(dct_filter);

        // encode jpeg data using ascii85
        let encoded_data = ascii85::encode(bytes).as_bytes().to_vec();

        // add ascii85 filter
        filters.insert(0, FilterData::new_ascii85());
        
        Ok(StreamData {
            length: encoded_data.len(),
            dict: hashmap! {},
            filter_data: filters,
            data: encoded_data,
        })
    }

    // fn encode_gif(image: &image::DynamicImage) -> Result<Self, Box<dyn std::error::Error>> {
    //     // use LZWDecode to encode the image
    //     // determine the LZWDecode filter parameters per pdf object stream LZWDecode
    //     // create a stream data
    //     // set the length, filter data and data for the compressed gif image
    //     let mut encoded_data = Vec::new();
    //     image.write_to(&mut std::io::Cursor::new(&mut encoded_data), image::ImageFormat::Gif)?;

    //     Ok(StreamData {
    //         length: encoded_data.len(),
    //         filter_data: FilterData {
    //             filter: Filter::LZWDecode,
    //             params: HashMap::new(), // Add parameters if needed for the filter
    //         },
    //         data: encoded_data,
    //     })
        
    // }

}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;
    use super::*;
    use crate::resources;

    #[test]
    fn test_png() {
        let img_path = resources::get_resource_path("tests/test.png");
        let stream_data = StreamData::encode_image_file(&img_path.unwrap().to_path_buf());
        assert!(stream_data.is_ok());
        let stream_data = stream_data.unwrap();
        assert_eq!(stream_data.to_string(), resources::get_resource_string("tests/test.png.stream_data").unwrap());
    }

    #[test]
    fn test_jpeg() {
        let img_path = resources::get_resource_path("tests/test.jpg");
        let stream_data = StreamData::encode_image_file(&img_path.unwrap().to_path_buf());
        assert!(stream_data.is_ok());
        let stream_data = stream_data.unwrap();
        if false {
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.disable_cleanup(true);
            let (mut file, path) = temp_file.keep().unwrap();
            eprintln!("Temp jpeg stream file: {:?}", path);
            stream_data.render_stream(&mut file).unwrap();
        }
        assert_eq!(stream_data.to_string(), resources::get_resource_string("tests/test.jpg.stream_data").unwrap());
    }

}
