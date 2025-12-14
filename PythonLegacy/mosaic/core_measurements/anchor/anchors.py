from pathlib import Path
import pandas as pd
from tqdm import tqdm
import math
from mosaic.core_measurements.anchor.anchor_point import Anchor
from mosaic.schemas import WIDE_CSV_STRUCTURE

"""

Because it is the first data to be written to the wide csv, it also appends the timestamp of the frame as well

"""



def _get_pose_row(df: pd.DataFrame, row: int):
    """Return (Rx,Ry,Rz) for this row. Accepts either pose_R* or bare R* column names."""
    def pick(col_a, col_b):
        if col_a in df.columns: return df.loc[row, col_a]
        if col_b in df.columns: return df.loc[row, col_b]
        return math.nan
    Rx = pick("pose_Rx", "Rx")
    Ry = pick("pose_Ry", "Ry")
    Rz = pick("pose_Rz", "Rz")
    return float(Rx) if pd.notna(Rx) else math.nan, \
           float(Ry) if pd.notna(Ry) else math.nan, \
           float(Rz) if pd.notna(Rz) else math.nan

def _get_timestamp(df: pd.DataFrame, row: int):
    """Gets the timestamp for the frame"""
    if "timestamp" in df.columns: return float(df.loc[row, "timestamp"])


def run(input_file: str, output_file: Path, *, pose_corr: bool, force: bool=False, start: int=0, end: int|None=None) -> None:
    out_csv = output_file  # WIDE CSV PATH

    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    df = pd.read_csv(input_file)
    A = Anchor(df, filtering=True)

    total = len(df)
    if end is None or end > total:
        end = total
    if start < 0:
        start = 0
    if start >= end:
        print("Nothing to do: start >= end.")
        return

    # Collect updates here (one row per frame with only the columns we want to fill)
    updates = []

    for row in tqdm(range(start, end), desc="Anchors", unit="frame"):
        try:
            d = A.get_all_anchors(row)
            Rx, Ry, Rz = _get_pose_row(df, row)
            timestamp = _get_timestamp(df, row)

            updates.append({
                "frame": int(row)+1,
                "timestamp": timestamp,
                "pose_correction": pose_corr,
                "pose_Rx": float(Rx), "pose_Ry": float(Ry), "pose_Rz": float(Rz),
                "x_anchor": float(d["x_anchor"][0]),
                "y_anchor": float(d["y_anchor"][0]),
                # ONLY anchor uncertainties (ignore per-landmark uncertainties)
                "x_unc": float(d["x_anchor"][1]),
                "y_unc": float(d["y_anchor"][1]),
            })

        except Exception as e:
            print(f"Error on row {row}: {e}")

    if updates:
        wide = pd.read_csv(out_csv).set_index("frame")
        upd  = pd.DataFrame(updates).set_index("frame")

        wide.update(upd)
        new_frames = upd.index.difference(wide.index)
        new_data = upd.loc[new_frames]

        if not new_data.empty:
            if wide.empty:
                wide = new_data.copy()
            else:
                wide = pd.concat([wide, new_data], axis=0)

        wide = wide.reset_index()
        wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
        wide.sort_values("frame", inplace=True)
        wide.to_csv(out_csv, index=False)

    print("Anchors →", out_csv.resolve())