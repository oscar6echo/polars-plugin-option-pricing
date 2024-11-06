from typing import Literal, TypeAlias, Union

import polars as pl

StrOrExpr: TypeAlias = Union[str, pl.Expr]

ImpliedVolMethod: TypeAlias = Literal["Newton", "Halley"]
