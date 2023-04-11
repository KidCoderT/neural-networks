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
            weights: vec![vec![0f32; outputs]; inputs],
            biases: vec![0f32; outputs],
        }
    }

    pub fn output(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut weighted_output = vec![0f32; self.outputs];

        for node_idx in 0..self.outputs {
            weighted_output[node_idx] += (0..self.inputs)
                .fold(self.biases[node_idx], |acc, input_idx| {
                    inputs[input_idx] * self.weights[input_idx][node_idx] + acc
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

        for idx in (1..layer_sizes.len()) {
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

    fn output(&self, inputs: Vec<f32>) -> Vec<f32> {
        assert_eq!(inputs.len(), self.inputs);
        let mut input: Vec<f32> = inputs.clone();

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

    fn old_nn(x: f32, y: f32, weights: [[f32; 2]; 2], biases: [f32; 2]) -> u8 {
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
    use super::Layer;
    // use super::NeuralNetwork;

    #[test]
    fn test_layer_output() {
        let mut layer = Layer::new(2, 3, 1, true);

        // setting the biases
        layer.biases[0] = 0.9;
        layer.biases[1] = 0.2;
        layer.biases[2] = 0.5;

        // setting the weights
        layer.weights[0][0] = 1.;
        layer.weights[1][0] = 1.5;

        layer.weights[0][1] = 3.;
        layer.weights[1][1] = 0.2;

        layer.weights[0][2] = 2.1;
        layer.weights[1][2] = 3.;

        let output = layer.output(vec![2., 1.]);

        assert_eq!(output.len(), 3);
        assert!(output[0] == 4.4);
        assert!(output[1] == 2f32 * 3. + 1f32 * 0.2 + 0.2);
        assert!(output[2] == 7.7);
    }
}
