import datetime

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
@click.option("-t", "--time", is_flag=True, help="Print elapsed time afterwards")
@click.argument("day", required=True)
def main(day: int, time: bool, data: str) -> None:
    runner_class = days.get_runner(day)

    start = datetime.datetime.now()

    part1, part2 = runner_class.run_both(data)

    if time:
        click.echo(f"Elapsed: {datetime.datetime.now() - start}", err=True)

    click.echo(part1)
    click.echo(part2)


if __name__ == "__main__":
    main()
