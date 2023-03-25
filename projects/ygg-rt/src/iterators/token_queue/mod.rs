// This structure serves to improve performance over Token objects in two ways:
//
//   * it is smaller than a Token, leading to both less memory use when stored in the queue but also
//     increased speed when pushing to the queue
//   * it finds its pair in O(1) time instead of O(N), since pair positions are known at parse time
//     and can easily be stored instead of recomputed
#[derive(Debug)]
pub enum TokenQueue<R> {
    Start {
        end_token_index: usize,
        input_offset: usize,
    },
    End {
        start_token_index: usize,
        input_offset: usize,
        rule: R,
        #[cfg(feature = "dynamic")]
        tag: Option<String>,
        #[cfg(not(feature = "dynamic"))]
        tag: Option<&'static str>,
    },
}
