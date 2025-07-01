

//use super::*;

	#[derive(Copy, Clone)]
	pub struct RgbColor
	{
		pub red : i32,
		pub green : i32,
		pub blue : i32,
	}
	
	impl RgbColor 
	{
		pub fn new(_red : i32, _green : i32, _blue : i32) -> RgbColor
		{
			RgbColor { red: _red, green : _green, blue : _blue }
		}
		pub fn default() -> RgbColor { RgbColor { red: 0, green : 0, blue : 0 } }
		pub fn fromi32(color : (i32,i32,i32)) -> RgbColor { RgbColor { red: color.0, green : color.1, blue : color.2 }}
		pub fn soft_gray(&self) -> i32 { (self.red + self.green + self.blue) / 3  }
	}
	#[derive(Copy, Clone)]
	pub struct RgbColorA
	{
		pub red : i32,
		pub green : i32,
		pub blue : i32,
		pub opacity : i32,
	}

	impl RgbColorA
	{
		pub fn new(_red : i32, _green : i32, _blue : i32, _opacity : i32) -> RgbColorA
		{
			RgbColorA { red: _red, green : _green, blue : _blue, opacity : _opacity }
		}
		pub fn from_rgb(color : RgbColor,_opacity : i32) -> RgbColorA
		{
			RgbColorA { red: color.red, green : color.green, blue : color.blue, opacity : _opacity }
		}
	}

