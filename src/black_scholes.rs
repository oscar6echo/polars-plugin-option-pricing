use itertools::izip;
use std::vec;

use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

use option_pricing::black_scholes::{BlackScholes, Input as BsInput};

fn output_type_blackscholes(_input_fields: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "output".into(),
        DataType::Struct(vec![
            Field::new("is_call".into(), DataType::Boolean),
            Field::new("d1".into(), DataType::Float32),
            Field::new("d2".into(), DataType::Float32),
            Field::new("cdf_d1".into(), DataType::Float32),
            Field::new("cdf_d2".into(), DataType::Float32),
            Field::new("pdf_d1".into(), DataType::Float32),
            Field::new("pdf_d2".into(), DataType::Float32),
            Field::new("pv".into(), DataType::Float32),
            Field::new("pv_k".into(), DataType::Float32),
            Field::new("w".into(), DataType::Float32),
            Field::new("price".into(), DataType::Float32),
            Field::new("delta".into(), DataType::Float32),
            Field::new("gamma".into(), DataType::Float32),
            Field::new("vega".into(), DataType::Float32),
            Field::new("theta".into(), DataType::Float32),
            Field::new("rho".into(), DataType::Float32),
            Field::new("voma".into(), DataType::Float32),
            Field::new("payoff".into(), DataType::Float32),
            Field::new("pv_payoff".into(), DataType::Float32),
        ]),
    ))
}

