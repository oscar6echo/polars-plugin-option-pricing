import polars as pl

# from ._rust import __version__ as _version_rust
from ._typing import ImpliedVolMethod, StrOrExpr
from ._util import pl_plugin, str_to_expr

# __version_rust__ = str(_version_rust)
__version__ = "0.1.0"


def black_scholes(
    is_call: str | pl.Expr,
    spot: str | pl.Expr,
    strike: str | pl.Expr,
    vol: str | pl.Expr,
    mat: str | pl.Expr,
    rate: str | pl.Expr,
    div: str | pl.Expr,
) -> pl.Expr:
    _is_call = str_to_expr(is_call)
    _spot = str_to_expr(spot)
    _strike = str_to_expr(strike)
    _vol = str_to_expr(vol)
    _mat = str_to_expr(mat)
    _rate = str_to_expr(rate)
    _div = str_to_expr(div)

    _expr = pl.struct(_is_call, _spot, _strike, _vol, _mat, _rate, _div)

    return pl_plugin(
        symbol="black_scholes",
        args=[_expr],
        is_elementwise=True,
    )


def black_scholes_2(expr: StrOrExpr) -> pl.Expr:
    _expr = str_to_expr(expr)

    return pl_plugin(
        symbol="black_scholes",
        args=[_expr],
        is_elementwise=True,
    )


def implied_vol(
    price: str | pl.Expr,
    spot: str | pl.Expr,
    strike: str | pl.Expr,
    mat: str | pl.Expr,
    rate: str | pl.Expr,
    div: str | pl.Expr,
    iter: int = 10,
    prec: float = 1e-6,
    method: ImpliedVolMethod = "Newton",
) -> pl.Expr:
    _price = str_to_expr(price)
    _spot = str_to_expr(spot)
    _strike = str_to_expr(strike)
    _mat = str_to_expr(mat)
    _rate = str_to_expr(rate)
    _div = str_to_expr(div)

    _expr = pl.struct(_price, _spot, _strike, _mat, _rate, _div)

    return pl_plugin(
        symbol="implied_vol",
        args=[_expr],
        kwargs={"iter": iter, "prec": prec, "method": method},
        is_elementwise=True,
    )
