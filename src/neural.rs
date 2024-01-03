use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};

// class ResNetBlock(nn.Module):
//     def __init__(self, channels: int):
//         super(ResNetBlock, self).__init__()
//         self.conv1 = nn.Conv2d(channels, channels, kernel_size=3, padding=1, bias=False)
//         self.bn1 = nn.BatchNorm2d(channels)
//         self.conv2 = nn.Conv2d(channels, channels, kernel_size=3, padding=1, bias=False)
//         self.bn2 = nn.BatchNorm2d(channels)

//     def forward(self, x: jaxtyping.Float[torch.Tensor, "1"]) -> torch.Tensor:
//         out = self.conv1(x)
//         out = self.bn1(out)
//         out = F.relu(out)

//         out = self.conv2(out)
//         out = self.bn2(out)

//         return F.relu(out + x)

struct ResnetBlock {
    conv1: Conv2d,
    bn1: batch_norm::BatchNorm,
    conv2: Conv2d,
    bn2: batch_norm::BatchNorm,
}

impl ResnetBlock {
    fn new(vs: VarBuilder) -> Result<Self> {
        let channel_size = 10;

        let conv1 = candle_nn::conv2d(
            channel_size,
            channel_size,
            3,
            Default::default(),
            vs.pp("c1"),
        )?;
        let bn1 = batch_norm(channel_size, 1e-5, vs.pp("bn1"))?;
        let conv2 = candle_nn::conv2d(
            channel_size,
            channel_size,
            3,
            Default::default(),
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
        let xs = xs.apply(&self.conv1)?.apply(&self.conv2)?.relu()?;
        return Ok(xs);
    }
}

#[derive(Debug)]
struct ConvNet {
    conv1: Conv2d,
    conv2: Conv2d,
    fc1: Linear,
    fc2: Linear,
    dropout: candle_nn::Dropout,
}

impl ConvNet {
    fn new(vs: VarBuilder) -> Result<Self> {
        let conv1 = candle_nn::conv2d(1, 32, 5, Default::default(), vs.pp("c1"))?;
        let conv2 = candle_nn::conv2d(32, 64, 5, Default::default(), vs.pp("c2"))?;
        let fc1 = candle_nn::linear(1024, 1024, vs.pp("fc1"))?;
        let fc2 = candle_nn::linear(1024, 10, vs.pp("fc2"))?;
        let dropout = candle_nn::Dropout::new(0.5);
        Ok(Self {
            conv1,
            conv2,
            fc1,
            fc2,
            dropout,
        })
    }

    fn forward(&self, xs: &Tensor, train: bool) -> Result<Tensor> {
        let xs = xs
            .apply(&self.conv1)?
            .max_pool2d(2)?
            .apply(&self.conv2)?
            .max_pool2d(2)?
            .flatten_from(1)?
            .apply(&self.fc1)?
            .relu()?;
        self.dropout.forward_t(&xs, train)?.apply(&self.fc2)
    }
}
