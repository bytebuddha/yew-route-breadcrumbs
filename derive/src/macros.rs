macro_rules! error {
    ($syntax:ident, $message:expr) => {
        $syntax.span().unwrap().error($message).emit()
    }
}
