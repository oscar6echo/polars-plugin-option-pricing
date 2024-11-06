from timeit import default_timer as timer

import numpy as np
import polars as pl

import polars_plugin_option_pricing as m

print(f"polars_plugin_option_pricing version: {m.__version__}")


def lap(t0, tag=""):
    t1 = timer()
    print(f"LAP {tag}: {t1 - t0:.3f} s")


def _range(e):
    start = e["start"]
    end = e["end"]
    n_step = e["n_step"]
    return np.linspace(start, end, n_step)


print("START")
print()


##########################
t0 = timer()

spots = {"start": 98, "end": 102, "n_step": 3}
strikes = {"start": 100, "end": 100, "n_step": 1}
mats = {"start": 1, "end": 10, "n_step": 1}
vols = {"start": 0.1, "end": 0.5, "n_step": 1}
rates = {"start": 0.00, "end": 0.04, "n_step": 1}
divs = {"start": 0.00, "end": 0.07, "n_step": 1}

spots = {"start": 80, "end": 120, "n_step": 25}
strikes = {"start": 80, "end": 120, "n_step": 100}
mats = {"start": 1, "end": 10, "n_step": 10}
vols = {"start": 0.1, "end": 0.5, "n_step": 5}
rates = {"start": 0.00, "end": 0.04, "n_step": 5}
divs = {"start": 0.00, "end": 0.07, "n_step": 8}


df_is_call = pl.DataFrame(data=[True, False], schema=["is_call"])
df_spot = pl.DataFrame(data=_range(spots), schema={"spot": pl.Float32})
df_strike = pl.DataFrame(data=_range(strikes), schema={"strike": pl.Float32})
df_mat = pl.DataFrame(data=_range(mats), schema={"mat": pl.Float32})
df_vol = pl.DataFrame(data=_range(vols), schema={"vol": pl.Float32})
df_rate = pl.DataFrame(data=_range(rates), schema={"rate": pl.Float32})
df_div = pl.DataFrame(data=_range(divs), schema={"div": pl.Float32})

lap(t0, "init")

##########################
t1 = timer()

df = (
    df_is_call.join(df_spot, how="cross")
    .join(df_strike, how="cross")
    .join(df_mat, how="cross")
    .join(df_vol, how="cross")
    .join(df_rate, how="cross")
    .join(df_div, how="cross")
)
print(df)
print(f"{df.shape[0]:,}")

lap(t1, "cross")


shape_input = "struct"
shape_input = "cols"

print("-" * 50)
print(f"SHAPE_INPUT: {shape_input}")
print("-" * 50)


################################
################################
################################
if shape_input == "struct":
    ##########################
    t2 = timer()

    df = df.with_columns(
        input=pl.struct(["is_call", "spot", "strike", "mat", "vol", "rate", "div"])
    )
    print(df)

    lap(t2, "struct")

    ##########################
    t3 = timer()

    df = df.with_columns(
        output=m.black_scholes_2("input"),
    )
    print(df)

    lap(t3, "bs")

    #########################
    t4 = timer()

    df = df.drop(["is_call", "input"]).unnest("output")
    print(df)

    lap(t4, "unnest")


################################
################################
################################
if shape_input == "cols":
    ##########################
    t3 = timer()

    df = df.with_columns(
        output=m.black_scholes(
            "is_call", "spot", "strike", "mat", "vol", "rate", "div"
        ),
    )
    print(df)

    lap(t3, "bs")

    #########################
    t4 = timer()

    df = df.drop(["is_call"]).unnest("output")
    print(df)

    lap(t4, "unnest")


##########################
print("\nEND")
