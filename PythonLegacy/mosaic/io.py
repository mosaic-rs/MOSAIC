from pathlib import Path
import pandas as pd
from .schemas import GENERAL_COLS, CENTRED_COLS, CURVE_COEFF_COLS, WIDE_CSV_STRUCTURE

def ensure_dir(p: Path) -> Path:
    p.mkdir(parents=True, exist_ok=True); return p

def initialize_csvs(filename: str, output_dir: Path, *, force: bool=False) -> Path | None:
    print(filename)
    output_dir = ensure_dir(output_dir)
    paths = {
        #"output":  output_dir / "output.csv", # commented out because we now just use the entire comprehensive csv
        #"centred": output_dir / "centred_landmarks.csv",
        #"curves":  output_dir / "curves.csv",
        "comprehensive": output_dir / f"{filename}.csv"
    }
    existing = [x for x in paths.values() if x.exists()]
    if existing and not force:
        print("files exist and will be overwritten:"); [print("  -", x.name) for x in existing]
        if input("Continue? (y/N): ").strip().lower() != "y": print("Aborting."); return
    for x in existing:
        try: x.unlink()
        except FileNotFoundError: pass

    #pd.DataFrame(columns=GENERAL_COLS).to_csv(paths["output"], index=False) # see above comment for why commented out
    #pd.DataFrame(columns=CENTRED_COLS).to_csv(paths["centred"], index=False)
    #pd.DataFrame(columns=CURVE_COEFF_COLS).to_csv(paths["curves"], index=False)
    pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(paths["comprehensive"], index=False)
    print("Created CSVs in", output_dir.resolve())
    return paths["comprehensive"]


def header_if_empty(path: Path, columns: list[str]) -> None:
    if not path.exists() or path.stat().st_size == 0:
        pd.DataFrame(columns=columns).to_csv(path, index=False)