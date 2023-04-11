use crate::consts::max_item_idx;

pub struct Layer {
    pub name: String,
    pub inputs: usize,
    pub outputs: usize,
    pub weights: Vec<Vec<f32>>,
    pub biases: Vec<f32>,
}

impl Layer {
    pub fn new(inputs: usize, outputs: usize, index: usize, last: bool) -> Self {
        let name = format!(
            "{} - {}",
            index.to_string(),
            String::from(if last { "Output Layer" } else { "Hidden Layer" })
        );

        Layer {
            name,
            inputs,
            outputs,
            weights: vec![vec![0f32; inputs]; outputs],
            biases: vec![0f32; outputs],
        }
    }

    pub fn output(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut weighted_output = vec![0f32; self.outputs];

        // todo: improve this
        for node_idx in 0..self.outputs {
            weighted_output[node_idx] += (0..self.inputs)
                .fold(self.biases[node_idx], |acc, input_idx| {
                    inputs[input_idx] * self.weights[node_idx][input_idx] + acc
                })
        }

        weighted_output
    }
}

pub struct NeuralNetwork {
    pub inputs: usize,
    pub layers: Vec<Layer>,
}

impl NeuralNetwork {
    pub fn new_network(layer_sizes: &[usize]) -> Self {
        let mut layers = vec![];

        for idx in 1..layer_sizes.len() {
            layers.push(Layer::new(
                layer_sizes[idx - 1],
                layer_sizes[idx],
                idx,
                idx == layer_sizes.len() - 1,
            ))
        }

        NeuralNetwork {
            inputs: layer_sizes[0],
            layers,
        }
    }

    // fixme
    pub fn output(&self, inputs: Vec<f32>) -> Vec<f32> {
        assert_eq!(inputs.len(), self.inputs);
        let mut input: Vec<f32> = inputs;

        for layer in &self.layers {
            input = layer.output(input)
        }

        let output = input;
        output
    }

    pub fn predict(&self, inputs: Vec<f32>) -> usize {
        assert_eq!(inputs.len(), self.inputs);
        let output = self.output(inputs);
        return max_item_idx(output);
    }
}

pub mod benchmark {
    use super::NeuralNetwork;
    use crate::consts::*;

    pub fn old_nn(x: f32, y: f32, weights: [[f32; 2]; 2], biases: [f32; 2]) -> u8 {
        let is_okay = x * weights[0][0] + y * weights[1][0] + biases[0];
        let not_okay = x * weights[0][1] + y * weights[1][1] + biases[1];
        if is_okay > not_okay {
            0
        } else {
            1
        }
    }

