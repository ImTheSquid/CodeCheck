use burn::nn::{
    Dropout, DropoutConfig, LeakyRelu, LeakyReluConfig, Linear, LinearConfig, Relu, Sigmoid,
};

/// Create a sequential neural network, similar to numpy's nn.Sequential.
///
/// To use this macro, separate your modules into three categories:
/// - Unit modules: Modules that don't take any parameters (eg. Relu, Sigmoid)
/// - Modules: Modules that take parameters, but don't have a backend parameter (eg. Dropout, LeakyRelu)
/// - Backend modules: Modules that take a backend parameter (eg. Linear)
///
/// List these classes of modules as comma-separated within classes, then semicolons between, like so:
/// ```
/// gen_sequential! {
///     // No config
///     Relu,
///     Sigmoid;
///     // Has config
///     DropoutConfig => Dropout,
///     LeakyReluConfig => LeakyRelu;
///     // Requires a backend (<B>)
///     LinearConfig => Linear
/// }
/// ```
///
/// To use this macro, use the type `SequentialConfig` and `Sequential<B>` in your code.
macro_rules! gen_sequential {
    ($($unit:tt),*; $($cfg:ty => $module:tt),*; $($bcfg:ty => $bmodule:tt),*) => {
        #[derive(Debug, ::burn::config::Config)]
        pub enum SequentialLayerConfig {
            $($unit,)*
            $($module($cfg),)*
            $($bmodule($bcfg),)*
        }

        #[derive(Debug, ::burn::config::Config)]
        pub struct SequentialConfig {
            pub layers: Vec<SequentialLayerConfig>
        }

        impl SequentialConfig {
            pub fn init<B: ::burn::prelude::Backend>(&self, device: &B::Device) -> Sequential<B> {
                Sequential {
                    layers: self.layers.iter().map(|l| match l {
                        $(SequentialLayerConfig::$unit => SequentialLayer::$unit($unit),)*
                        $(SequentialLayerConfig::$module(c) => SequentialLayer::$module(c.init()),)*
                        $(SequentialLayerConfig::$bmodule(c) => SequentialLayer::$bmodule(c.init(device)),)*
                    }).collect()
                }
            }
        }

        #[derive(Debug, ::burn::module::Module)]
        pub enum SequentialLayer<B: ::burn::prelude::Backend> {
            /// In case the expansion doesn't use any backend-based layers. This should never be used.
            _PhantomData(::core::marker::PhantomData<B>),
            $($unit($unit),)*
            $($module($module),)*
            $($bmodule($bmodule<B>),)*
        }

        #[derive(Debug, ::burn::module::Module)]
        pub struct Sequential<B: ::burn::prelude::Backend> {
            pub layers: Vec<SequentialLayer<B>>
        }

        impl<B: ::burn::prelude::Backend> Sequential<B> {
            pub fn forward<const D: usize>(&self, mut input: ::burn::tensor::Tensor<B, D>) -> ::burn::tensor::Tensor<B, D> {
                for layer in &self.layers {
                    input = match layer {
                        SequentialLayer::_PhantomData(_) => unreachable!("PhantomData should never be instantiated"),
                        $(SequentialLayer::$unit(u) => u.forward(input),)*
                        $(SequentialLayer::$module(m) => m.forward(input),)*
                        $(SequentialLayer::$bmodule(b) => b.forward(input),)*
                    };
                }

                input
            }
        }
    }
}

gen_sequential! {
    Relu,
    Sigmoid;
    DropoutConfig => Dropout,
    LeakyReluConfig => LeakyRelu;
    LinearConfig => Linear
}

// #[derive(Debug, Config)]
// pub enum SequentialLayerConfig {
//     Linear(LinearConfig),
//     Relu,
//     LeakyRelu(LeakyReluConfig),
//     Sigmoid,
//     Dropout(DropoutConfig),
// }

// #[derive(Debug, Config)]
// pub struct SequentialConfig {
//     pub layers: Vec<SequentialLayerConfig>,
// }

// impl SequentialConfig {
//     pub fn init<B: Backend>(&self, device: &B::Device) -> Sequential<B> {
//         Sequential {
//             layers: self
//                 .layers
//                 .iter()
//                 .map(|l| match l {
//                     SequentialLayerConfig::Relu => SequentialLayer::Relu(Relu),
//                     SequentialLayerConfig::Sigmoid => SequentialLayer::Sigmoid(Sigmoid),
//                     SequentialLayerConfig::LeakyRelu(lrc) => SequentialLayer::LeakyRelu(lrc.init()),
//                     SequentialLayerConfig::Linear(l) => SequentialLayer::Linear(l.init(device)),
//                     SequentialLayerConfig::Dropout(d) => SequentialLayer::Dropout(d.init()),
//                 })
//                 .collect(),
//         }
//     }
// }

// #[derive(Module, Debug)]
// pub enum SequentialLayer<B: Backend> {
//     Linear(Linear<B>),
//     Relu(Relu),
//     LeakyRelu(LeakyRelu),
//     Sigmoid(Sigmoid),
//     Dropout(Dropout),
// }

// #[derive(Module, Debug)]
// pub struct Sequential<B: Backend> {
//     layers: Vec<SequentialLayer<B>>,
// }

// impl<B: Backend> Sequential<B> {
//     pub fn forward<const D: usize>(&self, mut input: Tensor<B, D>) -> Tensor<B, D> {
//         for layer in &self.layers {
//             input = match layer {
//                 SequentialLayer::Relu(r) => r.forward(input),
//                 SequentialLayer::Sigmoid(s) => s.forward(input),
//                 SequentialLayer::LeakyRelu(lr) => lr.forward(input),
//                 SequentialLayer::Linear(l) => {
//                     println!(
//                         "LIN WEIGHTS: {} WSUM: {}",
//                         l.weight.val(),
//                         l.weight.val().sum()
//                     );
//                     l.forward(input)
//                 }
//                 SequentialLayer::Dropout(d) => d.forward(input),
//             };
//             println!(
//                 "AFTER {layer:?}, INPUT NOW: {input} SUM {}",
//                 input.clone().sum()
//             );
//         }

//         input
//     }
// }
