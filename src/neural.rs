use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::{batch_norm, Conv2dConfig};
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};
#[derive(Debug)]
pub struct Resnet {
    conv1: Conv2d,
    batch_norm1: batch_norm::BatchNorm,
    block: ResnetBlock,
    block2: ResnetBlock,
    block3: ResnetBlock,
    policy_conv: Conv2d,
    bias: Tensor,
}

impl Resnet {
    pub fn new(vs: VarBuilder) -> Result<Self> {
        let channel_size = 192;
        let input_channel_size = 104;
        let move_channel_size = 2187;

        let conv1 = candle_nn::conv2d_no_bias(
            input_channel_size,
            channel_size,
            3,
            Conv2dConfig {
                padding: 1,
                stride: 1,
                dilation: 1,
                groups: 1,
            },
            vs.pp("c1"),
        )?;

        let batch_norm1 = batch_norm(channel_size, 1e-5, vs.pp("bn1"))?;

        let block = ResnetBlock::new(vs.clone(), channel_size)?;
        let block2 = ResnetBlock::new(vs.clone(), channel_size)?;
        let block3 = ResnetBlock::new(vs.clone(), channel_size)?;

        let policy_conv = candle_nn::conv2d(
            channel_size,
            move_channel_size,
            1,
            Default::default(),
            vs.pp("c1"),
        )?;

        let bias = Tensor::zeros(move_channel_size, DType::F32, &Device::Cpu)?;

        Ok(Self {
            conv1,
            batch_norm1,
            block,
            block2,
            block3,
            policy_conv,
            bias,
        })
    }

    pub fn forward(&self, xs: &Tensor, train: bool) -> Result<Tensor> {
        let ys = xs
            .apply(&self.conv1)?
            .apply_t(&self.batch_norm1, train)?
            .relu()?;

        let ys = self.block.forward(&ys, train)?;
        let ys = self.block2.forward(&ys, train)?;
        let ys = self.block3.forward(&ys, train)?;

        let ys = self.policy_conv.forward(&ys)?;
        let ys = ys.flatten_all()?;
        let ys = (ys + &self.bias)?;
        return Ok(ys);
    }
}
#[derive(Debug)]
struct ResnetBlock {
    conv1: Conv2d,
    bn1: batch_norm::BatchNorm,
    conv2: Conv2d,
    bn2: batch_norm::BatchNorm,
}

impl ResnetBlock {
    fn new(vs: VarBuilder, channel_size: usize) -> Result<Self> {
        let conv1 = candle_nn::conv2d_no_bias(
            channel_size,
            channel_size,
            3,
            Conv2dConfig {
                padding: 1,
                stride: 1,
                dilation: 1,
                groups: 1,
            },
            vs.pp("c1"),
        )?;
        let bn1 = batch_norm(channel_size, 1e-5, vs.pp("bn1"))?;
        let conv2 = candle_nn::conv2d_no_bias(
            channel_size,
            channel_size,
            3,
            Conv2dConfig {
                padding: 1,
                stride: 1,
                dilation: 1,
                groups: 1,
            },
            vs.pp("c2"),
        )?;
        let bn2 = batch_norm(channel_size, 1e-5, vs.pp("bn1"))?;

        Ok(Self {
            conv1,
            conv2,
            bn1,
            bn2,
        })
    }

    fn forward(&self, xs: &Tensor, train: bool) -> Result<Tensor> {
        let ys = xs
            .apply(&self.conv1)?
            .apply_t(&self.bn1, train)?
            .relu()?
            .apply(&self.conv2)?
            .apply_t(&self.bn2, train)?;
        let ys = (ys + xs)?.relu()?;
        return Ok(ys);
    }
}
