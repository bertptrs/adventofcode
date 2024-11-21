import datetime
import time

import click

from . import days


@click.command()
@click.option(
    "-i",
    "--input",
    "data",
    type=click.File(mode="rt", encoding="utf8"),
    default="-",
    help="Problem input file",
)
@click.option("-t", "--time", "timing", is_flag=True, help="Print elapsed time afterwards")
@click.argument("day", required=True)
def main(day: int, timing: bool, data: str) -> None:
    runner_class = days.get_runner(day)

    start = time.perf_counter_ns()

    part1, part2 = runner_class.run_both(data)

    if timing:
        elapsed = time.perf_counter_ns() - start
        delta = datetime.timedelta(microseconds=elapsed / 1000)
        click.echo(f"Elapsed: {delta}", err=True)

    click.echo(part1)
    click.echo(part2)


if __name__ == "__main__":
    main()