// We want to keep color names casual, so we allow mixed-case here, e.g. sage_color::Yellow vs. sage_color::YELLOW
	//
	#[allow(non_upper_case_globals)]
	pub mod sage_color
	{
		pub use super::*; 
		
		//use crate::*;

		pub static DefaultBgColor                  : RgbColor = RgbColor{red: 20  , green: 40  , blue: 121 };
		pub static DefaultFgColor                  : RgbColor = RgbColor{red: 255 , green: 255 , blue: 255 };
		pub static SliderTextColor                 : RgbColor = RgbColor{red: 128 , green: 128 , blue: 128 };
		pub static Green                           : RgbColor = RgbColor{red: 0   , green: 255 , blue: 0   };
		pub static DarkGreen                       : RgbColor = RgbColor{red: 0   , green: 128 , blue: 0   };
		pub static LightGreen                      : RgbColor = RgbColor{red: 128 , green: 255 , blue: 128 };
		pub static Blue                            : RgbColor = RgbColor{red: 0   , green: 0   , blue: 255 };
		pub static Blue32                          : RgbColor = RgbColor{red: 0   , green: 0   , blue: 32  };
		pub static Blue48                          : RgbColor = RgbColor{red: 0   , green: 0   , blue: 48  };
		pub static Blue64                          : RgbColor = RgbColor{red: 0   , green: 0   , blue: 64  };
		pub static DarkBlue                        : RgbColor = RgbColor{red: 0   , green: 0   , blue: 92  };
		pub static LightBlue                       : RgbColor = RgbColor{red: 40  , green: 100 , blue: 255 };
		pub static SkyBlue                         : RgbColor = RgbColor{red: 40  , green: 145 , blue: 255 };
		pub static SkyBlueDark                     : RgbColor = RgbColor{red: 0   , green: 30  , blue: 128 };
		pub static SkyBlueLight                    : RgbColor = RgbColor{red: 75  , green: 165 , blue: 255 };
		pub static PaleBlueDark                    : RgbColor = RgbColor{red: 40  , green: 100 , blue: 140 };
		pub static PaleBlue                        : RgbColor = RgbColor{red: 103 , green: 179 , blue: 217 };
		pub static PaleBlueLight                   : RgbColor = RgbColor{red: 145 , green: 190 , blue: 215 };
		pub static LightCyan                       : RgbColor = RgbColor{red: 128 , green: 255 , blue: 255 }; 
		pub static Cyan                            : RgbColor = RgbColor{red: 0   , green: 255 , blue: 255 }; 
		pub static MidCyan                         : RgbColor = RgbColor{red: 15  , green: 200 , blue: 200 }; 
		pub static DarkCyan                        : RgbColor = RgbColor{red: 30  , green: 130 , blue: 130 }; 
		pub static Red                             : RgbColor = RgbColor{red: 255 , green: 0   , blue: 0   };
		pub static LightRed                        : RgbColor = RgbColor{red: 255 , green: 128 , blue: 128 };
		pub static LightYellow                     : RgbColor = RgbColor{red: 255 , green: 255 , blue: 128 };
		pub static Yellow                          : RgbColor = RgbColor{red: 255 , green: 255 , blue: 0   };
		pub static Magenta                         : RgbColor = RgbColor{red: 255 , green: 0   , blue: 255 };
		pub static MediumMagenta                   : RgbColor = RgbColor{red: 255 , green: 92  , blue: 255 };
		pub static LightMagenta                    : RgbColor = RgbColor{red: 255 , green: 128 , blue: 255 };
		pub static Purple                          : RgbColor = RgbColor{red: 255 , green: 0   , blue: 255 };
		pub static LightPurple                     : RgbColor = RgbColor{red: 255 , green: 128 , blue: 255 };
		pub static MediumPurple                    : RgbColor = RgbColor{red: 255 , green: 92  , blue: 255 };
		pub static White                           : RgbColor = RgbColor{red: 255 , green: 255 , blue: 255 }; 
		pub static Gray172                         : RgbColor = RgbColor{red: 172 , green: 172 , blue: 172 }; 
		pub static Gray192                         : RgbColor = RgbColor{red: 192 , green: 192 , blue: 192 }; 
		pub static Gray220                         : RgbColor = RgbColor{red: 220 , green: 220 , blue: 220 }; 
		pub static Gray128                         : RgbColor = RgbColor{red: 128 , green: 128 , blue: 128 }; 
		pub static Gray32                          : RgbColor = RgbColor{red: 32  , green: 32  , blue: 32  }; 
		pub static Gray42                          : RgbColor = RgbColor{red: 42  , green: 42  , blue: 42  }; 
		pub static Gray64                          : RgbColor = RgbColor{red: 64  , green: 64  , blue: 64  }; 
		pub static Gray72                          : RgbColor = RgbColor{red: 72  , green: 72  , blue: 72  }; 
		pub static Gray92                          : RgbColor = RgbColor{red: 92  , green: 92  , blue: 92  }; 
		pub static Black                           : RgbColor = RgbColor{red: 0   , green: 0   , blue: 0   }; 
		pub static LightGray                       : RgbColor = RgbColor{red: 200 , green: 200 , blue: 200 };
		pub static LightGrey                       : RgbColor = RgbColor{red: 200 , green: 200 , blue: 200 };
		pub static MidGray                         : RgbColor = RgbColor{red: 64  , green: 64  , blue: 64  };
		pub static MidGrey                         : RgbColor = RgbColor{red: 64  , green: 64  , blue: 64  };
		pub static DarkGray                        : RgbColor = RgbColor{red: 32  , green: 32  , blue: 32  };
		pub static DarkGrey                        : RgbColor = RgbColor{red: 32  , green: 32  , blue: 32  };
		pub static Gray                            : RgbColor = RgbColor{red: 128 , green: 128 , blue: 128 };
		pub static Grey                            : RgbColor = RgbColor{red: 128 , green: 128 , blue: 128 };
		pub static NearWhite                       : RgbColor = RgbColor{red: 220 , green: 220 , blue: 220 };
		pub static ButtonTextColorNormal           : RgbColor = RgbColor{red: 220 , green: 220 , blue: 220 };
		pub static ButtonTextColorHighlighted      : RgbColor = RgbColor{red: 255 , green: 255 , blue: 255 };
		pub static ButtonTextColorPressed          : RgbColor = RgbColor{red: 255 , green: 255 , blue: 255 };
		pub static ButtonTextColorDisabled         : RgbColor = RgbColor{red: 170 , green: 170 , blue: 170 };
		pub static CheckboxTextColorNormal         : RgbColor = RgbColor{red: 220 , green: 220 , blue: 220 };
		pub static CheckboxTextColorHighlighted    : RgbColor = RgbColor{red: 255 , green: 255 , blue: 255 };
		pub static CheckboxTextColorChecked        : RgbColor = RgbColor{red: 220 , green: 220 , blue: 220 };
		pub static CheckboxTextColorCheckedHigh    : RgbColor = RgbColor{red: 220 , green: 220 , blue: 220 };
		pub static CheckboxTextColorDisabled       : RgbColor = RgbColor{red: 170 , green: 170 , blue: 170 };
		pub static Orange                          : RgbColor = RgbColor{red: 255 , green: 115 , blue: 0   };
		pub static LightOrange                     : RgbColor = RgbColor{red: 255 , green: 130 , blue: 0   };
		pub static DarkOrange                      : RgbColor = RgbColor{red: 255 , green: 85  , blue: 0   };

	}

	#[allow(non_upper_case_globals,non_snake_case)]
	pub mod sage_color_a
	{
				pub use super::*; 
		
		//use crate::*; 
		pub fn DefaultBgColor                  (_opacity : i32) -> RgbColorA { RgbColorA {red: 20  , green: 40  , blue: 121, opacity : _opacity } }
		pub fn DefaultFgColor                  (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 255 , blue: 255, opacity : _opacity } }
		pub fn SliderTextColor                 (_opacity : i32) -> RgbColorA { RgbColorA {red: 128 , green: 128 , blue: 128, opacity : _opacity } }
		pub fn Green                           (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 255 , blue: 0  , opacity : _opacity } }
		pub fn DarkGreen                       (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 128 , blue: 0  , opacity : _opacity } }
		pub fn LightGreen                      (_opacity : i32) -> RgbColorA { RgbColorA {red: 128 , green: 255 , blue: 128, opacity : _opacity } }
		pub fn Blue                            (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 0   , blue: 255, opacity : _opacity } }
		pub fn Blue32                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 0   , blue: 32 , opacity : _opacity } }
		pub fn Blue48                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 0   , blue: 48 , opacity : _opacity } }
		pub fn Blue64                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 0   , blue: 64 , opacity : _opacity } }
		pub fn DarkBlue                        (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 0   , blue: 92 , opacity : _opacity } }
		pub fn LightBlue                       (_opacity : i32) -> RgbColorA { RgbColorA {red: 40  , green: 100 , blue: 255, opacity : _opacity } }
		pub fn SkyBlue                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 40  , green: 145 , blue: 255, opacity : _opacity } }
		pub fn SkyBlueDark                     (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 30  , blue: 128, opacity : _opacity } }
		pub fn SkyBlueLight                    (_opacity : i32) -> RgbColorA { RgbColorA {red: 75  , green: 165 , blue: 255, opacity : _opacity } }
		pub fn PaleBlueDark                    (_opacity : i32) -> RgbColorA { RgbColorA {red: 40  , green: 100 , blue: 140, opacity : _opacity } }
		pub fn PaleBlue                        (_opacity : i32) -> RgbColorA { RgbColorA {red: 103 , green: 179 , blue: 217, opacity : _opacity } }
		pub fn PaleBlueLight                   (_opacity : i32) -> RgbColorA { RgbColorA {red: 145 , green: 190 , blue: 215, opacity : _opacity } }
		pub fn LightCyan                       (_opacity : i32) -> RgbColorA { RgbColorA {red: 128 , green: 255 , blue: 255, opacity : _opacity } } 
		pub fn Cyan                            (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 255 , blue: 255, opacity : _opacity } } 
		pub fn MidCyan                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 15  , green: 200 , blue: 200, opacity : _opacity } } 
		pub fn DarkCyan                        (_opacity : i32) -> RgbColorA { RgbColorA {red: 30  , green: 130 , blue: 130, opacity : _opacity } } 
		pub fn Red                             (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 0   , blue: 0  , opacity : _opacity } }
		pub fn LightRed                        (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 128 , blue: 128, opacity : _opacity } }
		pub fn LightYellow                     (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 255 , blue: 128, opacity : _opacity } }
		pub fn Yellow                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 255 , blue: 0  , opacity : _opacity } }
		pub fn Magenta                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 0   , blue: 255, opacity : _opacity } }
		pub fn MediumMagenta                   (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 92  , blue: 255, opacity : _opacity } }
		pub fn LightMagenta                    (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 128 , blue: 255, opacity : _opacity } }
		pub fn Purple                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 0   , blue: 255, opacity : _opacity } }
		pub fn LightPurple                     (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 128 , blue: 255, opacity : _opacity } }
		pub fn MediumPurple                    (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 92  , blue: 255, opacity : _opacity } }
		pub fn White                           (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 255 , blue: 255, opacity : _opacity } } 
		pub fn Gray172                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 172 , green: 172 , blue: 172, opacity : _opacity } } 
		pub fn Gray192                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 192 , green: 192 , blue: 192, opacity : _opacity } } 
		pub fn Gray220                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 220 , green: 220 , blue: 220, opacity : _opacity } } 
		pub fn Gray128                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 128 , green: 128 , blue: 128, opacity : _opacity } } 
		pub fn Gray32                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 32  , green: 32  , blue: 32 , opacity : _opacity } } 
		pub fn Gray42                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 42  , green: 42  , blue: 42 , opacity : _opacity } } 
		pub fn Gray64                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 64  , green: 64  , blue: 64 , opacity : _opacity } } 
		pub fn Gray72                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 72  , green: 72  , blue: 72 , opacity : _opacity } } 
		pub fn Gray92                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 92  , green: 92  , blue: 92 , opacity : _opacity } } 
		pub fn Black                           (_opacity : i32) -> RgbColorA { RgbColorA {red: 0   , green: 0   , blue: 0  , opacity : _opacity } } 
		pub fn LightGray                       (_opacity : i32) -> RgbColorA { RgbColorA {red: 200 , green: 200 , blue: 200, opacity : _opacity } }
		pub fn LightGrey                       (_opacity : i32) -> RgbColorA { RgbColorA {red: 200 , green: 200 , blue: 200, opacity : _opacity } }
		pub fn MidGray                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 64  , green: 64  , blue: 64 , opacity : _opacity } }
		pub fn MidGrey                         (_opacity : i32) -> RgbColorA { RgbColorA {red: 64  , green: 64  , blue: 64 , opacity : _opacity } }
		pub fn DarkGray                        (_opacity : i32) -> RgbColorA { RgbColorA {red: 32  , green: 32  , blue: 32 , opacity : _opacity } }
		pub fn DarkGrey                        (_opacity : i32) -> RgbColorA { RgbColorA {red: 32  , green: 32  , blue: 32 , opacity : _opacity } }
		pub fn Gray                            (_opacity : i32) -> RgbColorA { RgbColorA {red: 128 , green: 128 , blue: 128, opacity : _opacity } }
		pub fn Grey                            (_opacity : i32) -> RgbColorA { RgbColorA {red: 128 , green: 128 , blue: 128, opacity : _opacity } }
		pub fn NearWhite                       (_opacity : i32) -> RgbColorA { RgbColorA {red: 220 , green: 220 , blue: 220, opacity : _opacity } }
		pub fn ButtonTextColorNormal           (_opacity : i32) -> RgbColorA { RgbColorA {red: 220 , green: 220 , blue: 220, opacity : _opacity } }
		pub fn ButtonTextColorHighlighted      (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 255 , blue: 255, opacity : _opacity } }
		pub fn ButtonTextColorPressed          (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 255 , blue: 255, opacity : _opacity } }
		pub fn ButtonTextColorDisabled         (_opacity : i32) -> RgbColorA { RgbColorA {red: 170 , green: 170 , blue: 170, opacity : _opacity } }
		pub fn CheckboxTextColorNormal         (_opacity : i32) -> RgbColorA { RgbColorA {red: 220 , green: 220 , blue: 220, opacity : _opacity } }
		pub fn CheckboxTextColorHighlighted    (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 255 , blue: 255, opacity : _opacity } }
		pub fn CheckboxTextColorChecked        (_opacity : i32) -> RgbColorA { RgbColorA {red: 220 , green: 220 , blue: 220, opacity : _opacity } }
		pub fn CheckboxTextColorCheckedHigh    (_opacity : i32) -> RgbColorA { RgbColorA {red: 220 , green: 220 , blue: 220, opacity : _opacity } }
		pub fn CheckboxTextColorDisabled       (_opacity : i32) -> RgbColorA { RgbColorA {red: 170 , green: 170 , blue: 170, opacity : _opacity } }
		pub fn Orange                          (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 115 , blue: 0  , opacity : _opacity } }
		pub fn LightOrange                     (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 130 , blue: 0  , opacity : _opacity } }
		pub fn DarkOrange                      (_opacity : i32) -> RgbColorA { RgbColorA {red: 255 , green: 85  , blue: 0  , opacity : _opacity } }	}


	#[allow(non_upper_case_globals)]
	pub mod pan_color
	{
				pub use super::*; 
		
		//use crate::*; 
		pub static AliceBlue            : RgbColor = RgbColor { red: 0xF0, green: 0xF8, blue: 0xFF };
		pub static AntiqueWhite         : RgbColor = RgbColor { red: 0xFA, green: 0xEB, blue: 0xD7 };
		pub static Aqua                 : RgbColor = RgbColor { red: 0x00, green: 0xFF, blue: 0xFF };
		pub static Aquamarine           : RgbColor = RgbColor { red: 0x7F, green: 0xFF, blue: 0xD4 };
		pub static Azure                : RgbColor = RgbColor { red: 0xF0, green: 0xFF, blue: 0xFF };
		pub static Beige                : RgbColor = RgbColor { red: 0xF5, green: 0xF5, blue: 0xDC };
		pub static Bisque               : RgbColor = RgbColor { red: 0xFF, green: 0xE4, blue: 0xC4 };
		pub static Black                : RgbColor = RgbColor { red: 0x00, green: 0x00, blue: 0x00 };
		pub static BlanchedAlmond       : RgbColor = RgbColor { red: 0xFF, green: 0xEB, blue: 0xCD };
		pub static Blue                 : RgbColor = RgbColor { red: 0x00, green: 0x00, blue: 0xFF };
		pub static BlueViolet           : RgbColor = RgbColor { red: 0x8A, green: 0x2B, blue: 0xE2 };
		pub static Brown                : RgbColor = RgbColor { red: 0xA5, green: 0x2A, blue: 0x2A };
		pub static BurlyWood            : RgbColor = RgbColor { red: 0xDE, green: 0xB8, blue: 0x87 };
		pub static CadetBlue            : RgbColor = RgbColor { red: 0x5F, green: 0x9E, blue: 0xA0 };
		pub static Chartreuse           : RgbColor = RgbColor { red: 0x7F, green: 0xFF, blue: 0x00 };
		pub static Chocolate            : RgbColor = RgbColor { red: 0xD2, green: 0x69, blue: 0x1E };
		pub static Coral                : RgbColor = RgbColor { red: 0xFF, green: 0x7F, blue: 0x50 };
		pub static CornflowerBlue       : RgbColor = RgbColor { red: 0x64, green: 0x95, blue: 0xED };
		pub static Cornsilk             : RgbColor = RgbColor { red: 0xFF, green: 0xF8, blue: 0xDC };
		pub static Crimson              : RgbColor = RgbColor { red: 0xDC, green: 0x14, blue: 0x3C };
		pub static Cyan                 : RgbColor = RgbColor { red: 0x00, green: 0xFF, blue: 0xFF };
		pub static DarkBlue             : RgbColor = RgbColor { red: 0x00, green: 0x00, blue: 0x8B };
		pub static DarkCyan             : RgbColor = RgbColor { red: 0x00, green: 0x8B, blue: 0x8B };
		pub static DarkGoldenrod        : RgbColor = RgbColor { red: 0xB8, green: 0x86, blue: 0x0B };
		pub static DarkGray             : RgbColor = RgbColor { red: 0xA9, green: 0xA9, blue: 0xA9 };
		pub static DarkGreen            : RgbColor = RgbColor { red: 0x00, green: 0x64, blue: 0x00 };
		pub static DarkKhaki            : RgbColor = RgbColor { red: 0xBD, green: 0xB7, blue: 0x6B };
		pub static DarkMagenta          : RgbColor = RgbColor { red: 0x8B, green: 0x00, blue: 0x8B };
		pub static DarkOliveGreen       : RgbColor = RgbColor { red: 0x55, green: 0x6B, blue: 0x2F };
		pub static DarkOrange           : RgbColor = RgbColor { red: 0xFF, green: 0x8C, blue: 0x00 };
		pub static DarkOrchid           : RgbColor = RgbColor { red: 0x99, green: 0x32, blue: 0xCC };
		pub static DarkRed              : RgbColor = RgbColor { red: 0x8B, green: 0x00, blue: 0x00 };
		pub static DarkSalmon           : RgbColor = RgbColor { red: 0xE9, green: 0x96, blue: 0x7A };
		pub static DarkSeaGreen         : RgbColor = RgbColor { red: 0x8F, green: 0xBC, blue: 0x8B };
		pub static DarkSlateBlue        : RgbColor = RgbColor { red: 0x48, green: 0x3D, blue: 0x8B };
		pub static DarkSlateGray        : RgbColor = RgbColor { red: 0x2F, green: 0x4F, blue: 0x4F };
		pub static DarkTurquoise        : RgbColor = RgbColor { red: 0x00, green: 0xCE, blue: 0xD1 };
		pub static DarkViolet           : RgbColor = RgbColor { red: 0x94, green: 0x00, blue: 0xD3 };
		pub static DeepPink             : RgbColor = RgbColor { red: 0xFF, green: 0x14, blue: 0x93 };
		pub static DeepSkyBlue          : RgbColor = RgbColor { red: 0x00, green: 0xBF, blue: 0xFF };
		pub static DimGray              : RgbColor = RgbColor { red: 0x69, green: 0x69, blue: 0x69 };
		pub static DodgerBlue           : RgbColor = RgbColor { red: 0x1E, green: 0x90, blue: 0xFF };
		pub static Firebrick            : RgbColor = RgbColor { red: 0xB2, green: 0x22, blue: 0x22 };
		pub static FloralWhite          : RgbColor = RgbColor { red: 0xFF, green: 0xFA, blue: 0xF0 };
		pub static ForestGreen          : RgbColor = RgbColor { red: 0x22, green: 0x8B, blue: 0x22 };
		pub static Fuchsia              : RgbColor = RgbColor { red: 0xFF, green: 0x00, blue: 0xFF };
		pub static Gainsboro            : RgbColor = RgbColor { red: 0xDC, green: 0xDC, blue: 0xDC };
		pub static GhostWhite           : RgbColor = RgbColor { red: 0xF8, green: 0xF8, blue: 0xFF };
		pub static Gold                 : RgbColor = RgbColor { red: 0xFF, green: 0xD7, blue: 0x00 };
		pub static Goldenrod            : RgbColor = RgbColor { red: 0xDA, green: 0xA5, blue: 0x20 };
		pub static Gray                 : RgbColor = RgbColor { red: 0x80, green: 0x80, blue: 0x80 };
		pub static Green                : RgbColor = RgbColor { red: 0x00, green: 0x80, blue: 0x00 };
		pub static GreenYellow          : RgbColor = RgbColor { red: 0xAD, green: 0xFF, blue: 0x2F };
		pub static Honeydew             : RgbColor = RgbColor { red: 0xF0, green: 0xFF, blue: 0xF0 };
		pub static HotPink              : RgbColor = RgbColor { red: 0xFF, green: 0x69, blue: 0xB4 };
		pub static IndianRed            : RgbColor = RgbColor { red: 0xCD, green: 0x5C, blue: 0x5C };
		pub static Indigo               : RgbColor = RgbColor { red: 0x4B, green: 0x00, blue: 0x82 };
		pub static Ivory                : RgbColor = RgbColor { red: 0xFF, green: 0xFF, blue: 0xF0 };
		pub static Khaki                : RgbColor = RgbColor { red: 0xF0, green: 0xE6, blue: 0x8C };
		pub static Lavender             : RgbColor = RgbColor { red: 0xE6, green: 0xE6, blue: 0xFA };
		pub static LavenderBlush        : RgbColor = RgbColor { red: 0xFF, green: 0xF0, blue: 0xF5 };
		pub static LawnGreen            : RgbColor = RgbColor { red: 0x7C, green: 0xFC, blue: 0x00 };
		pub static LemonChiffon         : RgbColor = RgbColor { red: 0xFF, green: 0xFA, blue: 0xCD };
		pub static LightBlue            : RgbColor = RgbColor { red: 0xAD, green: 0xD8, blue: 0xE6 };
		pub static LightCoral           : RgbColor = RgbColor { red: 0xF0, green: 0x80, blue: 0x80 };
		pub static LightCyan            : RgbColor = RgbColor { red: 0xE0, green: 0xFF, blue: 0xFF };
		pub static LightGoldenrodYellow : RgbColor = RgbColor { red: 0xFA, green: 0xFA, blue: 0xD2 };
		pub static LightGray            : RgbColor = RgbColor { red: 0xD3, green: 0xD3, blue: 0xD3 };
		pub static LightGreen           : RgbColor = RgbColor { red: 0x90, green: 0xEE, blue: 0x90 };
		pub static LightPink            : RgbColor = RgbColor { red: 0xFF, green: 0xB6, blue: 0xC1 };
		pub static LightSalmon          : RgbColor = RgbColor { red: 0xFF, green: 0xA0, blue: 0x7A };
		pub static LightSeaGreen        : RgbColor = RgbColor { red: 0x20, green: 0xB2, blue: 0xAA };
		pub static LightSkyBlue         : RgbColor = RgbColor { red: 0x87, green: 0xCE, blue: 0xFA };
		pub static LightSlateGray       : RgbColor = RgbColor { red: 0x77, green: 0x88, blue: 0x99 };
		pub static LightSteelBlue       : RgbColor = RgbColor { red: 0xB0, green: 0xC4, blue: 0xDE };
		pub static LightYellow          : RgbColor = RgbColor { red: 0xFF, green: 0xFF, blue: 0xE0 };
		pub static Lime                 : RgbColor = RgbColor { red: 0x00, green: 0xFF, blue: 0x00 };
		pub static LimeGreen            : RgbColor = RgbColor { red: 0x32, green: 0xCD, blue: 0x32 };
		pub static Linen                : RgbColor = RgbColor { red: 0xFA, green: 0xF0, blue: 0xE6 };
		pub static Magenta              : RgbColor = RgbColor { red: 0xFF, green: 0x00, blue: 0xFF };
		pub static Maroon               : RgbColor = RgbColor { red: 0x80, green: 0x00, blue: 0x00 };
		pub static MediumAquamarine     : RgbColor = RgbColor { red: 0x66, green: 0xCD, blue: 0xAA };
		pub static MediumBlue           : RgbColor = RgbColor { red: 0x00, green: 0x00, blue: 0xCD };
		pub static MediumOrchid         : RgbColor = RgbColor { red: 0xBA, green: 0x55, blue: 0xD3 };
		pub static MediumPurple         : RgbColor = RgbColor { red: 0x93, green: 0x70, blue: 0xDB };
		pub static MediumSeaGreen       : RgbColor = RgbColor { red: 0x3C, green: 0xB3, blue: 0x71 };
		pub static MediumSlateBlue      : RgbColor = RgbColor { red: 0x7B, green: 0x68, blue: 0xEE };
		pub static MediumSpringGreen    : RgbColor = RgbColor { red: 0x00, green: 0xFA, blue: 0x9A };
		pub static MediumTurquoise      : RgbColor = RgbColor { red: 0x48, green: 0xD1, blue: 0xCC };
		pub static MediumVioletRed      : RgbColor = RgbColor { red: 0xC7, green: 0x15, blue: 0x85 };
		pub static MidnightBlue         : RgbColor = RgbColor { red: 0x19, green: 0x19, blue: 0x70 };
		pub static MintCream            : RgbColor = RgbColor { red: 0xF5, green: 0xFF, blue: 0xFA };
		pub static MistyRose            : RgbColor = RgbColor { red: 0xFF, green: 0xE4, blue: 0xE1 };
		pub static Moccasin             : RgbColor = RgbColor { red: 0xFF, green: 0xE4, blue: 0xB5 };
		pub static NavajoWhite          : RgbColor = RgbColor { red: 0xFF, green: 0xDE, blue: 0xAD };
		pub static Navy                 : RgbColor = RgbColor { red: 0x00, green: 0x00, blue: 0x80 };
		pub static OldLace              : RgbColor = RgbColor { red: 0xFD, green: 0xF5, blue: 0xE6 };
		pub static Olive                : RgbColor = RgbColor { red: 0x80, green: 0x80, blue: 0x00 };
		pub static OliveDrab            : RgbColor = RgbColor { red: 0x6B, green: 0x8E, blue: 0x23 };
		pub static Orange               : RgbColor = RgbColor { red: 0xFF, green: 0xA5, blue: 0x00 };
		pub static OrangeRed            : RgbColor = RgbColor { red: 0xFF, green: 0x45, blue: 0x00 };
		pub static Orchid               : RgbColor = RgbColor { red: 0xDA, green: 0x70, blue: 0xD6 };
		pub static PaleGoldenrod        : RgbColor = RgbColor { red: 0xEE, green: 0xE8, blue: 0xAA };
		pub static PaleGreen            : RgbColor = RgbColor { red: 0x98, green: 0xFB, blue: 0x98 };
		pub static PaleTurquoise        : RgbColor = RgbColor { red: 0xAF, green: 0xEE, blue: 0xEE };
		pub static PaleVioletRed        : RgbColor = RgbColor { red: 0xDB, green: 0x70, blue: 0x93 };
		pub static PapayaWhip           : RgbColor = RgbColor { red: 0xFF, green: 0xEF, blue: 0xD5 };
		pub static PeachPuff            : RgbColor = RgbColor { red: 0xFF, green: 0xDA, blue: 0xB9 };
		pub static Peru                 : RgbColor = RgbColor { red: 0xCD, green: 0x85, blue: 0x3F };
		pub static Pink                 : RgbColor = RgbColor { red: 0xFF, green: 0xC0, blue: 0xCB };
		pub static Plum                 : RgbColor = RgbColor { red: 0xDD, green: 0xA0, blue: 0xDD };
		pub static PowderBlue           : RgbColor = RgbColor { red: 0xB0, green: 0xE0, blue: 0xE6 };
		pub static Purple               : RgbColor = RgbColor { red: 0x80, green: 0x00, blue: 0x80 };
		pub static Red                  : RgbColor = RgbColor { red: 0xFF, green: 0x00, blue: 0x00 };
		pub static RosyBrown            : RgbColor = RgbColor { red: 0xBC, green: 0x8F, blue: 0x8F };
		pub static RoyalBlue            : RgbColor = RgbColor { red: 0x41, green: 0x69, blue: 0xE1 };
		pub static SaddleBrown          : RgbColor = RgbColor { red: 0x8B, green: 0x45, blue: 0x13 };
		pub static Salmon               : RgbColor = RgbColor { red: 0xFA, green: 0x80, blue: 0x72 };
		pub static SandyBrown           : RgbColor = RgbColor { red: 0xF4, green: 0xA4, blue: 0x60 };
		pub static SeaGreen             : RgbColor = RgbColor { red: 0x2E, green: 0x8B, blue: 0x57 };
		pub static SeaShell             : RgbColor = RgbColor { red: 0xFF, green: 0xF5, blue: 0xEE };
		pub static Sienna               : RgbColor = RgbColor { red: 0xA0, green: 0x52, blue: 0x2D };
		pub static Silver               : RgbColor = RgbColor { red: 0xC0, green: 0xC0, blue: 0xC0 };
		pub static SkyBlue              : RgbColor = RgbColor { red: 0x87, green: 0xCE, blue: 0xEB };
		pub static SlateBlue            : RgbColor = RgbColor { red: 0x6A, green: 0x5A, blue: 0xCD };
		pub static SlateGray            : RgbColor = RgbColor { red: 0x70, green: 0x80, blue: 0x90 };
		pub static Snow                 : RgbColor = RgbColor { red: 0xFF, green: 0xFA, blue: 0xFA };
		pub static SpringGreen          : RgbColor = RgbColor { red: 0x00, green: 0xFF, blue: 0x7F };
		pub static SteelBlue            : RgbColor = RgbColor { red: 0x46, green: 0x82, blue: 0xB4 };
		pub static Tan                  : RgbColor = RgbColor { red: 0xD2, green: 0xB4, blue: 0x8C };
		pub static Teal                 : RgbColor = RgbColor { red: 0x00, green: 0x80, blue: 0x80 };
		pub static Thistle              : RgbColor = RgbColor { red: 0xD8, green: 0xBF, blue: 0xD8 };
		pub static Tomato               : RgbColor = RgbColor { red: 0xFF, green: 0x63, blue: 0x47 };
		pub static Transparent          : RgbColor = RgbColor { red: 0xFF, green: 0xFF, blue: 0xFF };
		pub static Turquoise            : RgbColor = RgbColor { red: 0x40, green: 0xE0, blue: 0xD0 };
		pub static Violet               : RgbColor = RgbColor { red: 0xEE, green: 0x82, blue: 0xEE };
		pub static Wheat                : RgbColor = RgbColor { red: 0xF5, green: 0xDE, blue: 0xB3 };
		pub static White                : RgbColor = RgbColor { red: 0xFF, green: 0xFF, blue: 0xFF };
		pub static WhiteSmoke           : RgbColor = RgbColor { red: 0xF5, green: 0xF5, blue: 0xF5 };
		pub static Yellow               : RgbColor = RgbColor { red: 0xFF, green: 0xFF, blue: 0x00 };
		pub static YellowGreen          : RgbColor = RgbColor { red: 0x9A, green: 0xCD, blue: 0x32 };
	}

	#[allow(non_upper_case_globals,non_snake_case)]
	pub mod pan_color_a
	{
		pub use super::*; 
		
		//use crate::*; 
		pub fn AliceBlue            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF0, green: 0xF8, blue: 0xFF, opacity : _opacity } }
		pub fn AntiqueWhit          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFA, green: 0xEB, blue: 0xD7, opacity : _opacity } }
		pub fn Aqua                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0xFF, blue: 0xFF, opacity : _opacity } }
		pub fn Aquamarine           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x7F, green: 0xFF, blue: 0xD4, opacity : _opacity } }
		pub fn Azure                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF0, green: 0xFF, blue: 0xFF, opacity : _opacity } }
		pub fn Beige                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF5, green: 0xF5, blue: 0xDC, opacity : _opacity } }
		pub fn Bisque               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xE4, blue: 0xC4, opacity : _opacity } }
		pub fn Black                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x00, blue: 0x00, opacity : _opacity } }
		pub fn BlanchedAlmnd        (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xEB, blue: 0xCD, opacity : _opacity } }
		pub fn Blue                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x00, blue: 0xFF, opacity : _opacity } }
		pub fn BlueViolet           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x8A, green: 0x2B, blue: 0xE2, opacity : _opacity } }
		pub fn Brown                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xA5, green: 0x2A, blue: 0x2A, opacity : _opacity } }
		pub fn BurlyWood            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xDE, green: 0xB8, blue: 0x87, opacity : _opacity } }
		pub fn CadetBlue            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x5F, green: 0x9E, blue: 0xA0, opacity : _opacity } }
		pub fn Chartreuse           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x7F, green: 0xFF, blue: 0x00, opacity : _opacity } }
		pub fn Chocolate            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xD2, green: 0x69, blue: 0x1E, opacity : _opacity } }
		pub fn Coral                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x7F, blue: 0x50, opacity : _opacity } }
		pub fn CornflowerBue        (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x64, green: 0x95, blue: 0xED, opacity : _opacity } }
		pub fn Cornsilk             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xF8, blue: 0xDC, opacity : _opacity } }
		pub fn Crimson              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xDC, green: 0x14, blue: 0x3C, opacity : _opacity } }
		pub fn Cyan                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0xFF, blue: 0xFF, opacity : _opacity } }
		pub fn DarkBlue             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x00, blue: 0x8B, opacity : _opacity } }
		pub fn DarkCyan             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x8B, blue: 0x8B, opacity : _opacity } }
		pub fn DarkGoldenrd         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xB8, green: 0x86, blue: 0x0B, opacity : _opacity } }
		pub fn DarkGray             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xA9, green: 0xA9, blue: 0xA9, opacity : _opacity } }
		pub fn DarkGreen            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x64, blue: 0x00, opacity : _opacity } }
		pub fn DarkKhaki            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xBD, green: 0xB7, blue: 0x6B, opacity : _opacity } }
		pub fn DarkMagenta          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x8B, green: 0x00, blue: 0x8B, opacity : _opacity } }
		pub fn DarkOliveGren        (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x55, green: 0x6B, blue: 0x2F, opacity : _opacity } }
		pub fn DarkOrange           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x8C, blue: 0x00, opacity : _opacity } }
		pub fn DarkOrchid           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x99, green: 0x32, blue: 0xCC, opacity : _opacity } }
		pub fn DarkRed              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x8B, green: 0x00, blue: 0x00, opacity : _opacity } }
		pub fn DarkSalmon           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xE9, green: 0x96, blue: 0x7A, opacity : _opacity } }
		pub fn DarkSeaGree          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x8F, green: 0xBC, blue: 0x8B, opacity : _opacity } }
		pub fn DarkSlateBle         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x48, green: 0x3D, blue: 0x8B, opacity : _opacity } }
		pub fn DarkSlateGry         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x2F, green: 0x4F, blue: 0x4F, opacity : _opacity } }
		pub fn DarkTurquoie         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0xCE, blue: 0xD1, opacity : _opacity } }
		pub fn DarkViolet           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x94, green: 0x00, blue: 0xD3, opacity : _opacity } }
		pub fn DeepPink             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x14, blue: 0x93, opacity : _opacity } }
		pub fn DeepSkyBlue          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0xBF, blue: 0xFF, opacity : _opacity } }
		pub fn DimGray              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x69, green: 0x69, blue: 0x69, opacity : _opacity } }
		pub fn DodgerBlue           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x1E, green: 0x90, blue: 0xFF, opacity : _opacity } }
		pub fn Firebrick            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xB2, green: 0x22, blue: 0x22, opacity : _opacity } }
		pub fn FloralWhite          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFA, blue: 0xF0, opacity : _opacity } }
		pub fn ForestGreen          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x22, green: 0x8B, blue: 0x22, opacity : _opacity } }
		pub fn Fuchsia              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x00, blue: 0xFF, opacity : _opacity } }
		pub fn Gainsboro            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xDC, green: 0xDC, blue: 0xDC, opacity : _opacity } }
		pub fn GhostWhite           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF8, green: 0xF8, blue: 0xFF, opacity : _opacity } }
		pub fn Gold                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xD7, blue: 0x00, opacity : _opacity } }
		pub fn Goldenrod            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xDA, green: 0xA5, blue: 0x20, opacity : _opacity } }
		pub fn Gray                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x80, green: 0x80, blue: 0x80, opacity : _opacity } }
		pub fn Green                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x80, blue: 0x00, opacity : _opacity } }
		pub fn GreenYellow          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xAD, green: 0xFF, blue: 0x2F, opacity : _opacity } }
		pub fn Honeydew             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF0, green: 0xFF, blue: 0xF0, opacity : _opacity } }
		pub fn HotPink              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x69, blue: 0xB4, opacity : _opacity } }
		pub fn IndianRed            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xCD, green: 0x5C, blue: 0x5C, opacity : _opacity } }
		pub fn Indigo               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x4B, green: 0x00, blue: 0x82, opacity : _opacity } }
		pub fn Ivory                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFF, blue: 0xF0, opacity : _opacity } }
		pub fn Khaki                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF0, green: 0xE6, blue: 0x8C, opacity : _opacity } }
		pub fn Lavender             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xE6, green: 0xE6, blue: 0xFA, opacity : _opacity } }
		pub fn LavenderBluh         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xF0, blue: 0xF5, opacity : _opacity } }
		pub fn LawnGreen            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x7C, green: 0xFC, blue: 0x00, opacity : _opacity } }
		pub fn LemonChiffo          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFA, blue: 0xCD, opacity : _opacity } }
		pub fn LightBlue            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xAD, green: 0xD8, blue: 0xE6, opacity : _opacity } }
		pub fn LightCoral           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF0, green: 0x80, blue: 0x80, opacity : _opacity } }
		pub fn LightCyan            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xE0, green: 0xFF, blue: 0xFF, opacity : _opacity } }
		pub fn LightGoldenrodYellow (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFA, green: 0xFA, blue: 0xD2, opacity : _opacity } }
		pub fn LightGray            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xD3, green: 0xD3, blue: 0xD3, opacity : _opacity } }
		pub fn LightGreen           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x90, green: 0xEE, blue: 0x90, opacity : _opacity } }
		pub fn LightPink            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xB6, blue: 0xC1, opacity : _opacity } }
		pub fn LightSalmon          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xA0, blue: 0x7A, opacity : _opacity } }
		pub fn LightSeaGren         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x20, green: 0xB2, blue: 0xAA, opacity : _opacity } }
		pub fn LightSkyBlu          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x87, green: 0xCE, blue: 0xFA, opacity : _opacity } }
		pub fn LightSlateGay        (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x77, green: 0x88, blue: 0x99, opacity : _opacity } }
		pub fn LightSteelBue        (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xB0, green: 0xC4, blue: 0xDE, opacity : _opacity } }
		pub fn LightYellow          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFF, blue: 0xE0, opacity : _opacity } }
		pub fn Lime                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0xFF, blue: 0x00, opacity : _opacity } }
		pub fn LimeGreen            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x32, green: 0xCD, blue: 0x32, opacity : _opacity } }
		pub fn Linen                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFA, green: 0xF0, blue: 0xE6, opacity : _opacity } }
		pub fn Magenta              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x00, blue: 0xFF, opacity : _opacity } }
		pub fn Maroon               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x80, green: 0x00, blue: 0x00, opacity : _opacity } }
		pub fn MediumAquamrine      (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x66, green: 0xCD, blue: 0xAA, opacity : _opacity } }
		pub fn MediumBlue           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x00, blue: 0xCD, opacity : _opacity } }
		pub fn MediumOrchi          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xBA, green: 0x55, blue: 0xD3, opacity : _opacity } }
		pub fn MediumPurpl          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x93, green: 0x70, blue: 0xDB, opacity : _opacity } }
		pub fn MediumSeaGren        (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x3C, green: 0xB3, blue: 0x71, opacity : _opacity } }
		pub fn MediumSlatelue       (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x7B, green: 0x68, blue: 0xEE, opacity : _opacity } }
		pub fn MediumSprinGreen     (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0xFA, blue: 0x9A, opacity : _opacity } }
		pub fn MediumTurquise       (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x48, green: 0xD1, blue: 0xCC, opacity : _opacity } }
		pub fn MediumVioleRed       (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xC7, green: 0x15, blue: 0x85, opacity : _opacity } }
		pub fn MidnightBlu          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x19, green: 0x19, blue: 0x70, opacity : _opacity } }
		pub fn MintCream            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF5, green: 0xFF, blue: 0xFA, opacity : _opacity } }
		pub fn MistyRose            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xE4, blue: 0xE1, opacity : _opacity } }
		pub fn Moccasin             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xE4, blue: 0xB5, opacity : _opacity } }
		pub fn NavajoWhite          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xDE, blue: 0xAD, opacity : _opacity } }
		pub fn Navy                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x00, blue: 0x80, opacity : _opacity } }
		pub fn OldLace              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFD, green: 0xF5, blue: 0xE6, opacity : _opacity } }
		pub fn Olive                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x80, green: 0x80, blue: 0x00, opacity : _opacity } }
		pub fn OliveDrab            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x6B, green: 0x8E, blue: 0x23, opacity : _opacity } }
		pub fn Orange               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xA5, blue: 0x00, opacity : _opacity } }
		pub fn OrangeRed            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x45, blue: 0x00, opacity : _opacity } }
		pub fn Orchid               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xDA, green: 0x70, blue: 0xD6, opacity : _opacity } }
		pub fn PaleGoldenrd         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xEE, green: 0xE8, blue: 0xAA, opacity : _opacity } }
		pub fn PaleGreen            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x98, green: 0xFB, blue: 0x98, opacity : _opacity } }
		pub fn PaleTurquoie         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xAF, green: 0xEE, blue: 0xEE, opacity : _opacity } }
		pub fn PaleVioletRd         (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xDB, green: 0x70, blue: 0x93, opacity : _opacity } }
		pub fn PapayaWhip           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xEF, blue: 0xD5, opacity : _opacity } }
		pub fn PeachPuff            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xDA, blue: 0xB9, opacity : _opacity } }
		pub fn Peru                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xCD, green: 0x85, blue: 0x3F, opacity : _opacity } }
		pub fn Pink                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xC0, blue: 0xCB, opacity : _opacity } }
		pub fn Plum                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xDD, green: 0xA0, blue: 0xDD, opacity : _opacity } }
		pub fn PowderBlue           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xB0, green: 0xE0, blue: 0xE6, opacity : _opacity } }
		pub fn Purple               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x80, green: 0x00, blue: 0x80, opacity : _opacity } }
		pub fn Red                  (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x00, blue: 0x00, opacity : _opacity } }
		pub fn RosyBrown            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xBC, green: 0x8F, blue: 0x8F, opacity : _opacity } }
		pub fn RoyalBlue            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x41, green: 0x69, blue: 0xE1, opacity : _opacity } }
		pub fn SaddleBrown          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x8B, green: 0x45, blue: 0x13, opacity : _opacity } }
		pub fn Salmon               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFA, green: 0x80, blue: 0x72, opacity : _opacity } }
		pub fn SandyBrown           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF4, green: 0xA4, blue: 0x60, opacity : _opacity } }
		pub fn SeaGreen             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x2E, green: 0x8B, blue: 0x57, opacity : _opacity } }
		pub fn SeaShell             (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xF5, blue: 0xEE, opacity : _opacity } }
		pub fn Sienna               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xA0, green: 0x52, blue: 0x2D, opacity : _opacity } }
		pub fn Silver               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xC0, green: 0xC0, blue: 0xC0, opacity : _opacity } }
		pub fn SkyBlue              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x87, green: 0xCE, blue: 0xEB, opacity : _opacity } }
		pub fn SlateBlue            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x6A, green: 0x5A, blue: 0xCD, opacity : _opacity } }
		pub fn SlateGray            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x70, green: 0x80, blue: 0x90, opacity : _opacity } }
		pub fn Snow                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFA, blue: 0xFA, opacity : _opacity } }
		pub fn SpringGreen          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0xFF, blue: 0x7F, opacity : _opacity } }
		pub fn SteelBlue            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x46, green: 0x82, blue: 0xB4, opacity : _opacity } }
		pub fn Tan                  (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xD2, green: 0xB4, blue: 0x8C, opacity : _opacity } }
		pub fn Teal                 (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x00, green: 0x80, blue: 0x80, opacity : _opacity } }
		pub fn Thistle              (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xD8, green: 0xBF, blue: 0xD8, opacity : _opacity } }
		pub fn Tomato               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0x63, blue: 0x47, opacity : _opacity } }
		pub fn Transparent          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFF, blue: 0xFF, opacity : _opacity } }
		pub fn Turquoise            (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x40, green: 0xE0, blue: 0xD0, opacity : _opacity } }
		pub fn Violet               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xEE, green: 0x82, blue: 0xEE, opacity : _opacity } }
		pub fn Wheat                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF5, green: 0xDE, blue: 0xB3, opacity : _opacity } }
		pub fn White                (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFF, blue: 0xFF, opacity : _opacity } }
		pub fn WhiteSmoke           (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xF5, green: 0xF5, blue: 0xF5, opacity : _opacity } }
		pub fn Yellow               (_opacity : i32) -> RgbColorA { RgbColorA { red: 0xFF, green: 0xFF, blue: 0x00, opacity : _opacity } }
		pub fn YellowGreen          (_opacity : i32) -> RgbColorA { RgbColorA { red: 0x9A, green: 0xCD, blue: 0x32, opacity : _opacity } }
	}
