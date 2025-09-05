#[macro_export]
macro_rules! keyboard_inputs {
    ($selfe:expr, $data:expr, $event:expr, $( $char:expr, $func:ident ),* ) => {
        if let Event::Key(key) = $event
            && key.is_press()
        {
            $(
                if key.code == KeyCode::Char($char) {
                    return $func($selfe, $data);
                }
            )*
        }
    };
}
