//! Implements [`SessionParams`], which configures a [`crate::LlamaSession`]

use std::ptr::null_mut;

use llama_cpp_sys::{
    ggml_type, llama_context_default_params, llama_context_params, llama_pooling_type,
    llama_pooling_type_LLAMA_POOLING_TYPE_CLS, llama_pooling_type_LLAMA_POOLING_TYPE_MEAN,
    llama_pooling_type_LLAMA_POOLING_TYPE_NONE, llama_pooling_type_LLAMA_POOLING_TYPE_UNSPECIFIED,
};

/// whether to pool (sum) embedding results by sequence id (ignored if no pooling layer)
#[derive(Clone, Copy, Debug)]
pub enum PoolingType {
    /// Unspecified.
    Unspecified,
    /// Don't pool.
    None,
    /// TODO lookup what this does
    Mean,
    /// TODO lookup what this does
    Cls,
}

impl From<PoolingType> for llama_pooling_type {
    fn from(value: PoolingType) -> Self {
        match value {
            PoolingType::Unspecified => llama_pooling_type_LLAMA_POOLING_TYPE_UNSPECIFIED,
            PoolingType::None => llama_pooling_type_LLAMA_POOLING_TYPE_NONE,
            PoolingType::Mean => llama_pooling_type_LLAMA_POOLING_TYPE_MEAN,
            PoolingType::Cls => llama_pooling_type_LLAMA_POOLING_TYPE_CLS,
        }
    }
}

impl From<llama_pooling_type> for PoolingType {
    fn from(value: llama_pooling_type) -> Self {
        #![allow(non_upper_case_globals)]
        match value {
            llama_pooling_type_LLAMA_POOLING_TYPE_UNSPECIFIED => PoolingType::Unspecified,
            llama_pooling_type_LLAMA_POOLING_TYPE_NONE => PoolingType::None,
            llama_pooling_type_LLAMA_POOLING_TYPE_MEAN => PoolingType::Mean,
            llama_pooling_type_LLAMA_POOLING_TYPE_CLS => PoolingType::Cls,
            _ => unimplemented!(),
        }
    }
}

/// Session-specific parameters.
#[derive(Clone)]
pub struct SessionParams {
    /// RNG seed, [`u32::MAX`] for random (default)
    pub seed: u32,

    /// text context, 0 = from model
    pub n_ctx: u32,

    /// prompt processing maximum batch size
    pub n_batch: u32,

    /// number of threads to use for generation
    pub n_threads: u32,

    /// number of threads to use for batch processing
    pub n_threads_batch: u32,

    /// RoPE scaling type, from [`llama_rope_scaling_type`]
    pub rope_scaling_type: i32,

    /// ref: https://github.com/ggerganov/llama.cpp/pull/2054

    /// RoPE base frequency, 0 = from model
    pub rope_freq_base: f32,

    /// RoPE frequency scaling factor, 0 = from model
    pub rope_freq_scale: f32,

    /// YaRN extrapolation mix factor, negative = from model
    pub yarn_ext_factor: f32,

    /// YaRN magnitude scaling factor
    pub yarn_attn_factor: f32,

    /// YaRN low correction dim
    pub yarn_beta_fast: f32,

    /// YaRN high correction dim
    pub yarn_beta_slow: f32,

    /// YaRN original context size
    pub yarn_orig_ctx: u32,

    /// data type for K cache
    pub type_k: u32,

    /// data type for V cache
    pub type_v: u32,

    /// embedding mode only
    pub embedding: bool,

    /// whether to offload the KQV ops (including the KV cache) to GPU
    pub offload_kqv: bool,

    /// whether to pool (sum) embedding results by sequence id (ignored if no pooling layer)
    pub pooling: PoolingType,

    /// defragment the KV cache if holes/size > thold, < 0 disabled (default)
    pub defrag_threshold: f32,
}

impl Default for SessionParams {
    fn default() -> Self {
        let c_defaults = unsafe {
            // SAFETY: Stack constructor, always safe.
            llama_context_default_params()
        };

        let threads = num_cpus::get_physical() as u32 - 1;

        Self {
            seed: c_defaults.seed,
            n_ctx: c_defaults.n_ctx,
            n_batch: c_defaults.n_batch,
            n_threads: threads,
            n_threads_batch: threads,
            rope_scaling_type: c_defaults.rope_scaling_type,
            rope_freq_base: c_defaults.rope_freq_base,
            rope_freq_scale: c_defaults.rope_freq_scale,
            yarn_ext_factor: c_defaults.yarn_ext_factor,
            yarn_attn_factor: c_defaults.yarn_attn_factor,
            yarn_beta_fast: c_defaults.yarn_beta_fast,
            yarn_beta_slow: c_defaults.yarn_beta_slow,
            yarn_orig_ctx: c_defaults.yarn_orig_ctx,
            type_k: c_defaults.type_k as u32,
            type_v: c_defaults.type_v as u32,
            embedding: c_defaults.embedding,
            offload_kqv: c_defaults.offload_kqv,
            pooling: c_defaults.pooling_type.into(),
            defrag_threshold: c_defaults.defrag_thold,
        }
    }
}

impl From<SessionParams> for llama_context_params {
    fn from(value: SessionParams) -> Self {
        Self {
            seed: value.seed,
            n_ctx: value.n_ctx,
            n_batch: value.n_batch,
            n_threads: value.n_threads,
            n_threads_batch: value.n_threads_batch,
            rope_scaling_type: value.rope_scaling_type,
            rope_freq_base: value.rope_freq_base,
            rope_freq_scale: value.rope_freq_scale,
            yarn_ext_factor: value.yarn_ext_factor,
            yarn_attn_factor: value.yarn_attn_factor,
            yarn_beta_fast: value.yarn_beta_fast,
            yarn_beta_slow: value.yarn_beta_slow,
            yarn_orig_ctx: value.yarn_orig_ctx,
            defrag_thold: value.defrag_threshold,
            cb_eval: None,
            cb_eval_user_data: null_mut(),
            type_k: value.type_k as ggml_type,
            type_v: value.type_v as ggml_type,
            logits_all: false, // Deprecated
            embedding: value.embedding,
            offload_kqv: value.offload_kqv,
            pooling_type: value.pooling.into(),
            abort_callback: None,
            abort_callback_data: null_mut(),
        }
    }
}
