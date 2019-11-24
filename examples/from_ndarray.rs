use vega_lite_3::*;

use ndarray::{arr2, Array, Array2};
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input data: a random ndarray
    //  let values: Array2<f64> = Array::random((100, 2), StandardNormal);
    let values = Array::from_shape_vec((2, 2), vec![1, 2, 3, 4])?;
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Random points")
        .data(values)
        .mark(Mark::Point)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("data.0")
                    .def_type(StandardType::Quantitative)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("data.1")
                    .def_type(StandardType::Quantitative)
                    .build()?)
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
