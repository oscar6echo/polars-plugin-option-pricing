from pathlib import Path
from typing import Any, Dict, List, Optional, Union

import polars as pl
from polars.plugins import register_plugin_function

from ._typing import StrOrExpr

_PLUGIN_PATH = Path(__file__).parent


def pl_plugin(
    *,
    symbol: str,
    args: List[Union[pl.Series, pl.Expr]],
    kwargs: Optional[Dict[str, Any]] = None,
    is_elementwise: bool = False,
    returns_scalar: bool = False,
    changes_length: bool = False,
    cast_to_supertype: bool = False,
    pass_name_to_apply: bool = False,
) -> pl.Expr:
    return register_plugin_function(
        plugin_path=_PLUGIN_PATH,
        args=args,
        function_name=symbol,
        kwargs=kwargs,
        is_elementwise=is_elementwise,
        returns_scalar=returns_scalar,
        changes_length=changes_length,
        cast_to_supertype=cast_to_supertype,
        pass_name_to_apply=pass_name_to_apply,
    )


def str_to_expr(e: StrOrExpr) -> pl.Expr:
    """"""
    if isinstance(e, str):
        return pl.col(e)
    elif isinstance(e, pl.Expr):
        return e
    else:
        raise ValueError("Input must either be a string or a Polars expression.")
