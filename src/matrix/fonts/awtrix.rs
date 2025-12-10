/**
** conversion from https://github.com/Blueforcer/awtrix3/blob/main/src/AwtrixFont.h
** original license text below
**
** The original 3x5 font is licensed under the 3-clause BSD license:
**
** Copyright 1999 Brian J. Swetland
** Copyright 1999 Vassilii Khachaturov
** Portions (of vt100.c/vt100.h) copyright Dan Marks
** Modifications for Awtrix for improved readability and LaMetric Style, 2023 Blueforcer
** Cyrillic font for Awtrix by 10der (Oleg Denisenko) /Ukraine/
** Cyrillic font tests by megadimich (Dmytro Sudakevych) /Ukraine/
** All rights reserved.
**
** Redistribution and use in source and binary forms, with or without

** All rights reserved.
**
** Redistribution and use in source and binary forms, with or without
** modification, are permitted provided that the following conditions
** are met:
** 1. Redistributions of source code must retain the above copyright
**    notice, this list of conditions, and the following disclaimer.
** 2. Redistributions in binary form must reproduce the above copyright
**    notice, this list of conditions, and the following disclaimer in the
**    documentation and/or other materials provided with the distribution.
** 3. The name of the authors may not be used to endorse or promote products
**    derived from this software without specific prior written permission.
**
** THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
** IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
** OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
** IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
** INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
** NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
** DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
** THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
** (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
** THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
**
** Modifications to TomThumb for improved readability are from Robey Pointer,
** see:
** http://robey.lag.net/2010/01/23/tiny-monospace-font.html
**
** Massive modifications in 2018-2023 for Awtrix for improved readability by Blueforcer
**
** The original author does not have any objection to relicensing of Robey
** Pointer's modifications (in this file) in a more permissive license.  See
** the discussion at the above blog, and also here:
** http://opengameart.org/forumtopic/how-to-submit-art-using-the-3-clause-bsd-license
**
** Feb 21, 2016: Conversion from Linux BDF --> Adafruit GFX font,
** with the help of this Python script:
** https://gist.github.com/skelliam/322d421f028545f16f6d
** William Skellenger (williamj@skellenger.net)
** Twitter: @skelliam
**
**/
use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor, text::renderer::TextRenderer, Pixel};
use phf::phf_map;

pub struct AwtrixGlyph {
    pub width: u8,
    pub height: u8,
    pub advance: u8,
    pub x_offset: i8,
    pub y_offset: i8,
    pub bitmap: &'static [u8],
}

