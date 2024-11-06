use itertools::izip;
use serde::Deserialize;
use std::vec;

use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

use option_pricing::implied_vol::{ImpliedVol, Input as IvInput, Method};

fn output_type_impliedvol(_input_fields: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "output".into(),
        DataType::Struct(vec![
            Field::new("vol".into(), DataType::Float32),
            Field::new("iter".into(), DataType::Int32),
            Field::new("prec".into(), DataType::Float32),
        ]),
    ))
}

#[derive(Deserialize)]
struct ImpliedVolKwargs {
    iter: u32,
    prec: f32,
    method: String,
}

#[polars_expr(output_type_func=output_type_impliedvol)]
fn implied_vol(inputs: &[Series], kwargs: ImpliedVolKwargs) -> PolarsResult<Series> {
    let struct_ = inputs[0].struct_()?;
    let fields = struct_.fields_as_series();

    if fields.len() != 6
        || fields[0].name() != "price"
        || fields[0].dtype() != &DataType::Float32
        || fields[1].name() != "spot"
        || fields[1].dtype() != &DataType::Float32
        || fields[2].name() != "strike"
        || fields[2].dtype() != &DataType::Float32
        || fields[3].name() != "mat"
        || fields[3].dtype() != &DataType::Float32
        || fields[4].name() != "rate"
        || fields[4].dtype() != &DataType::Float32
        || fields[5].name() != "div"
        || fields[5].dtype() != &DataType::Float32
    {
        let expected_type = "Struct({'price': Float32, 'spot': Float32, 'strike': Float32, 'mat': Float32, 'rate': Float32, 'div': Float32})";
        let msg = "Expected Implied-Vol input is ".to_string() + expected_type;
        return Err(PolarsError::ComputeError(msg.into()));
    }

    let ca_price = fields[0].f32()?;
    let ca_spot = fields[1].f32()?;
    let ca_strike = fields[2].f32()?;
    let ca_mat = fields[3].f32()?;
    let ca_rate = fields[4].f32()?;
    let ca_div = fields[5].f32()?;

    if ca_price.null_count() != 0
        || ca_spot.null_count() != 0
        || ca_strike.null_count() != 0
        || ca_mat.null_count() != 0
        || ca_rate.null_count() != 0
        || ca_div.null_count() != 0
    {
        return Err(PolarsError::ComputeError(
            "Implied-Vol calculation does not support null values".into(),
        ));
    }

    let vec_price = ca_price.to_vec_null_aware().unwrap_left();
    let vec_spot = ca_spot.to_vec_null_aware().unwrap_left();
    let vec_strike = ca_strike.to_vec_null_aware().unwrap_left();
    let vec_mat = ca_mat.to_vec_null_aware().unwrap_left();
    let vec_rate = ca_rate.to_vec_null_aware().unwrap_left();
    let vec_div = ca_div.to_vec_null_aware().unwrap_left();

    let n = ca_spot.len();

    let mut vec_vol: Vec<f32> = vec![0.0; n];
    let mut vec_iter: Vec<u32> = vec![0; n];
    let mut vec_prec: Vec<f32> = vec![0.0; n];

    izip!(vec_price, vec_spot, vec_strike, vec_mat, vec_rate, vec_div)
        .enumerate()
        .for_each(|(i, (price, spot, strike, mat, rate, div))| {
            let iv_input = IvInput {
                price,
                spot,
                strike,
                mat,

                rate,
                div,
                iter: kwargs.iter,
                prec: kwargs.prec,
            };
            let method = match kwargs.method.as_str().to_lowercase().as_str() {
                // Add this line
                "newton" => Method::Newton,
                "halley" => Method::Halley,
                _ => Method::Newton,
            };
            let iv = ImpliedVol::new(iv_input.clone(), method);
            let iv_output = iv.output.unwrap();

            // println!("i={}", i);
            // println!("iv_input={:?}", iv_input);
            // println!("iv_output={:?}", iv_output);

            vec_vol[i] = iv_output.vol;
            vec_iter[i] = iv_output.iter;
            vec_prec[i] = iv_output.prec;
        });

    let series_out = vec![
        Series::new("vol_implied".into(), vec_vol),
        Series::from_vec("iter_implied".into(), vec_iter),
        Series::from_vec("prec_implied".into(), vec_prec),
    ];

    StructChunked::from_series("output".into(), &series_out).map(|ca| ca.into_series())
}
