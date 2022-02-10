extern crate bitflags;
use bitflags::bitflags;

// improve this macro so that .mod(bool) can be chained and
// to_key_alt methods can be refactored out
macro_rules! if_escape_code {
    ($x:expr, $y:expr, $z:expr) => {
        if $x.starts_with($y) {
            return Some($z);
        }
    }
}

bitflags! {
    pub struct KeyModifier: u32 {
        const NONE  = 0b00001;
        const CTRL  = 0b00010;
        const ALT   = 0b00100;
        const SHIFT = 0b01000;
    }
}

#[derive(Debug,PartialEq)]
pub enum KeyValue {
    F(u8),
    Char(char),
    Delete,
    Insert,
    Enter,
    Space,
    Esc,
    Tab,
    Up,
    Down,
    Right,
    Left,
    End,
    Home,
    PageUp,
    PageDown,
    Backspace, // that curve tho
}

impl KeyValue {
    pub fn to_key(self) -> Key {
        return Key {
            value: self,
            modifier: KeyModifier::NONE,
        }
    }

    fn to_key_alt(self) -> Key {
        return Key {
            value: self,
            modifier: KeyModifier::ALT,
        }
    }

    fn to_key_ctrl(self) -> Key {
        return Key {
            value: self,
            modifier: KeyModifier::CTRL,
        }
    }

    fn to_key_shift(self) -> Key {
        return Key {
            value: self,
            modifier: KeyModifier::SHIFT,
        }
    }