static AWTRIX_GLYPHS: phf::Map<char, AwtrixGlyph> = phf_map! {
    ' ' => AwtrixGlyph {
        width: 8,
        height: 1,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x00],
    },
    '!' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x80, 0x00, 0x80],
    },
    '"' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0],
    },
    '#' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xE0, 0xA0, 0xE0, 0xA0],
    },
    '$' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xC0, 0x60, 0xC0, 0x40],
    },
    '%' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x20, 0x40, 0x80, 0xA0],
    },
    '&' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xC0, 0xE0, 0xA0, 0x60],
    },
    '\'' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80],
    },
    '(' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0x80, 0x80, 0x40],
    },
    ')' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x40, 0x40, 0x40, 0x80],
    },
    '*' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x40, 0xA0],
    },
    '+' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x40, 0xE0, 0x40],
    },
    ',' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 3,
        x_offset: 0,
        y_offset: -1,
        bitmap: &[0x40, 0x80],
    },
    '-' => AwtrixGlyph {
        width: 8,
        height: 1,
        advance: 4,
        x_offset: 0,
        y_offset: -3,
        bitmap: &[0xE0],
    },
    '.' => AwtrixGlyph {
        width: 8,
        height: 1,
        advance: 2,
        x_offset: 0,
        y_offset: -1,
        bitmap: &[0x80],
    },
    '/' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x20, 0x40, 0x80, 0x80],
    },
    '0' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xA0, 0xA0, 0xE0],
    },
    '1' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xC0, 0x40, 0x40, 0xE0],
    },
    '2' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x20, 0xE0, 0x80, 0xE0],
    },
    '3' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x20, 0xE0, 0x20, 0xE0],
    },
    '4' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xE0, 0x20, 0x20],
    },
    '5' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0xE0, 0x20, 0xE0],
    },
    '6' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0xE0, 0xA0, 0xE0],
    },
    '7' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x20, 0x20, 0x20, 0x20],
    },
    '8' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xE0, 0xA0, 0xE0],
    },
    '9' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xE0, 0x20, 0xE0],
    },
    ':' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 2,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x80, 0x00, 0x80],
    },
    ';' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 3,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x40, 0x00, 0x40, 0x80],
    },
    '<' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x40, 0x80, 0x40, 0x20],
    },
    '=' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xE0, 0x00, 0xE0],
    },
    '>' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x40, 0x20, 0x40, 0x80],
    },
    '?' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x20, 0x40, 0x00, 0x40],
    },
    '@' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xA0, 0xE0, 0x80, 0x60],
    },
    'A' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xA0, 0xE0, 0xA0, 0xA0],
    },
    'B' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xA0, 0xC0, 0xA0, 0xC0],
    },
    'C' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xA0, 0x80, 0xA0, 0x40],
    },
    'D' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xA0, 0xA0, 0xA0, 0xC0],
    },
    'E' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0xE0, 0x80, 0xE0],
    },
    'F' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0xE0, 0x80, 0x80],
    },
    'G' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x80, 0xA0, 0xA0, 0x60],
    },
    'H' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xE0, 0xA0, 0xA0],
    },
    'I' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x80, 0x80, 0x80],
    },
    'J' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x20, 0x20, 0xA0, 0x40],
    },
    'K' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xC0, 0xA0, 0xA0],
    },
    'L' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x80, 0x80, 0xE0],
    },
    'M' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x88, 0xD8, 0xA8, 0x88, 0x88],
    },
    'N' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 5,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x90, 0xD0, 0xB0, 0x90, 0x90],
    },
    'O' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xA0, 0xA0, 0xA0, 0x40],
    },
    'P' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xC0, 0x80, 0x80],
    },
    'Q' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 5,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xA0, 0xA0, 0xA0, 0x70],
    },
    'R' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xC0, 0xA0, 0xA0],
    },
    'S' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0xE0, 0x20, 0xE0],
    },
    'T' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x40, 0x40, 0x40, 0x40],
    },
    'U' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xA0, 0xA0, 0xE0],
    },
    'V' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xA0, 0xA0, 0x40],
    },
    'W' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x88, 0x88, 0x88, 0xA8, 0x50],
    },
    'X' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0x40, 0xA0, 0xA0],
    },
    'Y' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xE0, 0x20, 0xC0],
    },
    'Z' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x20, 0x40, 0x80, 0xE0],
    },
    '[' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0x80, 0x80, 0xE0],
    },
    '\\' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x80, 0x40, 0x20],
    },
    ']' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x20, 0x20, 0x20, 0xE0],
    },
    '^' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xA0],
    },
    '_' => AwtrixGlyph {
        width: 8,
        height: 1,
        advance: 4,
        x_offset: 0,
        y_offset: -1,
        bitmap: &[0xE0],
    },
    '`' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x40],
    },
    'a' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xC0, 0x60, 0xA0, 0xE0],
    },
    'b' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0xC0, 0xA0, 0xA0, 0xC0],
    },
    'c' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0x80, 0x80, 0x60],
    },
    'd' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x60, 0xA0, 0xA0, 0x60],
    },
    'e' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0xA0, 0xC0, 0x60],
    },
    'f' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x40, 0xE0, 0x40, 0x40],
    },
    'g' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0xA0, 0xE0, 0x20, 0x40],
    },
    'h' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0xC0, 0xA0, 0xA0, 0xA0],
    },
    'i' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x00, 0x80, 0x80, 0x80],
    },
    'j' => AwtrixGlyph {
        width: 8,
        height: 6,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x00, 0x20, 0x20, 0xA0, 0x40],
    },
    'k' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0xA0, 0xC0, 0xC0, 0xA0],
    },
    'l' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x40, 0x40, 0x40, 0xE0],
    },
    'm' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xE0, 0xE0, 0xE0, 0xA0],
    },
    'n' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xC0, 0xA0, 0xA0, 0xA0],
    },
    'o' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x40, 0xA0, 0xA0, 0x40],
    },
    'p' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xC0, 0xA0, 0xA0, 0xC0, 0x80],
    },
    'q' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0xA0, 0xA0, 0x60, 0x20],
    },
    'r' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0x80, 0x80, 0x80],
    },
    's' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0xC0, 0x60, 0xC0],
    },
    't' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xE0, 0x40, 0x40, 0x60],
    },
    'u' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xA0, 0xA0, 0xA0, 0x60],
    },
    'v' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xA0, 0xA0, 0xE0, 0x40],
    },
    'w' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xA0, 0xE0, 0xE0, 0xE0],
    },
    'x' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xA0, 0x40, 0x40, 0xA0],
    },
    'y' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xA0, 0xA0, 0x60, 0x20, 0x40],
    },
    'z' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xE0, 0x60, 0xC0, 0xE0],
    },
    '{' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x40, 0x80, 0x40, 0x60],
    },
    '|' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x80, 0x80, 0x80],
    },
    '}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x40, 0x20, 0x40, 0xC0],
    },
    '~' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xC0],
    },
    '\u{007F}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xE0, 0xA0, 0xA0],
    },
    '\u{0080}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0xE0, 0xA0, 0xE0],
    },
    '\u{0081}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xA0, 0xE0, 0xA0, 0xC0],
    },
    '\u{0082}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0x80, 0x80, 0x80],
    },
    '\u{0083}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x70, 0x50, 0x50, 0x50, 0xF8],
    },
    '\u{0084}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0xC0, 0x80, 0xE0],
    },
    '\u{0085}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA8, 0xA8, 0x70, 0xA8, 0xA8],
    },
    '\u{0086}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x20, 0x40, 0x20, 0xC0],
    },
    '\u{0087}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 5,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x90, 0x90, 0xB0, 0xD0, 0x90],
    },
    '\u{0088}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 5,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x90, 0xB0, 0xD0, 0x90],
    },
    '\u{0089}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xC0, 0xA0, 0xA0],
    },
    '\u{008A}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xA0, 0xA0, 0xA0, 0xA0],
    },
    '\u{008B}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x88, 0xD8, 0xA8, 0x88, 0x88],
    },
    '\u{008C}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xE0, 0xA0, 0xA0],
    },
    '\u{008D}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xA0, 0xA0, 0xE0],
    },
    '\u{008E}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xA0, 0xA0, 0xA0],
    },
    '\u{008F}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0xE0, 0x80, 0x80],
    },
    '\u{0090}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x80, 0x80, 0x80, 0xE0],
    },
    '\u{0091}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x40, 0x40, 0x40, 0x40],
    },
    '\u{0092}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xE0, 0x20, 0xC0],
    },
    '\u{0093}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xF8, 0xA8, 0xF8, 0x20, 0x20],
    },
    '\u{0094}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0x40, 0xA0, 0xA0],
    },
    '\u{0095}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 5,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xA0, 0xA0, 0xF0],
    },
    '\u{0096}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xE0, 0x20, 0x20],
    },
    '\u{0097}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA8, 0xA8, 0xA8, 0xA8, 0xF8],
    },
    '\u{0098}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 7,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA8, 0xA8, 0xA8, 0xA8, 0xFC],
    },
    '\u{0099}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 5,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x40, 0x70, 0x50, 0x70],
    },
    '\u{009A}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x88, 0x88, 0xE8, 0xA8, 0xE8],
    },
    '\u{009B}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0xE0, 0xA0, 0xE0],
    },
    '\u{009C}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x20, 0x60, 0x20, 0xC0],
    },
    '\u{009D}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 6,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xB8, 0xA8, 0xE8, 0xA8, 0xB8],
    },
    '\u{009E}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0xA0, 0x60, 0xA0, 0xA0],
    },
    '\u{009F}' => AwtrixGlyph {
        width: 8,
        height: 7,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0xE0, 0x80, 0x80, 0x80, 0x00, 0x00],
    },
    '\u{00A0}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x80, 0xC0, 0x80, 0x60],
    },
    '\u{00A1}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x00, 0x80, 0x80, 0x80],
    },
    '\u{00A2}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xE0, 0x80, 0xE0, 0x40],
    },
    '\u{00A3}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x40, 0xE0, 0x40, 0xE0],
    },
    '\u{00A4}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x40, 0xE0, 0x40, 0xA0],
    },
    '\u{00A5}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0x40, 0xE0, 0x40],
    },
    '\u{00A6}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x00, 0x80, 0x80],
    },
    '\u{00A7}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x40, 0xA0, 0x40, 0xC0],
    },
    '\u{00A8}' => AwtrixGlyph {
        width: 8,
        height: 1,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0],
    },
    '\u{00A9}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x80, 0x60],
    },
    '\u{00AA}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xA0, 0xE0, 0x00, 0xE0],
    },
    '\u{00AB}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0x40],
    },
    '\u{00AC}' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xE0, 0x20],
    },
    '\u{00AD}' => AwtrixGlyph {
        width: 8,
        height: 1,
        advance: 3,
        x_offset: 0,
        y_offset: -3,
        bitmap: &[0xC0],
    },
    '\u{00AE}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xC0, 0xA0],
    },
    '\u{00AF}' => AwtrixGlyph {
        width: 8,
        height: 1,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0],
    },
    '\u{00B0}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xC0, 0x00],
    },
    '\u{00B1}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xE0, 0x40, 0x00, 0xE0],
    },
    '\u{00B2}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x40, 0x60],
    },
    '\u{00B3}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x60, 0xE0],
    },
    '\u{00B4}' => AwtrixGlyph {
        width: 8,
        height: 2,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80],
    },
    '\u{00B5}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0xA0, 0xA0, 0xC0, 0x80],
    },
    '\u{00B6}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xC0, 0xE0, 0xC0, 0x60],
    },
    '\u{00B7}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xE0, 0xE0, 0xE0],
    },
    '\u{00B8}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -3,
        bitmap: &[0x40, 0x20, 0xC0],
    },
    '\u{00B9}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 2,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x80],
    },
    '\u{00BA}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0xA0, 0x40, 0x00, 0xE0],
    },
    '\u{00BB}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x40, 0x80],
    },
    '\u{00BC}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x00, 0x60, 0x20],
    },
    '\u{00BD}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x80, 0x00, 0xC0, 0x60],
    },
    '\u{00BE}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xC0, 0x00, 0x60, 0x20],
    },
    '\u{00BF}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x00, 0x40, 0x80, 0xE0],
    },
    '\u{00C0}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x20, 0x40, 0xE0, 0xA0],
    },
    '\u{00C1}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0x40, 0xE0, 0xA0],
    },
    '\u{00C2}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0x40, 0xE0, 0xA0],
    },
    '\u{00C3}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xC0, 0x40, 0xE0, 0xA0],
    },
    '\u{00C4}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x40, 0xA0, 0xE0, 0xA0],
    },
    '\u{00C5}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xC0, 0xA0, 0xE0, 0xA0],
    },
    '\u{00C6}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xC0, 0xE0, 0xC0, 0xE0],
    },
    '\u{00C7}' => AwtrixGlyph {
        width: 8,
        height: 6,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x80, 0x80, 0x60, 0x20, 0x40],
    },
    '\u{00C8}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x20, 0xE0, 0xC0, 0xE0],
    },
    '\u{00C9}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0xE0, 0xC0, 0xE0],
    },
    '\u{00CA}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0xE0, 0xC0, 0xE0],
    },
    '\u{00CB}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0xE0, 0xC0, 0xE0],
    },
    '\u{00CC}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x20, 0xE0, 0x40, 0xE0],
    },
    '\u{00CD}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0xE0, 0x40, 0xE0],
    },
    '\u{00CE}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0xE0, 0x40, 0xE0],
    },
    '\u{00CF}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0xE0, 0x40, 0xE0],
    },
    '\u{00D0}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0xA0, 0xE0, 0xA0, 0xC0],
    },
    '\u{00D1}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x60, 0xA0, 0xE0, 0xA0],
    },
    '\u{00D2}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x20, 0xE0, 0xA0, 0xE0],
    },
    '\u{00D3}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0xE0, 0xA0, 0xE0],
    },
    '\u{00D4}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0xE0, 0xA0, 0xE0],
    },
    '\u{00D5}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x60, 0xE0, 0xA0, 0xE0],
    },
    '\u{00D6}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0xE0, 0xA0, 0xE0],
    },
    '\u{00D7}' => AwtrixGlyph {
        width: 8,
        height: 3,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0xA0, 0x40, 0xA0],
    },
    '\u{00D8}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xA0, 0xE0, 0xA0, 0xC0],
    },
    '\u{00D9}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x40, 0xA0, 0xA0, 0xE0],
    },
    '\u{00DA}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x40, 0xA0, 0xA0, 0xE0],
    },
    '\u{00DB}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0xA0, 0xA0, 0xE0],
    },
    '\u{00DC}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0xA0, 0xA0, 0xE0],
    },
    '\u{00DD}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x40, 0xA0, 0xE0, 0x40],
    },
    '\u{00DE}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0xE0, 0xA0, 0xE0, 0x80],
    },
    '\u{00DF}' => AwtrixGlyph {
        width: 8,
        height: 6,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xA0, 0xC0, 0xA0, 0xC0, 0x80],
    },
    '\u{00E0}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x20, 0x60, 0xA0, 0xE0],
    },
    '\u{00E1}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0x60, 0xA0, 0xE0],
    },
    '\u{00E2}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0x60, 0xA0, 0xE0],
    },
    '\u{00E3}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xC0, 0x60, 0xA0, 0xE0],
    },
    '\u{00E4}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0x60, 0xA0, 0xE0],
    },
    '\u{00E5}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0x60, 0x60, 0xA0, 0xE0],
    },
    '\u{00E6}' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0xE0, 0xE0, 0xC0],
    },
    '\u{00E7}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0x80, 0x60, 0x20, 0x40],
    },
    '\u{00E8}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x20, 0x60, 0xE0, 0x60],
    },
    '\u{00E9}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0x60, 0xE0, 0x60],
    },
    '\u{00EA}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0x60, 0xE0, 0x60],
    },
    '\u{00EB}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0x60, 0xE0, 0x60],
    },
    '\u{00EC}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x40, 0x80, 0x80, 0x80],
    },
    '\u{00ED}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 3,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0x40, 0x40, 0x40],
    },
    '\u{00EE}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0x40, 0x40, 0x40],
    },
    '\u{00EF}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0x40, 0x40, 0x40],
    },
    '\u{00F0}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x60, 0xC0, 0x60, 0xA0, 0x60],
    },
    '\u{00F1}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x60, 0xC0, 0xA0, 0xA0],
    },
    '\u{00F2}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x20, 0x40, 0xA0, 0x40],
    },
    '\u{00F3}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x80, 0x40, 0xA0, 0x40],
    },
    '\u{00F4}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0x40, 0xA0, 0x40],
    },
    '\u{00F5}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xC0, 0x60, 0x40, 0xA0, 0x40],
    },
    '\u{00F6}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0x40, 0xA0, 0x40],
    },
    '\u{00F7}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x40, 0x00, 0xE0, 0x00, 0x40],
    },
    '\u{00F8}' => AwtrixGlyph {
        width: 8,
        height: 4,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x60, 0xE0, 0xA0, 0xC0],
    },
    '\u{00F9}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x80, 0x40, 0xA0, 0xA0, 0x60],
    },
    '\u{00FA}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x40, 0xA0, 0xA0, 0x60],
    },
    '\u{00FB}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xE0, 0x00, 0xA0, 0xA0, 0x60],
    },
    '\u{00FC}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0xA0, 0xA0, 0x60],
    },
    '\u{00FD}' => AwtrixGlyph {
        width: 8,
        height: 6,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0x20, 0x40, 0xA0, 0x60, 0x20, 0x40],
    },
    '\u{00FE}' => AwtrixGlyph {
        width: 8,
        height: 5,
        advance: 4,
        x_offset: 0,
        y_offset: -4,
        bitmap: &[0x80, 0xC0, 0xA0, 0xC0, 0x80],
    },
    '\u{00FF}' => AwtrixGlyph {
        width: 8,
        height: 6,
        advance: 4,
        x_offset: 0,
        y_offset: -5,
        bitmap: &[0xA0, 0x00, 0xA0, 0x60, 0x20, 0x40],
    },
};

