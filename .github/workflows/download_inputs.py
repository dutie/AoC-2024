import os
from datetime import datetime
import requests


def most_recent_day(year) -> int:
    """
    Get the year of the most recent advent
    """
    time = datetime.now()

    if time.year > year:
        return 25
    elif time.year == year and time.month == 12:
        return time.day
    return 0


if __name__ == "__main__":
    #: Location of the README file
    inputs_location = "data/inputs/"
    #: Advent of Code session cookie
    session_cookie = os.environ.get("SESSION_COOKIE", "")
    #: Year to query for
    year = int(os.environ.get("YEAR"))
    #: Advent of Code base URL, for testing
    advent_url = os.environ.get("ADVENT_URL", "https://adventofcode.com")
    #: Stars info endpoint
    input_endpoint = f"{advent_url}/{year}/day/*/input"

    for i in range(most_recent_day(year)):
        day = i+1
        res = requests.get(input_endpoint.replace("*", str(day)),
                           cookies={"session": session_cookie},
                           timeout=10)
        res.raise_for_status()
        with open(inputs_location + f"{day:02}.txt", "wb") as f:
            f.write(res.content)
