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

minutes = list(range(0, 60))
sleep_start = None
total_sleep = 0
guard_id = None

shift_start_idx = [i for i, evt in enumerate(events) if evt.type == EventType.BEGIN]


for evt in events:
    if evt.type == EventType.BEGIN:
        # Edge case where guard sleeps entire shift
        if sleep_start is not None and guard_id is not None:
            sleep_duration = 60 - sleep_start
            total_sleep += sleep_duration
            guard_sleep[guard_id] += total_sleep
            sleep_range = slice(sleep_start, 60)
            latest_row = guard_calendar[guard_id][-1]
            latest_row[sleep_range] = [State.ASLEEP] * sleep_duration

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
        sleep_start = None


def get_minute_freq(schedule):
    minute_freq = cl.Counter([i for row in schedule
                              for i, state in enumerate(row) if state == State.ASLEEP])
    return minute_freq


minute_freq = {guard_id: get_minute_freq(cal) for guard_id, cal in guard_calendar.items()}
minute_freq = {guard_id: minute_ctr for guard_id, minute_ctr in minute_freq.items() if minute_ctr}

sleepy_guard_id, sleepy_guard_minutes = max(minute_freq.items(),
                                            key=lambda g_id:
                                                minute_freq[g_id[0]].most_common(1)[0][1])
most_sleepy_minute = sleepy_guard_minutes.most_common(1)[0][0]

print('Answer =', sleepy_guard_id * most_sleepy_minute)