    fn to_key_hyper(self) -> Key {
        return Key {
            value: self,
            modifier: KeyModifier::ALT | KeyModifier::CTRL,
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Key {
    pub value: KeyValue,
    pub modifier: KeyModifier,
}

impl From<KeyValue> for Key {
    fn from(value: KeyValue) -> Self {
        return Key {
            value,
            modifier: KeyModifier::NONE,
        }
    }
}

impl Key {
    pub const ESC: Key = Key {
       value: KeyValue::Esc,
       modifier: KeyModifier::NONE,
    };

    pub const TAB: Key = Key {
       value: KeyValue::Tab,
       modifier: KeyModifier::NONE,
    };

    todo!("add constants for every special key");

    pub fn from_char(value: char) -> Option<Key> {
        if value.len_utf8() != 1 {
            return None
        }
        return Self::from_byte(value as u8);
    }


    pub fn from_ansi_escape_str(value: &str) -> Option<Key> {

        // PAGE UP
        if_escape_code!(value, "\x1b[5~", KeyValue::PageUp.to_key());

        if_escape_code!(value, "\x1b[5;3~", KeyValue::PageUp.to_key_alt());
        if_escape_code!(value, "\x1b[5;5~", KeyValue::PageUp.to_key_ctrl());
        if_escape_code!(value, "\x1b[5;7~", KeyValue::PageUp.to_key_hyper());

        // PAGE DOWN
        if_escape_code!(value, "\x1b[6~", KeyValue::PageDown.to_key());
        if_escape_code!(value, "\x1b[6;3~", KeyValue::PageDown.to_key_alt());
        if_escape_code!(value, "\x1b[6;5~", KeyValue::PageDown.to_key_ctrl());
        if_escape_code!(value, "\x1b[6;7~", KeyValue::PageDown.to_key_hyper());

        // HOME VT
        if_escape_code!(value, "\x1b[1~", KeyValue::Home.to_key());
        if_escape_code!(value, "\x1b[1;3~", KeyValue::Home.to_key_alt());
        if_escape_code!(value, "\x1b[1;5~", KeyValue::Home.to_key_ctrl());
        if_escape_code!(value, "\x1b[1;7~", KeyValue::Home.to_key_hyper());
        if_escape_code!(value, "\x1b[7~", KeyValue::Home.to_key());
        if_escape_code!(value, "\x1b[7;3~", KeyValue::Home.to_key_alt());
        if_escape_code!(value, "\x1b[7;5~", KeyValue::Home.to_key_ctrl());
        if_escape_code!(value, "\x1b[7;7~", KeyValue::Home.to_key_hyper());

        // HOME xterm
        if_escape_code!(value, "\x1b[H", KeyValue::Home.to_key());
        if_escape_code!(value, "\x1b[1;3H", KeyValue::Home.to_key_alt());
        if_escape_code!(value, "\x1b[1;5H", KeyValue::Home.to_key_ctrl());
        if_escape_code!(value, "\x1b[1;7H", KeyValue::Home.to_key_hyper());

        // END VT
        if_escape_code!(value, "\x1b[4~", KeyValue::End.to_key());
        if_escape_code!(value, "\x1b[4;3~", KeyValue::End.to_key_alt());
        if_escape_code!(value, "\x1b[4;5~", KeyValue::End.to_key_ctrl());
        if_escape_code!(value, "\x1b[4;7~", KeyValue::End.to_key_hyper());
        if_escape_code!(value, "\x1b[8~", KeyValue::End.to_key());
        if_escape_code!(value, "\x1b[8;3~", KeyValue::End.to_key_alt());
        if_escape_code!(value, "\x1b[8;5~", KeyValue::End.to_key_ctrl());
        if_escape_code!(value, "\x1b[8;7~", KeyValue::End.to_key_hyper());

        // END xterm
        if_escape_code!(value, "\x1b[F", KeyValue::End.to_key());
        if_escape_code!(value, "\x1b[1;3F", KeyValue::End.to_key_alt());
        if_escape_code!(value, "\x1b[1;5F", KeyValue::End.to_key_ctrl());
        if_escape_code!(value, "\x1b[1;7F", KeyValue::End.to_key_hyper());

        // UP
        if_escape_code!(value, "\x1b[A", KeyValue::Up.to_key());
        if_escape_code!(value, "\x1b[1;3A", KeyValue::Up.to_key_alt());
        if_escape_code!(value, "\x1b[1;5A", KeyValue::Up.to_key_ctrl());
        if_escape_code!(value, "\x1b[1;7A", KeyValue::Up.to_key_hyper());

        // DOWN
        if_escape_code!(value, "\x1b[B", KeyValue::Down.to_key());
        if_escape_code!(value, "\x1b[1;3B", KeyValue::Down.to_key_alt());
        if_escape_code!(value, "\x1b[1;5B", KeyValue::Down.to_key_ctrl());
        if_escape_code!(value, "\x1b[1;7B", KeyValue::Down.to_key_hyper());

        // RIGHT
        if_escape_code!(value, "\x1b[C", KeyValue::Right.to_key());
        if_escape_code!(value, "\x1b[1;3C", KeyValue::Right.to_key_alt());
        if_escape_code!(value, "\x1b[1;5C", KeyValue::Right.to_key_ctrl());
        if_escape_code!(value, "\x1b[1;7C", KeyValue::Right.to_key_hyper());

        // LEFT
        if_escape_code!(value, "\x1b[D", KeyValue::Left.to_key());
        if_escape_code!(value, "\x1b[1;3D", KeyValue::Left.to_key_alt());
        if_escape_code!(value, "\x1b[1;5D", KeyValue::Left.to_key_ctrl());
        if_escape_code!(value, "\x1b[1;7D", KeyValue::Left.to_key_hyper());

        // INSERT
        if_escape_code!(value, "\x1b[2~", KeyValue::Insert.to_key());
        if_escape_code!(value, "\x1b[2;3~", KeyValue::Insert.to_key_alt());
        if_escape_code!(value, "\x1b[2;5~", KeyValue::Insert.to_key_ctrl());
        if_escape_code!(value, "\x1b[2;7~", KeyValue::Insert.to_key_hyper());

        // DELETE
        if_escape_code!(value, "\x1b[3~", KeyValue::Delete.to_key());
        if_escape_code!(value, "\x1b[3;3~", KeyValue::Delete.to_key_alt());
        if_escape_code!(value, "\x1b[3;5~", KeyValue::Delete.to_key_ctrl());
        if_escape_code!(value, "\x1b[3;7~", KeyValue::Delete.to_key_hyper());

        // F1 vt
        if_escape_code!(value, "\x1b[11~", KeyValue::F(1).to_key());
        if_escape_code!(value, "\x1b[11;3~", KeyValue::F(1).to_key_alt());
        if_escape_code!(value, "\x1b[11;5~", KeyValue::F(1).to_key_ctrl());
        if_escape_code!(value, "\x1b[11;7~", KeyValue::F(1).to_key_hyper());

        // F1 xterm
        if_escape_code!(value, "\x1b[1P", KeyValue::F(1).to_key());

        // F1 pc
        if_escape_code!(value, "\x1bOP", KeyValue::F(1).to_key());
        if_escape_code!(value, "\x1b[1;3P", KeyValue::F(1).to_key_alt());
        if_escape_code!(value, "\x1b[1;5P", KeyValue::F(1).to_key_ctrl());

        // F2 VT
        if_escape_code!(value, "\x1b[12~", KeyValue::F(2).to_key());
        if_escape_code!(value, "\x1b[12;3~", KeyValue::F(2).to_key_alt());
        if_escape_code!(value, "\x1b[12;5~", KeyValue::F(2).to_key_ctrl());

        // F2 xterm
        if_escape_code!(value, "\x1b[1Q", KeyValue::F(2).to_key());

        // F2 pc
        if_escape_code!(value, "\x1bOQ", KeyValue::F(2).to_key());
        if_escape_code!(value, "\x1b[1;3Q", KeyValue::F(2).to_key_alt());
        if_escape_code!(value, "\x1b[1;5Q", KeyValue::F(2).to_key_ctrl());

        // F3 VT
        if_escape_code!(value, "\x1b[13~", KeyValue::F(3).to_key());
        if_escape_code!(value, "\x1b[13;3~", KeyValue::F(3).to_key_alt());
        if_escape_code!(value, "\x1b[13;5~", KeyValue::F(3).to_key_ctrl());

        // F3 xterm
        if_escape_code!(value, "\x1b[1R", KeyValue::F(3).to_key());

        // F3 pc
        if_escape_code!(value, "\x1bOR", KeyValue::F(3).to_key());
        if_escape_code!(value, "\x1b[1;3R", KeyValue::F(3).to_key_alt());
        if_escape_code!(value, "\x1b[1;5R", KeyValue::F(3).to_key_ctrl());

        // F4 VT
        if_escape_code!(value, "\x1b[14~", KeyValue::F(4).to_key());
        if_escape_code!(value, "\x1b[14;3~", KeyValue::F(4).to_key_alt());
        if_escape_code!(value, "\x1b[14;5~", KeyValue::F(4).to_key_ctrl());

        // F4 xterm
        if_escape_code!(value, "\x1b[1S", KeyValue::F(4).to_key());

        // F4 pc
        if_escape_code!(value, "\x1bOS", KeyValue::F(4).to_key());
        if_escape_code!(value, "\x1b[1;3S", KeyValue::F(4).to_key_alt());
        if_escape_code!(value, "\x1b[1;5S", KeyValue::F(4).to_key_ctrl());

        // F5 VT
        if_escape_code!(value, "\x1b[15~", KeyValue::F(5).to_key());
        if_escape_code!(value, "\x1b[15;3~", KeyValue::F(5).to_key_alt());
        if_escape_code!(value, "\x1b[15;5~", KeyValue::F(5).to_key_ctrl());

        // F6 VT
        if_escape_code!(value, "\x1b[17~", KeyValue::F(6).to_key());
        if_escape_code!(value, "\x1b[17;3~", KeyValue::F(6).to_key_alt());
        if_escape_code!(value, "\x1b[17;5~", KeyValue::F(6).to_key_ctrl());

        // F7 VT
        if_escape_code!(value, "\x1b[18~", KeyValue::F(7).to_key());
        if_escape_code!(value, "\x1b[18;3~", KeyValue::F(7).to_key_alt());
        if_escape_code!(value, "\x1b[18;5~", KeyValue::F(7).to_key_ctrl());

        // F8 VT
        if_escape_code!(value, "\x1b[19~", KeyValue::F(8).to_key());
        if_escape_code!(value, "\x1b[19;3~", KeyValue::F(8).to_key_alt());
        if_escape_code!(value, "\x1b[19;5~", KeyValue::F(8).to_key_ctrl());

        // F9 VT
        if_escape_code!(value, "\x1b[20~", KeyValue::F(9).to_key());
        if_escape_code!(value, "\x1b[20;3~", KeyValue::F(9).to_key_alt());
        if_escape_code!(value, "\x1b[20;5~", KeyValue::F(9).to_key_ctrl());

        // F10 VT
        if_escape_code!(value, "\x1b[21~", KeyValue::F(10).to_key());
        if_escape_code!(value, "\x1b[21;3~", KeyValue::F(10).to_key_alt());
        if_escape_code!(value, "\x1b[21;5~", KeyValue::F(10).to_key_ctrl());

        // F11 VT
        if_escape_code!(value, "\x1b[23~", KeyValue::F(11).to_key());
        if_escape_code!(value, "\x1b[23;3~", KeyValue::F(11).to_key_alt());
        if_escape_code!(value, "\x1b[23;5~", KeyValue::F(11).to_key_ctrl());

        // F12 VT
        if_escape_code!(value, "\x1b[24~", KeyValue::F(12).to_key());
        if_escape_code!(value, "\x1b[24;3~", KeyValue::F(12).to_key_alt());
        if_escape_code!(value, "\x1b[24;5~", KeyValue::F(12).to_key_ctrl());

        // SHIFT TAB
        if_escape_code!(value, "\x1b[Z", KeyValue::Tab.to_key_shift());

        return None;
    }

    pub fn from_byte(value: u8) -> Option<Key> {
        if Self::is_byte_print_char(value) {
            return Self::parse_print_char_key(value);
        }

        if Self::is_byte_control_char(value) {
            return Self::parse_control_char_key(value);
        }

        return None
    }

    pub fn shift(mut self , is_on: bool) -> Key {
        if is_on {
            if self.modifier == KeyModifier::NONE {
                self.modifier = KeyModifier::SHIFT;
                return self;
            }

            self.modifier = self.modifier | KeyModifier::SHIFT;
            return self
        } else {
            self.modifier = self.modifier - KeyModifier::SHIFT;
            return self
        }
    }

    pub fn ctrl(mut self , is_on: bool) -> Key {
        if is_on {
            if self.modifier == KeyModifier::NONE {
                self.modifier = KeyModifier::CTRL;
                return self;
            }

            self.modifier = self.modifier | KeyModifier::CTRL;
            return self
        } else {
            self.modifier = self.modifier - KeyModifier::CTRL;
            return self
        }
    }

    pub fn alt(mut self , is_on: bool) -> Key {
        if is_on {
            if self.modifier == KeyModifier::NONE {
                self.modifier = KeyModifier::ALT;
                return self;
            }

            self.modifier = self.modifier | KeyModifier::ALT;
            return self
        } else {
            self.modifier = self.modifier - KeyModifier::ALT;
            return self
        }
    }

    /// parse a single printable charicter into a Key
    /// if the charicter is not printable the result will be None
    fn parse_print_char_key(byte: u8) -> Option<Key> {
        if !Self::is_byte_print_char(byte) {
            return None
        }

        if byte == 32 {
            return Some(Key {
                value: KeyValue::Space,
                modifier: KeyModifier::NONE,
            });
        }

        let is_uppercase = (byte as char).is_uppercase();

        let value = if is_uppercase {
            (byte + 32) as char
        } else {
            byte as char
        };

        return Some(Key {
            value: KeyValue::Char(value),
            modifier: if is_uppercase { KeyModifier::SHIFT } else { KeyModifier::NONE },
        })
    }

    fn parse_control_char_key(byte: u8) -> Option<Key> {
        if !Self::is_byte_control_char(byte) {
            return None
        }

        // TODO clean this up with match statement and struct constants i.e. ESC
        if byte == 0 {
            return Some(Key {
                value: KeyValue::Space,
                modifier: KeyModifier::CTRL,
            });
        }

        if byte == 27 {
            return Some(Key {
                value: KeyValue::Esc,
                modifier: KeyModifier::NONE,
            });
        }

        if byte == 127 {
            return Some(Key {
                value: KeyValue::Backspace,
                modifier: KeyModifier::NONE,
            });
        }

        let value = (byte + 96) as char;

        if value == 'h' {
            return Some(Key {
                value: KeyValue::Backspace,
                modifier: KeyModifier::CTRL,
            });
        }

        if value == 'm' {
            return Some(Key {
                value: KeyValue::Enter,
                modifier: KeyModifier::NONE,
            });
        }

        if value == 'i' {
            return Some(Key {
                value: KeyValue::Tab,
                modifier: KeyModifier::NONE,
            });
        }

        return Some(Key {
            value: KeyValue::Char(value),
            modifier: KeyModifier::CTRL,
        });
    }

    fn is_byte_control_char(byte: u8) -> bool {
        if  byte <= 31 || byte == 127 {
            return true;
        }
        return false;
    }

    fn is_byte_print_char(byte: u8) -> bool {
            if byte >= 32 && byte <= 126 {
                return true;
            }
            return false;
    }
}
