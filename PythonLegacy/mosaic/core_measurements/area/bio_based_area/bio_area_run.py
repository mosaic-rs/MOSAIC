from pathlib import Path
import pandas as pd
from tqdm import tqdm

from mosaic.core_measurements.area.bio_based_area.bio_based_area import BioArea
from mosaic.schemas import WIDE_CSV_STRUCTURE


def _build_outer_cubics(df: pd.DataFrame, row: int):
    cubics = []
    for quad in ("UR", "UL", "LR", "LL"):
        P = [
            (df.loc[row, f"{quad}_Ax"], df.loc[row, f"{quad}_Ay"]),
            (df.loc[row, f"{quad}_Bx"], df.loc[row, f"{quad}_By"]),
            (df.loc[row, f"{quad}_Cx"], df.loc[row, f"{quad}_Cy"]),
            (df.loc[row, f"{quad}_Dx"], df.loc[row, f"{quad}_Dy"]),
        ]
        cubics.append(P)
    return cubics


def _build_inner_quadratics(df: pd.DataFrame, row: int):
    quads = []
    for quad in ("IUR", "IUL", "ILR", "ILL"):
        P = [
            (df.loc[row, f"{quad}_Ax"], df.loc[row, f"{quad}_Ay"]),
            (df.loc[row, f"{quad}_Bx"], df.loc[row, f"{quad}_By"]),
            (df.loc[row, f"{quad}_Cx"], df.loc[row, f"{quad}_Cy"]),
        ]
        quads.append(P)
    return quads


def run(input_file: str, output_file: Path, *, force: bool=False, start: int=0, end: int|None=None) -> None:

    out_csv = output_file
    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    df = pd.read_csv(input_file)
    BA = BioArea(df)

    total = len(df)
    if end is None or end > total:
        end = total
    if start < 0:
        start = 0
    if start >= end:
        print("Nothing to do: start >= end.")
        return

    updates_outer = []
    for row in tqdm(range(start, end), desc="BIO Outer Area", unit="frame"):
        try:
            cubics = _build_outer_cubics(df, row)
            O = BA.compute_outer_bio(row, cubics)
            timestamp = float(df.loc[row, "timestamp"]) if "timestamp" in df.columns else None

            updates_outer.append({
                "frame": int(row) + 1,
                "timestamp": timestamp,
                "BIO_O_Q1": O[1],
                "BIO_O_Q2": O[2],
                "BIO_O_Q3": O[3],
                "BIO_O_Q4": O[4],
                "BIO_O_total": O["total"],
            })

        except Exception as e:
            print(f"Error (BIO outer) on row {row}: {e}")

    updates_inner = []
    for row in tqdm(range(start, end), desc="BIO Inner Area", unit="frame"):
        try:
            quads = _build_inner_quadratics(df, row)
            I = BA.compute_inner_bio(row, quads)
            timestamp = float(df.loc[row, "timestamp"]) if "timestamp" in df.columns else None

            updates_inner.append({
                "frame": int(row) + 1,
                "timestamp": timestamp,
                "BIO_I_Q1": I[1],
                "BIO_I_Q2": I[2],
                "BIO_I_Q3": I[3],
                "BIO_I_Q4": I[4],
                "BIO_I_total": I["total"],
            })

        except Exception as e:
            print(f"Error (BIO inner) on row {row}: {e}")

    wide = pd.read_csv(out_csv).set_index("frame")

    def safe_update(wide, updates):
        if not updates:
            return wide
        upd = pd.DataFrame(updates).set_index("frame")

        wide = wide[~wide.index.duplicated(keep='last')]
        upd = upd[~upd.index.duplicated(keep='last')]

        wide.update(upd)

        new_frames = upd.index.difference(wide.index)
        new_data = upd.loc[new_frames]

        if not new_data.empty:
            if wide.empty:
                wide = new_data.copy()
            else:
                wide = pd.concat([wide, new_data], axis=0)

        return wide

    wide = safe_update(wide, updates_outer)
    wide = safe_update(wide, updates_inner)

    wide = wide.reset_index()
    wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
    wide.sort_values("frame", inplace=True)
    wide.to_csv(out_csv, index=False)