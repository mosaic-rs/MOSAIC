from pathlib import Path
import math
from typing import Tuple, List, Optional
import pandas as pd
from tqdm import tqdm

from mosaic.core_measurements.pose_correction.pose_correction import LandmarkCorrection
from mosaic.schemas import WIDE_CSV_STRUCTURE

def _unpack_xy(pt) -> Tuple[float, float]:
    if pt is None:
        return math.nan, math.nan
    try:
        return float(pt[0]), float(pt[1])
    except Exception:
        return math.nan, math.nan

def _get_pose_row(df: pd.DataFrame, row: int):
    def pick(col_a, col_b):
        if col_a in df.columns: return df.loc[row, col_a]
        if col_b in df.columns: return df.loc[row, col_b]
        return math.nan
    Rx = pick("pose_Rx", "Rx")
    Ry = pick("pose_Ry", "Ry")
    Rz = pick("pose_Rz", "Rz")
    return (
        float(Rx) if pd.notna(Rx) else math.nan,
        float(Ry) if pd.notna(Ry) else math.nan,
        float(Rz) if pd.notna(Rz) else math.nan,
    )

def run(input_file: str, output_file: Path, *, force: bool = False, start: int = 0, end: Optional[int] = None) -> None:
    out_csv = output_file
    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    df = pd.read_csv(input_file)
    total = len(df)
    if end is None or end > total:
        end = total
    if start < 0:
        start = 0
    if start >= end:
        print("Nothing to do: start >= end.")
        return

    updates: List[dict] = []

    for row in tqdm(range(start, end), desc="Pose correction", unit="frame"):
        try:
            Rx, Ry, Rz = _get_pose_row(df, row=row)
            P = LandmarkCorrection(df, row=row)
            pts = P.pose_correction(Rx, Ry, Rz)

            xs: List[float] = []
            ys: List[float] = []
            for i in range(20):
                x, y = _unpack_xy(pts[i] if i < len(pts) else None)
                xs.append(x); ys.append(y)

            rec = {"frame": int(row)+1}
            rec.update({f"x_{i}": xs[i - 48] for i in range(48, 68)})
            rec.update({f"y_{i}": ys[i - 48] for i in range(48, 68)})
            updates.append(rec)

        except Exception as e:
            print(f"Error on row {row}: {e}")

    if updates:
        wide = pd.read_csv(out_csv).set_index("frame")
        upd = pd.DataFrame(updates).set_index("frame")
        wide.update(upd)
        new_frames = upd.index.difference(wide.index)
        if len(new_frames) > 0 and not upd.loc[new_frames].empty:
            wide = pd.concat([wide, upd.loc[new_frames]], axis=0)
        wide = wide.reset_index()
        wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
        wide.sort_values("frame", inplace=True)
        wide.to_csv(out_csv, index=False)

    print(f"Pose-corrected XY → {out_csv.resolve()}")
