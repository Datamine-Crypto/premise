use patterns_macros::{because, source};

pub struct BitmapFileFormat;
source!(
    BitmapFileFormat,
    "the Windows bitmap file format, its outer and inner headers and the order of their fields"
);

pub const CHANNELS: usize = 3;
because!(CHANNELS, BitmapFileFormat, "the bytes one pixel takes in a saved image, one for each of blue green and red, since a picture of a screen keeps colour and has no use for transparency");

pub const CHANNELS_WITH_ALPHA: usize = 4;
because!(CHANNELS_WITH_ALPHA, BitmapFileFormat, "the bytes one pixel takes as a graphics device hands it back, the three colours and an alpha that means nothing once the frame is flat");

const ROW_BLOCK: usize = 4;
because!(ROW_BLOCK, BitmapFileFormat, "the byte multiple every row of a bitmap is padded up to, which the format demands and which is the reason a saved frame comes out sheared when a writer forgets it");

const FILE_HEADER: u32 = 14;
because!(FILE_HEADER, BitmapFileFormat, "the bytes of the outer header, holding the signature, the length of the whole file and the offset the pixels start at");

const INFO_HEADER: u32 = 40;
because!(INFO_HEADER, BitmapFileFormat, "the bytes of the inner header, holding the width, the height, the plane count, the depth and the compression; the oldest of several sizes the format allows and the one every viewer reads");

const PIXELS_AT: u32 = FILE_HEADER + INFO_HEADER;

const SIGNATURE_FIRST: u8 = 66;
because!(SIGNATURE_FIRST, BitmapFileFormat, "the letter B that opens every bitmap, written as its code point because a byte string is a literal and a literal here would state the fact a second time");

const SIGNATURE_SECOND: u8 = 77;
because!(SIGNATURE_SECOND, BitmapFileFormat, "the letter M that completes the two byte signature a bitmap opens with, written as its code point because a byte string is a literal and one here would state the fact a second time");

const PLANES: u16 = 1;
because!(PLANES, BitmapFileFormat, "the single plane a bitmap has, a field the format kept from a time when a device stored each colour separately, and which every reader still expects to find");

const DEPTH: u16 = 24;
because!(DEPTH, BitmapFileFormat, "the bits one pixel takes in the file, eight for each colour, the one depth that needs no palette and that every viewer opens");

fn padded_row(wide: usize) -> usize {
    let bare = wide * CHANNELS;
    bare + (ROW_BLOCK - bare % ROW_BLOCK) % ROW_BLOCK
}
because!(padded_row, "the bytes one row of the saved image occupies once it is padded, kept apart from the writing so the size in the header and the padding after each row cannot disagree");

pub fn without_alpha(pixels: &[u8]) -> Vec<u8> {
    pixels
        .chunks(CHANNELS_WITH_ALPHA)
        .flat_map(|p| p.iter().take(CHANNELS).copied())
        .collect()
}
because!(without_alpha, "a frame from a device narrowed to the colours a file keeps, so a caller hands over what it was given rather than counting bytes at the call");

pub fn rows_flipped(pixels: &[u8], stride: usize) -> Vec<u8> {
    if stride == 0 {
        return Vec::new();
    }
    pixels.chunks(stride).rev().flatten().copied().collect()
}
because!(rows_flipped, "a frame turned over, for a device whose first row is the bottom of the screen rather than the top, which is the difference between the two conventions and nothing else");

pub fn bitmap(pixels: &[u8], wide: usize, high: usize) -> Vec<u8> {
    let row = padded_row(wide);
    let body = row * high;
    let size = PIXELS_AT as usize + body;
    let mut out: Vec<u8> = Vec::with_capacity(size);
    out.push(SIGNATURE_FIRST);
    out.push(SIGNATURE_SECOND);
    out.extend_from_slice(&(size as u32).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&PIXELS_AT.to_le_bytes());
    out.extend_from_slice(&INFO_HEADER.to_le_bytes());
    out.extend_from_slice(&(wide as i32).to_le_bytes());
    out.extend_from_slice(&(high as i32).to_le_bytes());
    out.extend_from_slice(&PLANES.to_le_bytes());
    out.extend_from_slice(&DEPTH.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(body as u32).to_le_bytes());
    out.extend_from_slice(&0i32.to_le_bytes());
    out.extend_from_slice(&0i32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    let line = wide * CHANNELS;
    for y in (0..high).rev() {
        let from = y * line;
        let taken = pixels.get(from..from + line).unwrap_or(&[]);
        for pixel in taken.chunks(CHANNELS) {
            out.extend(pixel.iter().rev().copied());
        }
        out.resize(out.len() + row.saturating_sub(taken.len()), 0);
    }
    out
}
because!(bitmap, "a frame of colours as the bytes of a file any desktop opens, holding in one place the row padding, the bottom upward row order and the blue before red byte order that a viewer expects and that nothing about the frame itself would tell you");

pub fn saved(path: &str, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}
because!(saved, "bytes put on the disk, the one place the library touches a file, so a binding that wants to keep something says where and not how, and a failure comes back as a value rather than ending the program");

pub fn shot(path: &str, pixels: &[u8], wide: usize, high: usize) -> Result<(), String> {
    saved(path, &bitmap(pixels, wide, high))
}
because!(shot, "a frame written to a file in one call, the whole of what a binding needs to keep a picture of what it drew, so the two halves are never composed wrongly at the call");
