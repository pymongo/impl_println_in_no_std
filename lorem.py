# Only support Linux
import datetime
import os
import json


def modify():
    file = open('lorem.md', 'r')
    flag = int(file.readline()) == 0
    file.close()
    with open('lorem.md', 'w+') as f:
        f.write('1' if flag else '0')


def commit():
    os.system('git commit -a -m update_lorem > /dev/null 2>&1')


def set_sys_time(year: int, month: int, day: int):
    """
    `date -s` command only support in Linux os
    to reset system datetime: use ntp server or
    date +"%d %b %Y %T %Z" -s "$(wget -qSO- --max-redirect=0 https://baidu.com 2>&1 | grep '^  Date:' | cut -d' ' -f 5-)"
    """
    os.system('date -s %04d%02d%02d' % (year, month, day))


def trick_commit(year: int, month: int, day: int):
    set_sys_time(year, month, day)
    modify()
    commit()


def daily_commit(start_date: datetime.date, end_date: datetime.date):
    for i in range((end_date - start_date).days + 1):
        cur_date = start_date + datetime.timedelta(days=i)
        trick_commit(cur_date.year, cur_date.month, cur_date.day)


if __name__ == '__main__':
    f = open('lorem_py_config.json')
    config = json.load(f)
    since = datetime.datetime.strptime(config["since"], '%Y-%m-%d')
    until = datetime.datetime.strptime(config["until"], '%Y-%m-%d')
    f.close()
    daily_commit(since.date(), until.date())

