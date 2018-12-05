import sys
import re
import enum
import collections as cl
from datetime import datetime


patterns = (re.compile(r'\[(.*?)\] Guard \#(\d+) begins shift'),
            re.compile(r'\[(.*?)\] falls asleep'),
            re.compile(r'\[(.*?)\] wakes up'))

Event = cl.namedtuple('Event', 'time minute type guard')


class EventType(enum.IntEnum):
    BEGIN = 0
    ASLEEP = 1
    AWAKE = 2


class State(enum.IntEnum):
    AWAKE = 0
    ASLEEP = 1


def parse_line(line):
    for event, pattern in zip(EventType, patterns):
        match = pattern.match(line)
        if match:
            date = datetime.strptime(match.group(1), '%Y-%m-%d %H:%M')
            guard_id = None if event != 0 else match.group(2)
            return Event(date, date.minute, event, guard_id)


events = list(sorted(map(parse_line, sys.stdin), key=lambda event: event.time))

assert events[0].type == EventType.BEGIN

guard_calendar = cl.defaultdict(list)  # dict of list of lists
guard_sleep = cl.defaultdict(int)

for evt in events:
    if evt.type == EventType.BEGIN:
        guard_id = int(evt.guard)
        total_sleep = 0
        guard_calendar[guard_id].append([State.AWAKE] * 60)
    elif evt.type == EventType.ASLEEP:
        sleep_start = evt.minute
    elif evt.type == EventType.AWAKE:
        sleep_duration = evt.minute - sleep_start
        total_sleep += sleep_duration
        guard_sleep[guard_id] += total_sleep
        sleep_range = slice(sleep_start, evt.minute)
        latest_row = guard_calendar[guard_id][-1]
        latest_row[sleep_range] = [State.ASLEEP] * sleep_duration


def get_minute_freq(schedule):
    minute_freq = cl.Counter([i for row in schedule
                              for i, state in enumerate(row) if state == State.ASLEEP])
    return minute_freq


sleepy_guard = max(guard_sleep, key=lambda g: guard_sleep[g])
sleepy_schedule = guard_calendar[sleepy_guard]


minute_freq = get_minute_freq(sleepy_schedule)
sleepy_minute, _ = minute_freq.most_common(1)[0]

print(f'Guard #{sleepy_guard} is most sleepy at minute {sleepy_minute}')
print('Answer is', sleepy_guard * sleepy_minute)
