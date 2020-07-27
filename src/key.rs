#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// Reserved.
    None,
    /// BACKSPACE key.
    Back,
    /// TAB key.
    Tab,
    /// ENTER key.
    Enter,
    /// CAPS LOCK key.
    CapsLock,
    /// ESC key.
    Escape,
    /// SPACEBAR key.
    Space,
    /// PAGE UP key.
    PageUp,
    /// PAGE DOWN key.
    PageDown,
    /// END key.
    End,
    /// HOME key.
    Home,
    /// LEFT ARROW key.
    Left,
    /// UP ARROW key.
    Up,
    /// RIGHT ARROW key.
    Right,
    /// DOWN ARROW key.
    Down,
    /// SELECT key.
    Select,
    /// PRINT key.
    Print,
    /// EXECUTE key.
    Execute,
    /// PRINT SCREEN key.
    PrintScreen,
    /// INS key.
    Insert,
    /// DEL key.
    Delete,
    /// HELP key.
    Help,
    /// Digit zero key.
    D0,
    /// Digit one key.
    D1,
    /// Digit two key.
    D2,
    /// Digit three key.
    D3,
    /// Digit four key.
    D4,
    /// Digit five key.
    D5,
    /// Digit six key.
    D6,
    /// Digit seven key.
    D7,
    /// Digit eight key.
    D8,
    /// Digit nine key.
    D9,
    /// A key.
    A,
    /// B key.
    B,
    /// C key.
    C,
    /// D key.
    D,
    /// E key.
    E,
    /// F key.
    F,
    /// G key.
    G,
    /// H key.
    H,
    /// I key.
    I,
    /// J key.
    J,
    /// K key.
    K,
    /// L key.
    L,
    /// M key.
    M,
    /// N key.
    N,
    /// O key.
    O,
    /// P key.
    P,
    /// Q key.
    Q,
    /// R key.
    R,
    /// S key.
    S,
    /// T key.
    T,
    /// U key.
    U,
    /// V key.
    V,
    /// W key.
    W,
    /// X key.
    X,
    /// Y key.
    Y,
    /// Z key.
    Z,
    /// Left Windows key.
    LeftWindows,
    /// Right Windows key.
    RightWindows,
    /// Applications key.
    Apps,
    /// Computer Sleep key.
    Sleep,
    /// Numeric keypad 0 key.
    NumPad0,
    /// Numeric keypad 1 key.
    NumPad1,
    /// Numeric keypad 2 key.
    NumPad2,
    /// Numeric keypad 3 key.
    NumPad3,
    /// Numeric keypad 4 key.
    NumPad4,
    /// Numeric keypad 5 key.
    NumPad5,
    /// Numeric keypad 6 key.
    NumPad6,
    /// Numeric keypad 7 key.
    NumPad7,
    /// Numeric keypad 8 key.
    NumPad8,
    /// Numeric keypad 9 key.
    NumPad9,
    /// Multiply key.
    Multiply,
    /// Add key.
    Add,
    /// Separator key.
    Separator,
    /// Subtract key.
    Subtract,
    /// Decimal key.
    Decimal,
    /// Divide key.
    Divide,
    /// F1 key.
    F1,
    /// F2 key.
    F2,
    /// F3 key.
    F3,
    /// F4 key.
    F4,
    /// F5 key.
    F5,
    /// F6 key.
    F6,
    /// F7 key.
    F7,
    /// F8 key.
    F8,
    /// F9 key.
    F9,
    /// F10 key.
    F10,
    /// F11 key.
    F11,
    /// F12 key.
    F12,
    /// F13 key.
    F13,
    /// F14 key.
    F14,
    /// F15 key.
    F15,
    /// F16 key.
    F16,
    /// F17 key.
    F17,
    /// F18 key.
    F18,
    /// F19 key.
    F19,
    /// F20 key.
    F20,
    /// F21 key.
    F21,
    /// F22 key.
    F22,
    /// F23 key.
    F23,
    /// F24 key.
    F24,
    /// NUM LOCK key.
    NumLock,
    /// SCROLL LOCK key.
    Scroll,
    /// Left SHIFT key.
    LeftShift,
    /// Right SHIFT key.
    RightShift,
    /// Left CONTROL key.
    LeftControl,
    /// Right CONTROL key.
    RightControl,
    /// Left ALT key.
    LeftAlt,
    /// Right ALT key.
    RightAlt,
    /// Browser Back key.
    BrowserBack,
    /// Browser Forward key.
    BrowserForward,
    /// Browser Refresh key.
    BrowserRefresh,
    /// Browser Stop key.
    BrowserStop,
    /// Browser Search key.
    BrowserSearch,
    /// Browser Favorites key.
    BrowserFavorites,
    /// Browser Start and Home key.
    BrowserHome,
    /// Volume Mute key.
    VolumeMute,
    /// Volume Down key.
    VolumeDown,
    /// Volume Up key.
    VolumeUp,
    /// Next Track key.
    MediaNextTrack,
    /// Previous Track key.
    MediaPreviousTrack,
    /// Stop Media key.
    MediaStop,
    /// Play/Pause Media key.
    MediaPlayPause,
    /// Start Mail key.
    LaunchMail,
    /// Select Media key.
    SelectMedia,
    /// Start Application 1 key.
    LaunchApplication1,
    /// Start Application 2 key.
    LaunchApplication2,
    /// The OEM Semicolon key on a US standard keyboard.
    OemSemicolon,
    /// For any country/region, the '+' key.
    OemPlus,
    /// For any country/region, the ',' key.
    OemComma,
    /// For any country/region, the '-' key.
    OemMinus,
    /// For any country/region, the '.' key.
    OemPeriod,
    /// The OEM question mark key on a US standard keyboard.
    OemQuestion,
    /// The OEM tilde key on a US standard keyboard.
    OemTilde,
    /// The OEM open bracket key on a US standard keyboard.
    OemOpenBrackets,
    /// The OEM pipe key on a US standard keyboard.
    OemPipe,
    /// The OEM close bracket key on a US standard keyboard.
    OemCloseBrackets,
    /// The OEM singled/double quote key on a US standard keyboard.
    OemQuotes,
    /// Used for miscellaneous characters; it can vary by keyboard.
    Oem8,
    /// The OEM angle bracket or backslash key on the RT 102 key keyboard.
    OemBackslash,
    /// IME PROCESS key.
    ProcessKey,
    /// Attn key.
    Attn,
    /// CrSel key.
    Crsel,
    /// ExSel key.
    Exsel,
    /// Erase EOF key.
    EraseEof,
    /// Play key.
    Play,
    /// Zoom key.
    Zoom,
    /// PA1 key.
    Pa1,
    /// CLEAR key.
    OemClear,
    /// Green ChatPad key.
    ChatPadGreen,
    /// Orange ChatPad key.
    ChatPadOrange,
    /// PAUSE key.
    Pause,
    /// IME Convert key.
    ImeConvert,
    /// IME NoConvert key.
    ImeNoConvert,
    /// Kana key on Japanese keyboards.
    Kana,
    /// Kanji key on Japanese keyboards.
    Kanji,
    /// OEM Auto key.
    OemAuto,
    /// OEM Copy key.
    OemCopy,
    /// OEM Enlarge Window key.
    OemEnlW,
}