pub struct AwtrixFont {
    text_color: Rgb888,
}

impl AwtrixFont {
    pub fn new(text_color: Rgb888) -> Self {
        AwtrixFont { text_color }
    }
}

impl TextRenderer for AwtrixFont {
    type Color = Rgb888;

    fn draw_string<D>(
        &self,
        text: &str,
        position: embedded_graphics::prelude::Point,
        _baseline: embedded_graphics::text::Baseline,
        target: &mut D,
    ) -> Result<embedded_graphics::prelude::Point, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        let point = {
            let mut printer = AwtrixFontInner::new();
            printer.cursor_x = position.x;
            printer.cursor_y = position.y + self.line_height() as i32;

            printer.print_str(text, &mut move |x, y| {
                target.draw_iter([Pixel(embedded_graphics::prelude::Point::new(x, y), self.text_color)]).ok();
            });
            embedded_graphics::prelude::Point::new(printer.cursor_x, printer.cursor_y)
        };

        Ok(point)
    }

    fn draw_whitespace<D>(
        &self,
        _width: u32,
        _position: embedded_graphics::prelude::Point,
        _baseline: embedded_graphics::text::Baseline,
        _target: &mut D,
    ) -> Result<embedded_graphics::prelude::Point, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        todo!("draw_whitespace not implemented")
    }

    fn measure_string(
        &self,
        text: &str,
        position: embedded_graphics::prelude::Point,
        _baseline: embedded_graphics::text::Baseline,
    ) -> embedded_graphics::text::renderer::TextMetrics {
        let mut printer = AwtrixFontInner::new();
        printer.cursor_x = position.x;
        printer.cursor_y = position.y + self.line_height() as i32;

        printer.print_str(text, &mut move |_x, _y| {});
        embedded_graphics::text::renderer::TextMetrics {
            bounding_box: embedded_graphics::primitives::Rectangle::new(
                position,
                embedded_graphics::prelude::Size::new((printer.cursor_x - position.x) as u32, self.line_height()),
            ),
            next_position: embedded_graphics::prelude::Point::new(printer.cursor_x, printer.cursor_y),
        }
    }

    fn line_height(&self) -> u32 {
        5
    }
}

