from pathlib import Path
from typing import Optional, List, Dict, Tuple
import numpy as np
import pandas as pd
from tqdm import tqdm

from mosaic.core_measurements.curves.curve_fitting import CurveFitting
from mosaic.schemas import WIDE_CSV_STRUCTURE, cubic_curve_col_func, quadratic_curve_col_func

def _pad_points(pts: np.ndarray, target: int) -> List[Tuple[Optional[float], Optional[float]]]:
    out: List[Tuple[Optional[float], Optional[float]]] = []
    for i in range(target):
        if i < len(pts):
            out.append((float(pts[i][0]), float(pts[i][1])))
        else:
            out.append((np.nan, np.nan))
    return out

def _prefix_from_region(region: str, deg: int) -> Optional[str]:
    u = str(region).upper()
    base = None
    if "UR" in u: base = "UR"
    elif "UL" in u: base = "UL"
    elif "LR" in u: base = "LR"
    elif "LL" in u: base = "LL"
    else:
        is_upper = ("UPPER" in u) or ("TOP" in u)
        is_lower = ("LOWER" in u) or ("BOTTOM" in u)
        is_right = ("RIGHT" in u)
        is_left  = ("LEFT"  in u)
        if is_upper and is_right: base = "UR"
        elif is_upper and is_left: base = "UL"
        elif is_lower and is_right: base = "LR"
        elif is_lower and is_left: base = "LL"
    if base is None:
        return None
    return base if deg == 3 else "I" + base

def export_curve_coefficients(
    centred_csv: Path | str,
    output_file: Path,
    *,
    force: bool = False,
    start: int = 0,
    end: Optional[int] = None,
) -> None:
    out_csv = output_file
    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    df = pd.read_csv(centred_csv)
    total = len(df)
    if end is None or end > total:
        end = total
    if start < 0:
        start = 0
    if start >= end:
        print("Nothing to do: start >= end.")
        return

    allowed_cubic = list(cubic_curve_col_func())
    allowed_quad  = list(quadratic_curve_col_func())

    updates: List[Dict[str, object]] = []

    for row in tqdm(range(start, end), desc="Curves", unit="frame"):
        try:
            cf = CurveFitting(df, row=row, t=0.0)
            rec: Dict[str, object] = {"frame": int(row)+1}

            for region, (_, deg) in cf._REGIONS.items():
                pref = _prefix_from_region(region, deg)
                if pref is None:
                    continue

                cx, cy, pts = cf._coeffs_with_pts(region)

                if deg == 3:
                    Ax, Bx, Cx, Dx = float(cx[0]), float(cx[1]), float(cx[2]), float(cx[3])
                    Ay, By, Cy, Dy = float(cy[0]), float(cy[1]), float(cy[2]), float(cy[3])
                    P = _pad_points(pts, target=4)
                    kv = {
                        f"{pref}_Ax": Ax, f"{pref}_Bx": Bx, f"{pref}_Cx": Cx, f"{pref}_Dx": Dx,
                        f"{pref}_Ay": Ay, f"{pref}_By": By, f"{pref}_Cy": Cy, f"{pref}_Dy": Dy,
                        f"{pref}_X0": P[0][0], f"{pref}_Y0": P[0][1],
                        f"{pref}_X1": P[1][0], f"{pref}_Y1": P[1][1],
                        f"{pref}_X2": P[2][0], f"{pref}_Y2": P[2][1],
                        f"{pref}_X3": P[3][0], f"{pref}_Y3": P[3][1],
                    }
                    for k, v in kv.items():
                        if k in allowed_cubic:
                            rec[k] = v

                elif deg == 2:
                    Ax, Bx, Cx = float(cx[0]), float(cx[1]), float(cx[2])
                    Ay, By, Cy = float(cy[0]), float(cy[1]), float(cy[2])
                    P = _pad_points(pts, target=3)
                    kv = {
                        f"{pref}_Ax": Ax, f"{pref}_Bx": Bx, f"{pref}_Cx": Cx, f"{pref}_Dx": np.nan,
                        f"{pref}_Ay": Ay, f"{pref}_By": By, f"{pref}_Cy": Cy, f"{pref}_Dy": np.nan,
                        f"{pref}_X0": P[0][0], f"{pref}_Y0": P[0][1],
                        f"{pref}_X1": P[1][0], f"{pref}_Y1": P[1][1],
                        f"{pref}_X2": P[2][0], f"{pref}_Y2": P[2][1],
                    }
                    for k, v in kv.items():
                        if k in allowed_quad:
                            rec[k] = v

            updates.append(rec)

        except Exception as e:
            print(f"Error on row {row}: {e}")

    if updates:
        wide = pd.read_csv(out_csv)
        if "frame" not in wide.columns:
            raise FileNotFoundError("Wide CSV missing 'frame' column.")
        wide_idx = wide.set_index("frame")
        upd = pd.DataFrame(updates).set_index("frame")

        write_cols = [c for c in upd.columns if c in wide_idx.columns]
        if write_cols:
            all_idx = wide_idx.index.union(upd.index)
            wide_idx = wide_idx.reindex(all_idx)
            wide_idx.loc[upd.index, write_cols] = upd[write_cols].to_numpy()

            wide = wide_idx.reset_index()
            wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
            wide.sort_values("frame", inplace=True)
            wide.to_csv(out_csv, index=False)

    print(f"Curves → {out_csv.resolve()}")
