impl From<ropey::Error> for YggdrasilError {
    fn from(e: ropey::Error) -> Self {
        Self::language_error(e.to_string())
    }
}