    pub fn test_old(weights: [[f32; 2]; 2], biases: [f32; 2]) {
        for x in -(OFFSET as i16)..(WIDTH - OFFSET) as i16 {
            for y in (0i16..HEIGHT as i16).rev() {
                old_nn(
                    x as f32 / WIDTH as f32,
                    (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                    weights,
                    biases,
                );
            }
        }
    }

    pub fn emtpy_new_nn() {
        let network = NeuralNetwork::new_network(&[2, 2]);

        for x in -(OFFSET as i16)..(WIDTH - OFFSET) as i16 {
            for y in (0i16..HEIGHT as i16).rev() {
                network.predict(vec![
                    x as f32 / WIDTH as f32,
                    (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                ]) as u8;
            }
        }
    }

    pub fn train_new_nn(weights: [[f32; 2]; 2], biases: [f32; 2]) {
        let mut network = NeuralNetwork::new_network(&[2, 2]);

        network.layers[0].weights = weights.map(|v| v.to_vec()).to_vec();
        network.layers[0].biases = biases.to_vec();

        for x in -(OFFSET as i16)..(WIDTH - OFFSET) as i16 {
            for y in (0i16..HEIGHT as i16).rev() {
                network.predict(vec![
                    x as f32 / WIDTH as f32,
                    (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                ]) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::OFFSET;

    use super::benchmark::{old_nn, test_old};
    use super::Layer;
    use super::NeuralNetwork;
    use crate::consts::*;

    #[test]
    fn test_layer_basic() {
        let mut layer = Layer::new(2, 3, 1, true);

        // setting the biases
        layer.biases[0] = 0.9;
        layer.biases[1] = 0.2;
        layer.biases[2] = 0.5;

        // setting the weights
        layer.weights[0][0] = 1.;
        layer.weights[0][1] = 1.5;

        layer.weights[1][0] = 3.;
        layer.weights[1][1] = 0.2;

        layer.weights[2][0] = 2.1;
        layer.weights[2][1] = 3.;

        let output = layer.output(vec![2., 1.]);

        assert_eq!(output.len(), 3);
        assert!(output[0] == 4.4);
        assert!(output[1] == 2f32 * 3. + 1f32 * 0.2 + 0.2);
        assert!(output[2] == 7.7);
    }

    #[test]
    fn test_layer_to_nn() {
        let mut layer = Layer::new(2, 2, 1, true);

        // untrained layer (default/empty)

        for x in -(OFFSET as i16)..(OFFSET as i16) {
            for y in (0i16..(OFFSET * 2) as i16).rev() {
                let layer_output: u8 = {
                    let outputs = layer.output(vec![
                        x as f32 / WIDTH as f32,
                        (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                    ]);

                    if outputs[0] > outputs[1] {
                        0
                    } else {
                        1
                    }
                };

                assert_eq!(
                    old_nn(
                        x as f32 / WIDTH as f32,
                        (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                        [[0f32; 2]; 2],
                        [0f32; 2]
                    ),
                    layer_output
                )
            }
        }

        // Test Case

        let layer_output: u8 = {
            let outputs = layer.output(vec![-0.033333335, 0.83492064]);

            if outputs[0] > outputs[1] {
                0
            } else {
                1
            }

        };
        assert_eq!(
            old_nn(-0.033333335, 0.83492064, [[0f32; 2]; 2], [0f32; 2]),
            layer_output
        );

        // Now testing trained layer

        layer.weights[0][0] = 0.31386864;
        layer.weights[0][1] = 0.5474453;
        layer.weights[1][0] = -0.6496351;
        layer.weights[1][1] = -0.021897793;

        layer.biases = [-0.44525546, 0.0].to_vec();

        for x in -(OFFSET as i16)..(OFFSET as i16) {
            for y in (0i16..(OFFSET * 2) as i16).rev() {
                let layer_output: u8 = {
                    let outputs = layer.output(vec![
                        x as f32 / WIDTH as f32,
                        (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                    ]);

                    if outputs[0] > outputs[1] {
                        0
                    } else {
                        1
                    }
                };

                assert_eq!(
                    old_nn(
                        x as f32 / WIDTH as f32,
                        (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                        [[0.31386864, -0.6496351], [0.5474453, -0.021897793]],
                        [-0.44525546, 0.0],
                    ),
                    layer_output
                )
            }
        }
    }

    #[test]
    fn test_nn_struct_to_old_nn() {
        let mut network = NeuralNetwork::new_network(&[2, 2]);
        assert_eq!(network.layers.len(), 1);
        assert_eq!(network.inputs, 2);

        for x in -(OFFSET as i16)..(OFFSET as i16) {
            for y in (0i16..(OFFSET * 2) as i16).rev() {
                println!(
                    "{:?}, {:?}",
                    x as f32 / WIDTH as f32,
                    (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32
                );
                assert_eq!(
                    old_nn(
                        x as f32 / WIDTH as f32,
                        (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
                        [[0f32; 2]; 2],
                        [0f32; 2]
                    ),
                    network.predict(vec![
                        x as f32 / WIDTH as f32,
                        (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32
                    ]) as u8
                )
            }
        }

        // todo: check with trained network
    }
}