pub struct AwtrixFontInner {
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub y_advance: i32,
}

impl AwtrixFontInner {
    pub fn new() -> Self {
        AwtrixFontInner { cursor_x: 0, cursor_y: 0, y_advance: 0 }
    }

    pub fn print_str<'a>(&'a mut self, s: &'a str, out: &'a mut impl FnMut(i32, i32)) {
        for c in s.chars() {
            self.print_char(c, out);
        }
    }

    pub fn print_char<'a>(&'a mut self, c: char, out: &'a mut impl FnMut(i32, i32)) {
        match c {
            '\n' => {
                self.cursor_y += self.y_advance;
                self.cursor_x = 0;
                return;
            }
            '\r' => {
                // optional: carriage return handling
                return;
            }
            _ => {}
        }

        let glyph = AWTRIX_GLYPHS.get(&c).or_else(|| AWTRIX_GLYPHS.get(&' ')).unwrap();

        let w = glyph.width as usize;
        let h = glyph.height as usize;
        let xo = glyph.x_offset as i32;
        let yo = glyph.y_offset as i32;

        let mut bits: u8 = 0;
        let mut bit: u8 = 0;
        let mut bo: usize = 0;

        for yy in 0..h {
            for xx in 0..w {
                if (bit & 7) == 0 {
                    // Load next byte every 8 pixels
                    if bo < glyph.bitmap.len() {
                        bits = glyph.bitmap[bo];
                        bo += 1;
                    } else {
                        bits = 0;
                    }
                }
                if (bits & 0x80) != 0 {
                    let x = self.cursor_x + xo + xx as i32;
                    let y = self.cursor_y + yo + yy as i32;
                    //self.matrix.draw_pixel(x, y, self.text_color);
                    out(x, y);
                }
                bits <<= 1;
                bit = bit.wrapping_add(1);
            }
        }

        self.cursor_x += glyph.advance as i32;
    }
}
