import os


def get_data(day: int, sub: int | None = None) -> str:
    basename = f"{day:02d}" if sub is None else f"{day:02d}.{sub}"
    sample = os.path.dirname(__file__) + f"/samples/{basename}.txt"
    with open(sample, mode="rt", encoding="utf-8") as f:
        return f.read()