#[polars_expr(output_type_func=output_type_blackscholes)]
fn black_scholes(inputs: &[Series]) -> PolarsResult<Series> {
    let struct_ = inputs[0].struct_()?;
    let fields = struct_.fields_as_series();

    if fields.len() != 7
        || fields[0].name() != "is_call"
        || fields[0].dtype() != &DataType::Boolean
        || fields[1].name() != "spot"
        || fields[1].dtype() != &DataType::Float32
        || fields[2].name() != "strike"
        || fields[2].dtype() != &DataType::Float32
        || fields[3].name() != "mat"
        || fields[3].dtype() != &DataType::Float32
        || fields[4].name() != "vol"
        || fields[4].dtype() != &DataType::Float32
        || fields[5].name() != "rate"
        || fields[5].dtype() != &DataType::Float32
        || fields[6].name() != "div"
        || fields[6].dtype() != &DataType::Float32
    {
        let expected_type = "Struct({'is_call': Boolean, 'spot': Float32, 'strike': Float32, 'mat': Float32, 'vol': Float32, 'rate': Float32, 'div': Float32})";
        let msg = "Expected Black-Scholes input is ".to_string() + expected_type;
        return Err(PolarsError::ComputeError(msg.into()));
    }

    let ca_is_call = fields[0].bool()?;
    let ca_spot = fields[1].f32()?;
    let ca_strike = fields[2].f32()?;
    let ca_mat = fields[3].f32()?;
    let ca_vol = fields[4].f32()?;
    let ca_rate = fields[5].f32()?;
    let ca_div = fields[6].f32()?;

    if ca_is_call.null_count() != 0
        || ca_spot.null_count() != 0
        || ca_strike.null_count() != 0
        || ca_mat.null_count() != 0
        || ca_vol.null_count() != 0
        || ca_rate.null_count() != 0
        || ca_div.null_count() != 0
    {
        return Err(PolarsError::ComputeError(
            "Black-Scholes calculation does not support null values".into(),
        ));
    }

    let vec_is_call: Vec<bool> = ca_is_call.into_no_null_iter().collect();
    let vec_spot = ca_spot.to_vec_null_aware().unwrap_left();
    let vec_strike = ca_strike.to_vec_null_aware().unwrap_left();
    let vec_mat = ca_mat.to_vec_null_aware().unwrap_left();
    let vec_vol = ca_vol.to_vec_null_aware().unwrap_left();
    let vec_rate = ca_rate.to_vec_null_aware().unwrap_left();
    let vec_div = ca_div.to_vec_null_aware().unwrap_left();

    let n = ca_spot.len();

    let mut vec_d1: Vec<f32> = vec![0.0; n];
    let mut vec_d2: Vec<f32> = vec![0.0; n];
    let mut vec_cdf_d1: Vec<f32> = vec![0.0; n];
    let mut vec_cdf_d2: Vec<f32> = vec![0.0; n];
    let mut vec_pdf_d1: Vec<f32> = vec![0.0; n];
    let mut vec_pdf_d2: Vec<f32> = vec![0.0; n];
    let mut vec_pv: Vec<f32> = vec![0.0; n];
    let mut vec_pv_k: Vec<f32> = vec![0.0; n];
    let mut vec_w: Vec<f32> = vec![0.0; n];
    let mut vec_price: Vec<f32> = vec![0.0; n];
    let mut vec_delta: Vec<f32> = vec![0.0; n];
    let mut vec_gamma: Vec<f32> = vec![0.0; n];
    let mut vec_vega: Vec<f32> = vec![0.0; n];
    let mut vec_theta: Vec<f32> = vec![0.0; n];
    let mut vec_rho: Vec<f32> = vec![0.0; n];
    let mut vec_voma: Vec<f32> = vec![0.0; n];
    let mut vec_payoff: Vec<f32> = vec![0.0; n];
    let mut vec_pv_payoff: Vec<f32> = vec![0.0; n];

    izip!(
        vec_is_call.clone(),
        vec_spot,
        vec_strike,
        vec_mat,
        vec_vol,
        vec_rate,
        vec_div
    )
    .enumerate()
    .for_each(|(i, (is_call, spot, strike, mat, vol, rate, div))| {
        let bs_input = BsInput {
            is_call,
            spot,
            strike,
            mat,
            vol,
            rate,
            div,
        };
        let bs = BlackScholes::new(bs_input);
        let bs_output = bs.output.unwrap();

        vec_d1[i] = bs_output.d1;
        vec_d2[i] = bs_output.d2;
        vec_cdf_d1[i] = bs_output.cdf_d1;
        vec_cdf_d2[i] = bs_output.cdf_d2;
        vec_pdf_d1[i] = bs_output.pdf_d1;
        vec_pdf_d2[i] = bs_output.pdf_d2;
        vec_pv[i] = bs_output.pv;
        vec_pv_k[i] = bs_output.pv_k;
        vec_w[i] = bs_output.w;
        vec_price[i] = bs_output.price;
        vec_delta[i] = bs_output.delta;
        vec_gamma[i] = bs_output.gamma;
        vec_vega[i] = bs_output.vega;
        vec_theta[i] = bs_output.theta;
        vec_rho[i] = bs_output.rho;
        vec_voma[i] = bs_output.voma;
        vec_payoff[i] = bs_output.payoff;
        vec_pv_payoff[i] = bs_output.pv_payoff;
    });

    let series_out = vec![
        Series::new("is_call".into(), vec_is_call),
        Series::from_vec("d1".into(), vec_d1),
        Series::from_vec("d2".into(), vec_d2),
        Series::from_vec("cdf_d1".into(), vec_cdf_d1),
        Series::from_vec("cdf_d2".into(), vec_cdf_d2),
        Series::from_vec("pdf_d1".into(), vec_pdf_d1),
        Series::from_vec("pdf_d2".into(), vec_pdf_d2),
        Series::from_vec("pv".into(), vec_pv),
        Series::from_vec("pv_k".into(), vec_pv_k),
        Series::from_vec("w".into(), vec_w),
        Series::from_vec("price".into(), vec_price),
        Series::from_vec("delta".into(), vec_delta),
        Series::from_vec("gamma".into(), vec_gamma),
        Series::from_vec("vega".into(), vec_vega),
        Series::from_vec("theta".into(), vec_theta),
        Series::from_vec("rho".into(), vec_rho),
        Series::from_vec("voma".into(), vec_voma),
        Series::from_vec("payoff".into(), vec_payoff),
        Series::from_vec("pv_payoff".into(), vec_pv_payoff),
    ];

    StructChunked::from_series("output".into(), &series_out).map(|ca| ca.into_series())

    // Ok(Field::new(name, DataType::Struct(fields)))
    // let se: Series = (0..n).map(|i| i as i32).collect();
    // Ok(se)
}
