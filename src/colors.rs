use ratatui::prelude::Color                                                 ;

/// Function color_setter:
/// takes a u16 input value (this is the value of the node)
/// and returns a Color enum from the ratatui crate.
/// The default color is black, and after a certain score in a node
/// the color stays stagnant at gold.

pub fn color_setter(value: u16) -> Color                                    {
    // check value and generate rgb values based on the value
    if value == 0                                                           {
        return Color::Black                                                 ;
                                                                            }

    //write a match statement that returns a Color::RGB based on the value

    match value                                                             {
        2 => Color::Rgb                                                     {
            0: 238,
            1: 228,
            2: 218,
        },
        4 => Color::Rgb                                                     {
            0: 237,
            1: 224,
            2: 200,
        },
        8 => Color::Rgb                                                     {
            0: 242,
            1: 177,
            2: 121,
        },
        16 => Color::Rgb                                                    {
            0: 245,
            1: 149,
            2: 99,
        },
        32 => Color::Rgb                                                    {
            0: 246,
            1: 124,
            2: 96,
        },
        64 => Color::Rgb                                                    {
            0: 246,
            1: 94,
            2: 59,
        },
        128 => Color::Rgb                                                   {
            0: 237,
            1: 207,
            2: 114,
        },
        256 => Color::Rgb                                                   {
            0: 237,
            1: 204,
            2: 97,
        },
        512 => Color::Rgb                                                   {
            0: 237,
            1: 200,
            2: 80,
        },
        1024 => Color::Rgb                                                  {
            0: 237,
            1: 197,
            2: 63,
        },
        2048 => Color::Rgb                                                  {
            0: 237,
            1: 194,
            2: 46,
        },
        _ => Color::Rgb                                                     {
            0: 237,
            1: 194,
            2: 46,
        },
                                                                            }
                                                                            }
