/* automatically generated by rust-bindgen */

pub const FASTTEXT_TRUE: ::std::os::raw::c_uint = 1;
pub const CFASTTEXT_FALSE: ::std::os::raw::c_uint = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_args_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_prediction_t {
    pub prob: f32,
    pub label: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_predictions_t {
    pub predictions: *mut fasttext_prediction_t,
    pub length: usize,
}

extern "C" {
    pub fn cft_args_new() -> *mut fasttext_args_t;
    pub fn cft_args_parse(
        handle: *mut fasttext_args_t,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_args_free(handle: *mut fasttext_args_t);
    pub fn cft_args_get_input(handle: *mut fasttext_args_t) -> *const ::std::os::raw::c_char;
    pub fn cft_args_set_input(handle: *mut fasttext_args_t, input: *const ::std::os::raw::c_char);
    pub fn cft_args_get_output(handle: *mut fasttext_args_t) -> *const ::std::os::raw::c_char;
    pub fn cft_args_set_output(handle: *mut fasttext_args_t, output: *const ::std::os::raw::c_char);
    pub fn cft_fasttext_new() -> *mut fasttext_t;
    pub fn cft_fasttext_free(handle: *mut fasttext_t);
    pub fn cft_fasttext_load_model(
        handle: *mut fasttext_t,
        filename: *const ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_save_model(handle: *mut fasttext_t);
    pub fn cft_fasttext_save_output(handle: *mut fasttext_t);
    pub fn cft_fasttext_save_vectors(handle: *mut fasttext_t);
    pub fn cft_fasttext_get_dimension(handle: *mut fasttext_t) -> ::std::os::raw::c_int;
    pub fn cft_fasttext_get_word_id(
        handle: *mut fasttext_t,
        word: *const ::std::os::raw::c_char,
    ) -> i32;
    pub fn cft_fasttext_get_subword_id(
        handle: *mut fasttext_t,
        word: *const ::std::os::raw::c_char,
    ) -> i32;
    pub fn cft_fasttext_is_quant(handle: *mut fasttext_t) -> bool;
    pub fn cft_fasttext_analogies(handle: *mut fasttext_t, k: i32);
    pub fn cft_fasttext_train_thread(handle: *mut fasttext_t, n: i32);
    pub fn cft_fasttext_load_vectors(
        handle: *mut fasttext_t,
        filename: *const ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_train(handle: *mut fasttext_t, args: *mut fasttext_args_t);
    pub fn cft_fasttext_predict(
        handle: *mut fasttext_t,
        text: *const ::std::os::raw::c_char,
        k: i32,
        threshold: f32,
    ) -> *mut fasttext_predictions_t;
    pub fn cft_fasttext_predictions_free(predictions: *mut fasttext_predictions_t);
    pub fn cft_fasttext_quantize(handle: *mut fasttext_t, args: *mut fasttext_args_t);
    pub fn cft_fasttext_get_word_vector(handle: *mut fasttext_t, word: *const ::std::os::raw::c_char, buf: *mut ::std::os::raw::c_float);
}
