from pathlib import Path
import math
from typing import Tuple, List, Optional
import pandas as pd
from tqdm import tqdm

from mosaic.core_measurements.anchor.anchor_point import Centering
from mosaic.schemas import WIDE_CSV_STRUCTURE

"""
for some reason the centering code was done in the same file that calculates the anchor so look there for that
"""

def _unpack_xy(pt) -> Tuple[float, float]:
    if pt is None:
        return math.nan, math.nan
    try:
        return float(pt[0]), float(pt[1])
    except Exception:
        return math.nan, math.nan

def run(input_file: str, output_file: Path, *, force: bool = False, pose_corr: bool, start: int = 0, end: Optional[int] = None,) -> None:
    out_csv = output_file
    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    in_df = pd.read_csv(input_file)
    wide = pd.read_csv(out_csv)
    if "frame" not in wide.columns:
        raise FileNotFoundError("Wide CSV missing 'frame' column.")
    anchors_df = wide.set_index("frame")

    total = len(in_df)
    if end is None or end > total:
        end = total
    if start < 0:
        start = 0
    if start >= end:
        print("Nothing to do: start >= end.")
        return

    updates = []

    for row in tqdm(range(start, end), desc="Centering", unit="frame"):
        try:
            if row not in anchors_df.index:
                continue
            xa = float(anchors_df.at[row, "x_anchor"]) if pd.notna(anchors_df.at[row, "x_anchor"]) else math.nan
            ya = float(anchors_df.at[row, "y_anchor"]) if pd.notna(anchors_df.at[row, "y_anchor"]) else math.nan

            pts = Centering(in_df, row, True).center_with_anchor(xa, ya)
            xs: List[float] = []
            ys: List[float] = []
            for i in range(20):
                x, y = _unpack_xy(pts[i] if i < len(pts) else None)
                xs.append(x); ys.append(y)

            rec = {"frame": int(row), "pose_correction": pose_corr}
            rec.update({f"x_{i}": xs[i - 48] for i in range(48, 68)})
            rec.update({f"y_{i}": ys[i - 48] for i in range(48, 68)})
            updates.append(rec)

        except Exception as e:
            print(f"Error on row {row}: {e}")

    if updates:
        wide_idx = wide.set_index("frame")
        upd = pd.DataFrame(updates).set_index("frame")
        wide_idx.update(upd)
        new_frames = upd.index.difference(wide_idx.index)
        if len(new_frames) > 0 and not upd.loc[new_frames].empty:
            wide_idx = pd.concat([wide_idx, upd.loc[new_frames]], axis=0)
        wide = wide_idx.reset_index()
        wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
        wide.sort_values("frame", inplace=True)
        wide.to_csv(out_csv, index=False)

    print(f"Centered → {out_csv.resolve()}")
